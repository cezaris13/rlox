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

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len() as usize
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
            _ => {
                return Err(format!(
                    "Unexpected character {0} at line {1}",
                    symbol, self.line
                ))
            }
        }
        Ok(())
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

    fn advance(self: &mut Self) -> char {
        let symbol = self.source.as_bytes()[self.current];
        self.current += 1;
        symbol as char
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

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.as_bytes()[self.current] as char
    }
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
}
