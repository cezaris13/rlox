use crate::token::LiteralValue::*;
use crate::token::TokenType::*;
use crate::token::{LiteralValue, Token, TokenType};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_string(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(self: &mut Self) -> Result<Vec<Token>, String> {
        let mut errors = vec![];

        while !self.is_at_end() {
            self.start = self.current;
            match self.scan_token() {
                Ok(_) => (),
                Err(msg) => errors.push(msg),
            }
        }

        self.tokens
            .push(Token::new(EOF, "".to_string(), None, self.line));

        if errors.len() > 0 {
            let mut joined = String::new();
            let _ = errors
                .iter()
                .map(|msg| {
                    joined.push_str(&msg);
                    joined.push_str("\n");
                })
                .collect::<Vec<_>>();
            return Err(joined);
        }

        Ok(self.tokens.clone()) // temp fix
    }

    fn scan_token(self: &mut Self) -> Result<(), String> {
        let symbol = self.advance();

        match symbol {
            '(' => self.add_token(LEFT_PAREN),
            ')' => self.add_token(RIGHT_PAREN),
            '{' => self.add_token(LEFT_BRACE),
            '}' => self.add_token(RIGHT_BRACE),
            ',' => self.add_token(COMMA),
            '.' => self.add_token(DOT),
            '-' => self.add_token(MINUS),
            '+' => self.add_token(PLUS),
            ';' => self.add_token(SEMICOLON),
            '*' => self.add_token(STAR),
            '!' => {
                if self.match_character('=') {
                    self.add_token(BANG_EQUAL)
                } else {
                    self.add_token(BANG)
                }
            }
            '=' => {
                if self.match_character('=') {
                    self.add_token(EQUAL_EQUAL)
                } else {
                    self.add_token(EQUAL)
                }
            }
            '<' => {
                if self.match_character('=') {
                    self.add_token(LESS_EQUAL)
                } else {
                    self.add_token(LESS)
                }
            }
            '>' => {
                if self.match_character('=') {
                    self.add_token(GREATER_EQUAL)
                } else {
                    self.add_token(GREATER)
                }
            }
            '/' => {
                if self.match_character('/') {
                    // A comment goes until the end of the line.
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(SLASH);
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            '"' => self.string()?,
            _ => {
                if self.is_digit(symbol) {
                    self.number()?;
                } else if self.is_alpha(symbol) {
                    self.identifier();
                } else {
                    return Err(format!(
                        "Unexpected character {0} at line {1}",
                        symbol, self.line
                    ));
                }
            }
        }
        Ok(())
    }

    fn add_token(self: &mut Self, token_type: TokenType) {
        self.add_token_lit(token_type, None);
    }

    fn add_token_lit(self: &mut Self, token_type: TokenType, literal: Option<LiteralValue>) {
        let mut text: String = String::new();
        let bytes = self.source.as_bytes();

        for i in self.start..self.current {
            text.push(bytes[i] as char);
        }

        self.tokens
            .push(Token::new(token_type, text, literal, self.line));
    }

    // region parser function

    fn string(self: &mut Self) -> Result<(), String> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }

            self.advance();
        }

        if self.is_at_end() {
            return Err(format!("Unterminated string at line {0}", self.line));
        }

        self.advance();

        // rust ranges are inclusive
        let value = self.source.as_bytes()[self.start + 1..self.current - 1]
            .iter()
            .map(|byte| *byte as char)
            .collect::<String>();

        self.add_token_lit(STRING, Some(StringValue(value)));

        Ok(())
    }

    fn number(self: &mut Self) -> Result<(), String> {
        let mut is_fraction = false;

        while self.is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            is_fraction = true;
            self.advance();

            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        let string_literal = self.source.as_bytes()[self.start..self.current]
            .iter()
            .map(|bytes| *bytes as char)
            .collect::<String>();

        println!("{}", string_literal);

        if is_fraction {
            match string_literal.parse::<f64>() {
                Ok(value) => self.add_token_lit(NUMBER, Some(FValue(value))),
                _ => return Err(format!("Failed to parse the float at line: {0}", self.line)),
            }
        } else {
            match string_literal.parse::<i64>() {
                Ok(value) => self.add_token_lit(NUMBER, Some(IntValue(value))),
                _ => return Err(format!("Failed to parse the int at line: {0}", self.line)),
            }
        }

        Ok(())
    }

    fn identifier(self: &mut Self) {
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }

        self.add_token(IDENTIFIER);
    }

    // endregion

    // region character manipulation

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len() as usize
    }

    fn advance(self: &mut Self) -> char {
        let symbol = self.source.as_bytes()[self.current];
        self.current += 1;
        symbol as char
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.as_bytes()[self.current] as char
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }

        self.source.as_bytes()[self.current + 1] as char
    }

    fn match_character(self: &mut Self, character: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source.as_bytes()[self.current] as char != character {
            return false;
        }

        self.current += 1;
        true
    }

    // endregion

    // region helper functions

    fn is_digit(&self, symbol: char) -> bool {
        symbol >= '0' && symbol <= '9'
    }

    fn is_alpha(&self, character: char) -> bool {
        (character >= 'a' && character <= 'z')
            || (character >= 'A' && character <= 'Z')
            || character == '_'
    }

    fn is_alpha_numeric(&self, character: char) -> bool {
        self.is_alpha(character) || self.is_digit(character)
    }

    // endregion
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handler_one_char_tokens() {
        let source = "((  ))";

        let mut scanner = Scanner::new(source);

        let _ = scanner.scan_tokens();

        assert_eq!(scanner.tokens.len(), 5); // due to EOF token
        assert_eq!(scanner.tokens[0].token_type, LEFT_PAREN);
        assert_eq!(scanner.tokens[1].token_type, LEFT_PAREN);
        assert_eq!(scanner.tokens[2].token_type, RIGHT_PAREN);
        assert_eq!(scanner.tokens[3].token_type, RIGHT_PAREN);
        assert_eq!(scanner.tokens[4].token_type, EOF);
    }

    #[test]
    fn handler_two_char_tokens() {
        let source = "! != == >=";

        let mut scanner = Scanner::new(source);

        let _ = scanner.scan_tokens();

        assert_eq!(scanner.tokens.len(), 5); // due to EOF token
        assert_eq!(scanner.tokens[0].token_type, BANG);
        assert_eq!(scanner.tokens[1].token_type, BANG_EQUAL);
        assert_eq!(scanner.tokens[2].token_type, EQUAL_EQUAL);
        assert_eq!(scanner.tokens[3].token_type, GREATER_EQUAL);
        assert_eq!(scanner.tokens[4].token_type, EOF);
    }

    #[test]
    fn handler_string_literal() {
        let source = "\"ABC\"";

        let mut scanner = Scanner::new(source);

        let result = scanner.scan_tokens();

        assert!(result.is_ok());
        assert_eq!(scanner.tokens.len(), 2); // due to EOF token
        assert_eq!(scanner.tokens[0].token_type, STRING);
        match scanner.tokens[0].literal.as_ref().unwrap() {
            StringValue(val) => assert_eq!(val, "ABC"),
            _ => panic!("Incorrect literal"),
        }

        assert_eq!(scanner.tokens[1].token_type, EOF);
    }

    #[test]
    fn handler_string_literal_not_closed_returns_error() {
        let source = "\"AB";

        let mut scanner = Scanner::new(source);

        let result = scanner.scan_tokens();

        assert!(result.is_err());
        assert_eq!(
            result.err(),
            Some("Unterminated string at line 1\n".to_string())
        );
    }

    #[test]
    fn handler_numeral_trailing_dot_returns_int() {
        let source = "123.";

        let mut scanner = Scanner::new(source);

        let result = scanner.scan_tokens();

        assert!(result.is_ok());
        assert_eq!(scanner.tokens[0].token_type, NUMBER);
        match scanner.tokens[0].literal.as_ref().unwrap() {
            IntValue(val) => assert_eq!(*val, 123),
            _ => panic!("Incorrect literal"),
        }
    }

    #[test]
    fn handler_float_numeral_returns_float_token() {
        let source = "123.15";

        let mut scanner = Scanner::new(source);

        let result = scanner.scan_tokens();

        assert!(result.is_ok());
        assert_eq!(scanner.tokens.len(), 2);
        assert_eq!(scanner.tokens[0].token_type, NUMBER);
        match scanner.tokens[0].literal.as_ref().unwrap() {
            FValue(val) => assert_eq!(*val, 123.15),
            _ => panic!("Incorrect literal"),
        }

        assert_eq!(scanner.tokens[1].token_type, EOF);
    }

    #[test]
    fn handler_int_numeral_returns_int_token() {
        let source = "123";

        let mut scanner = Scanner::new(source);

        let result = scanner.scan_tokens();

        assert!(result.is_ok());
        assert_eq!(scanner.tokens.len(), 2);
        assert_eq!(scanner.tokens[0].token_type, NUMBER);
        match scanner.tokens[0].literal.as_ref().unwrap() {
            IntValue(val) => assert_eq!(*val, 123),
            _ => panic!("Incorrect literal"),
        }

        assert_eq!(scanner.tokens[1].token_type, EOF);
    }
}
