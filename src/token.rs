use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

impl FromStr for TokenType {
    type Err = String;

    fn from_str(input: &str) -> Result<TokenType, Self::Err> {
        let token_type = match input {
            "LeftParen" => TokenType::LeftParen,
            "RightParen" => TokenType::RightParen,
            "LeftBrace" => TokenType::LeftBrace,
            "RightBrace" => TokenType::RightBrace,
            "Comma" => TokenType::Comma,
            "Dot" => TokenType::Dot,
            "-" | "Minus" => TokenType::Minus,
            "+" | "Plus" => TokenType::Plus,
            ";" => TokenType::Semicolon,
            "/" | "Slash" => TokenType::Slash,
            "*" | "Star" => TokenType::Star,

            // One or two character tokens
            "!" => TokenType::Bang,
            "BangEqual" => TokenType::BangEqual,
            "=" => TokenType::Equal,
            "EqualEqual" => TokenType::EqualEqual,
            ">" | "Greater" => TokenType::Greater,
            ">=" | "GreaterEqual" => TokenType::GreaterEqual,
            "<" | "Less" => TokenType::Less,
            "<=" | "LessEqual" => TokenType::LessEqual,

            // Keywords
            "And" => TokenType::And,
            "Class" => TokenType::Class,
            "Else" => TokenType::Else,
            "False" => TokenType::False,
            "Fun" => TokenType::Fun,
            "For" => TokenType::For,
            "If" => TokenType::If,
            "Nil" => TokenType::Nil,
            "Or" => TokenType::Or,
            "Print" => TokenType::Print,
            "Return" => TokenType::Return,
            "Super" => TokenType::Super,
            "This" => TokenType::This,
            "True" => TokenType::True,
            "Var" => TokenType::Var,
            "While" => TokenType::While,

            // Literals
            "Identifier" => TokenType::Identifier,
            "String" => TokenType::String,
            "Number" => TokenType::Number,

            // End of file
            "Eof" => TokenType::Eof,

            _ => return Err(format!("Could not convert '{}' to TokenType", input)),
        };

        Ok(token_type)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralValue {
    IntValue(i64),
    FValue(f64),
    StringValue(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<LiteralValue>,
    pub line: usize,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<LiteralValue>,
        line: usize,
    ) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Token {{ type: {:?}, lexeme: '{}', literal: {:?}, line: {} }}",
            self.token_type, self.lexeme, self.literal, self.line
        )
    }
}
