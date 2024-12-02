use crate::token::LiteralValue::*;
use crate::token::TokenType::*;
use crate::token::{LiteralValue, Token, TokenType};

use std::collections::HashMap;
use std::string::String;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    keywords: HashMap<String, TokenType>,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        let mut keywords: HashMap<String, TokenType> = HashMap::new();

        keywords.insert("and".to_string(), And);
        keywords.insert("class".to_string(), Class);
        keywords.insert("else".to_string(), Else);
        keywords.insert("false".to_string(), False);
        keywords.insert("for".to_string(), For);
        keywords.insert("fun".to_string(), Fun);
        keywords.insert("if".to_string(), If);
        keywords.insert("nil".to_string(), Nil);
        keywords.insert("or".to_string(), Or);
        keywords.insert("print".to_string(), Print);
        keywords.insert("return".to_string(), Return);
        keywords.insert("super".to_string(), Super);
        keywords.insert("this".to_string(), This);
        keywords.insert("true".to_string(), True);
        keywords.insert("var".to_string(), Var);
        keywords.insert("while".to_string(), While);

        Self {
            source: source.to_string(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
            keywords: keywords,
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
            .push(Token::new(Eof, "".to_string(), None, self.line));

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
            '(' => self.add_token(LeftParen),
            ')' => self.add_token(RightParen),
            '{' => self.add_token(LeftBrace),
            '}' => self.add_token(RightBrace),
            ',' => self.add_token(Comma),
            '.' => self.add_token(Dot),
            '-' => self.add_token(Minus),
            '+' => self.add_token(Plus),
            ';' => self.add_token(Semicolon),
            '*' => {
                if self.match_character('/') {
                    return Err(format!(
                        "Extra multiline ending comment at line {0}",
                        self.line
                    ));
                }
                self.add_token(Star)
            }
            '!' => {
                if self.match_character('=') {
                    self.add_token(BangEqual)
                } else {
                    self.add_token(Bang)
                }
            }
            '=' => {
                if self.match_character('=') {
                    self.add_token(EqualEqual)
                } else {
                    self.add_token(Equal)
                }
            }
            '<' => {
                if self.match_character('=') {
                    self.add_token(LessEqual)
                } else {
                    self.add_token(Less)
                }
            }
            '>' => {
                if self.match_character('=') {
                    self.add_token(GreaterEqual)
                } else {
                    self.add_token(Greater)
                }
            }
            '/' => {
                if self.match_character('/') {
                    // A comment goes until the end of the line.
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else if self.match_character('*') {
                    // /* comment goes until you reach this combination of symbols */
                    while !(self.peek() == '*' && self.peek_next() == '/') && !self.is_at_end() {
                        if self.peek() == '\n' {
                            self.line += 1;
                        }

                        self.advance();
                    }

                    if self.is_at_end() {
                        return Err(format!(
                            "Unterminated multiline comment at line {0}",
                            self.line
                        ));
                    } else {
                        for _ in 0..2 {
                            self.advance();
                        }
                    }
                } else {
                    self.add_token(Slash);
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
        let text = self.source[self.start..self.current].to_string();

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

        self.add_token_lit(String, Some(StringValue(value)));

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

        if is_fraction {
            match string_literal.parse::<f64>() {
                Ok(value) => self.add_token_lit(Number, Some(FValue(value))),
                _ => return Err(format!("Failed to parse the float at line: {0}", self.line)),
            }
        } else {
            match string_literal.parse::<i64>() {
                Ok(value) => self.add_token_lit(Number, Some(IntValue(value))),
                _ => return Err(format!("Failed to parse the int at line: {0}", self.line)),
            }
        }

        Ok(())
    }

    fn identifier(self: &mut Self) {
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let string_literal = self.source.as_bytes()[self.start..self.current]
            .iter()
            .map(|bytes| *bytes as char)
            .collect::<String>();

        let token_type = self.keywords.get(&string_literal);

        let token_type = match token_type {
            Some(token_val) => token_val,
            None => &Identifier,
        };

        self.add_token(token_type.clone());
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

        assert_eq!(scanner.tokens.len(), 5); // due to Eof token
        assert_eq!(scanner.tokens[0].token_type, LeftParen);
        assert_eq!(scanner.tokens[1].token_type, LeftParen);
        assert_eq!(scanner.tokens[2].token_type, RightParen);
        assert_eq!(scanner.tokens[3].token_type, RightParen);
        assert_eq!(scanner.tokens[4].token_type, Eof);
    }

    #[test]
    fn handler_two_char_tokens() {
        let source = "! != == >=";

        let mut scanner = Scanner::new(source);

        let _ = scanner.scan_tokens();

        assert_eq!(scanner.tokens.len(), 5); // due to Eof token
        assert_eq!(scanner.tokens[0].token_type, Bang);
        assert_eq!(scanner.tokens[1].token_type, BangEqual);
        assert_eq!(scanner.tokens[2].token_type, EqualEqual);
        assert_eq!(scanner.tokens[3].token_type, GreaterEqual);
        assert_eq!(scanner.tokens[4].token_type, Eof);
    }

    #[test]
    fn handler_string_literal() {
        let source = "\"ABC\"";

        let mut scanner = Scanner::new(source);

        let result = scanner.scan_tokens();

        assert!(result.is_ok());
        assert_eq!(scanner.tokens.len(), 2); // due to Eof token
        assert_eq!(scanner.tokens[0].token_type, String);
        match scanner.tokens[0].literal.as_ref().unwrap() {
            StringValue(val) => assert_eq!(val, "ABC"),
            _ => panic!("Incorrect literal"),
        }

        assert_eq!(scanner.tokens[1].token_type, Eof);
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
        assert_eq!(scanner.tokens[0].token_type, Number);
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
        assert_eq!(scanner.tokens[0].token_type, Number);
        match scanner.tokens[0].literal.as_ref().unwrap() {
            FValue(val) => assert_eq!(*val, 123.15),
            _ => panic!("Incorrect literal"),
        }

        assert_eq!(scanner.tokens[1].token_type, Eof);
    }

    #[test]
    fn handler_int_numeral_returns_int_token() {
        let source = "123";

        let mut scanner = Scanner::new(source);

        let result = scanner.scan_tokens();

        assert!(result.is_ok());
        assert_eq!(scanner.tokens.len(), 2);
        assert_eq!(scanner.tokens[0].token_type, Number);
        match scanner.tokens[0].literal.as_ref().unwrap() {
            IntValue(val) => assert_eq!(*val, 123),
            _ => panic!("Incorrect literal"),
        }

        assert_eq!(scanner.tokens[1].token_type, Eof);
    }

    #[test]
    fn handler_multiline_comments_gets_cut_of() {
        let source = "/*some text\n\n\n*/";

        let mut scanner = Scanner::new(source);

        let result = scanner.scan_tokens();

        assert!(result.is_ok());
        assert_eq!(scanner.tokens.len(), 1);
        assert_eq!(scanner.tokens[0].token_type, Eof);
    }

    #[test]
    fn handler_multiline_comments_unclosed_comment_returns_error() {
        let source = "/*some text\n\n";

        let mut scanner = Scanner::new(source);

        let result = scanner.scan_tokens();

        assert!(result.is_err());
        assert_eq!(
            result.err(),
            Some("Unterminated multiline comment at line 3\n".to_string())
        );
        assert_eq!(scanner.tokens.len(), 1);
        assert_eq!(scanner.tokens[0].token_type, Eof);
    }

    #[test]
    fn handler_multiline_comments_extra_end_comment_returns_error() {
        let source = "*/";

        let mut scanner = Scanner::new(source);

        let result = scanner.scan_tokens();

        assert!(result.is_err());
        assert_eq!(
            result.err(),
            Some("Extra multiline ending comment at line 1\n".to_string())
        );
        assert_eq!(scanner.tokens.len(), 1);
        assert_eq!(scanner.tokens[0].token_type, Eof);
    }

    #[test]
    fn handler_keywords_returns_keyword_token() {
        let source = "class var";

        let mut scanner = Scanner::new(source);

        let result = scanner.scan_tokens();

        assert!(result.is_ok());
        assert_eq!(scanner.tokens.len(), 3);
        assert_eq!(scanner.tokens[0].token_type, Class);
        assert_eq!(scanner.tokens[1].token_type, Var);
        assert_eq!(scanner.tokens[2].token_type, Eof);
    }

    #[test]
    fn handler_keywords_unindentified_token_returns_identifier_token() {
        let source = "bigvar";

        let mut scanner = Scanner::new(source);

        let result = scanner.scan_tokens();

        assert!(result.is_ok());
        assert_eq!(scanner.tokens.len(), 2);
        assert_eq!(scanner.tokens[0].token_type, Identifier);
        assert_eq!(scanner.tokens[0].lexeme, "bigvar");
        assert_eq!(scanner.tokens[1].token_type, Eof);
    }

    #[test]
    fn get_keywords() {
        let source = "var this_is_a_var = 12;\nwhile true { print 3 };";
        let mut scanner = Scanner::new(source);
        let result = scanner.scan_tokens();

        assert!(result.is_ok());

        assert_eq!(scanner.tokens.len(), 13);

        assert_eq!(scanner.tokens[0].token_type, Var);
        assert_eq!(scanner.tokens[1].token_type, Identifier);
        assert_eq!(scanner.tokens[2].token_type, Equal);
        assert_eq!(scanner.tokens[3].token_type, Number);
        assert_eq!(scanner.tokens[4].token_type, Semicolon);
        assert_eq!(scanner.tokens[5].token_type, While);
        assert_eq!(scanner.tokens[6].token_type, True);
        assert_eq!(scanner.tokens[7].token_type, LeftBrace);
        assert_eq!(scanner.tokens[8].token_type, Print);
        assert_eq!(scanner.tokens[9].token_type, Number);
        assert_eq!(scanner.tokens[10].token_type, RightBrace);
        assert_eq!(scanner.tokens[11].token_type, Semicolon);
        assert_eq!(scanner.tokens[12].token_type, Eof);
    }
}
