use crate::token::LiteralValue::*;
use crate::token::TokenType::*;
use crate::token::{LiteralValue, Token, TokenType};

use std::collections::HashMap;
use std::string::String;

#[cfg(test)]
#[path = "./tests/scanner_tests.rs"]
mod tests;

pub struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    keywords: HashMap<&'a str, TokenType>,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut keywords: HashMap<&str, TokenType> = HashMap::new();

        keywords.insert("and", And);
        keywords.insert("class", Class);
        keywords.insert("else", Else);
        keywords.insert("false", False);
        keywords.insert("for", For);
        keywords.insert("fun", Fun);
        keywords.insert("if", If);
        keywords.insert("nil", Nil);
        keywords.insert("or", Or);
        keywords.insert("print", Print);
        keywords.insert("return", Return);
        keywords.insert("super", Super);
        keywords.insert("this", This);
        keywords.insert("true", True);
        keywords.insert("var", Var);
        keywords.insert("while", While);

        Self {
            source: source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
            keywords: keywords,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, String> {
        let mut errors = vec![];

        while !self.is_at_end() {
            self.start = self.current;
            if let Err(message) = self.scan_token() {
                errors.push(message);
            }
        }

        self.tokens
            .push(Token::new(Eof, "".to_string(), None, self.line));

        if errors.len() > 0 {
            let joined = errors.join("\n");
            return Err(joined);
        }

        Ok(self.tokens.clone()) // temp fix
    }

    fn scan_token(&mut self) -> Result<(), String> {
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

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_lit(token_type, None);
    }

    fn add_token_lit(&mut self, token_type: TokenType, literal: Option<LiteralValue>) {
        let text = self.source[self.start..self.current].to_string();

        self.tokens
            .push(Token::new(token_type, text, literal, self.line));
    }

    // region parser function

    fn string(&mut self) -> Result<(), String> {
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
        let value = &self.source[self.start + 1..self.current - 1];

        self.add_token_lit(String, Some(StringValue(value.to_string())));

        Ok(())
    }

    fn number(&mut self) -> Result<(), String> {
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

        let string_literal = &self.source[self.start..self.current];

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

    fn identifier(&mut self) {
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let string_literal = &self.source[self.start..self.current];

        let token_type = self.keywords.get(string_literal);

        let token_type = token_type.unwrap_or_else(|| &Identifier);

        self.add_token(token_type.clone());
    }

    // endregion

    // region character manipulation

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
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

    fn match_character(&mut self, character: char) -> bool {
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
