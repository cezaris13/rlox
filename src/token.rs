use std::str::FromStr;
use std::fmt::{Display, Formatter};

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
        match input {
            "LeftParen" => Ok(TokenType::LeftParen),
            "RightParen" => Ok(TokenType::RightParen),
            "LeftBrace" => Ok(TokenType::LeftBrace),
            "RightBrace" => Ok(TokenType::RightBrace),
            "Comma" => Ok(TokenType::Comma),
            "Dot" => Ok(TokenType::Dot),
            "-" | "Minus" => Ok(TokenType::Minus),
            "+" | "Plus" => Ok(TokenType::Plus),
            ";" => Ok(TokenType::Semicolon),
            "/" | "Slash" => Ok(TokenType::Slash),
            "*" | "Star" => Ok(TokenType::Star),

            // One or two character tokens
            "!" => Ok(TokenType::Bang),
            "BangEqual" => Ok(TokenType::BangEqual),
            "=" => Ok(TokenType::Equal),
            "EqualEqual" => Ok(TokenType::EqualEqual),
            ">" | "Greater" => Ok(TokenType::Greater),
            ">=" | "GreaterEqual" => Ok(TokenType::GreaterEqual),
            "<" | "Less" => Ok(TokenType::Less),
            "<=" | "LessEqual" => Ok(TokenType::LessEqual),

            // Keywords
            "And" => Ok(TokenType::And),
            "Class" => Ok(TokenType::Class),
            "Else" => Ok(TokenType::Else),
            "False" => Ok(TokenType::False),
            "Fun" => Ok(TokenType::Fun),
            "For" => Ok(TokenType::For),
            "If" => Ok(TokenType::If),
            "Nil" => Ok(TokenType::Nil),
            "Or" => Ok(TokenType::Or),
            "Print" => Ok(TokenType::Print),
            "Return" => Ok(TokenType::Return),
            "Super" => Ok(TokenType::Super),
            "This" => Ok(TokenType::This),
            "True" => Ok(TokenType::True),
            "Var" => Ok(TokenType::Var),
            "While" => Ok(TokenType::While),

            // Literals
            "Identifier" => Ok(TokenType::Identifier),
            "String" => Ok(TokenType::String),
            "Number" => Ok(TokenType::Number),

            // End of file
            "Eof" => Ok(TokenType::Eof),

            _ => Err(format!("Could not convert '{}' to TokenType", input)),
        }
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