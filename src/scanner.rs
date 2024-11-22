#[derive(Debug)]
pub struct Token {}

pub struct Scanner<'a> {
    pub source: &'a str,
}

impl Scanner<'_> {
    pub fn scan_tokens(&self) -> Vec<Token> {
        let mut vec: Vec<Token> = Vec::new();

        for i in 1..5 {
            vec.push(Token {});
        }

        vec
    }
}
