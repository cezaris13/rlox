use crate::token_type::TokenType;

#[derive(Debug)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub lexeme: &'a str,
    pub literal: &'a str, // change to obj?
    pub line: i32,
}

impl Token<'_> {
    pub fn to_string(&self) -> String {
        format!("{:?} {1} {2}", self.token_type, self.lexeme, self.literal)
    }
}

pub struct Scanner<'a> {
    pub source: &'a str,
}

impl Scanner<'_> {
    pub fn scan_tokens(&self) -> Vec<Token> {
        let mut vec: Vec<Token> = Vec::new();

        for _ in 1..5 {
            vec.push(Token {
                token_type: TokenType::IF,
                lexeme: "some str",
                literal: "some literal",
                line: 12,
            });
        }

        vec
    }
}
