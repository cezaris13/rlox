use crate::token::{LiteralValue, Token, TokenType};

pub struct Scanner<'a> {
    pub source: &'a str,
    pub tokens: Vec<Token>,
}

impl Scanner<'_> {
    pub fn scan_tokens(&self) -> Result<Vec<Token>, String> {
        let mut vec: Vec<Token> = Vec::new();

        for _ in 1..5 {
            vec.push(Token {
                token_type: TokenType::IF,
                lexeme: String::from("some str"),
                literal: Some(LiteralValue::IntValue(12)),
                line: 12,
            });
        }

        Ok(vec)
    }
}
