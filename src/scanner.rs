use crate::token::{LiteralValue, Token, TokenType};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: u64,
    current: u64,
    line: u64,
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
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }

        for _ in 1..5 {
            self.tokens.push(Token::new(
                TokenType::IF,
                String::from("some str"),
                Some(LiteralValue::IntValue(12)),
                12,
            ));
        }

        self.tokens
            .push(Token::new(TokenType::EOF, "".to_string(), None, self.line));

        Ok(self.tokens.clone()) // temp fix
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len() as u64
    }

    fn scan_token(self: &mut Self) -> Result<Token, String> {
        todo!()
    }
}
