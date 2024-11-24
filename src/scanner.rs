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
            _ => {
                return Err(format!(
                    "Unexpected character {0} at line {1}",
                    symbol, self.line
                ))
            }
        }
        Ok(())
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
}
