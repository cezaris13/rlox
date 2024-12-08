use crate::expression_literal_value::LiteralValue::*;
use crate::token::{Token, TokenType, LiteralValue as TokenLiteralValue};

use std::fmt::{Display, Formatter};
use std::string::String;

#[derive(Clone, Debug, PartialEq)]
pub enum LiteralValue {
    IntValue(i64),
    FValue(f64),
    StringValue(String),
    True,
    False,
    Nil,
}

impl From<Token> for LiteralValue {
    fn from(token: Token) -> Self {
        match token.token_type {
            TokenType::Number => match token.literal {
                Some(TokenLiteralValue::IntValue(int_value)) => IntValue(int_value),
                Some(TokenLiteralValue::FValue(float_value)) => FValue(float_value),
                _ => panic!("Could not unwrap as number"),
            },
            TokenType::String => match token.literal {
                Some(TokenLiteralValue::StringValue(string_value)) => {
                    StringValue(string_value)
                }
                _ => panic!("Could not unwrap as String"),
            },
            TokenType::False => Self::False,
            TokenType::True => Self::True,
            TokenType::Nil => Self::Nil,
            _ => panic!("Could not create literal value from {}", token),
        }
    }
}

impl From<bool> for LiteralValue {
    fn from(boolean: bool) -> Self {
        if boolean {
            Self::True
        } else {
            Self::False
        }
    }
}

impl From<&LiteralValue> for bool {
    fn from(literal_value: &LiteralValue) -> Self {
        match literal_value {
            IntValue(x) => *x != 0,
            FValue(x) => *x != 0.0,
            StringValue(string) => string.len() != 0,
            LiteralValue::True => true,
            LiteralValue::False => false,
            LiteralValue::Nil => false,
        }
    }
}

impl From<LiteralValue> for bool {
    fn from(literal_value: LiteralValue) -> Self {
        Self::from(&literal_value)
    }
}

impl Display for LiteralValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            IntValue(integer) => integer.to_string(),
            FValue(float) => float.to_string(),
            StringValue(string) => string.clone(),
            Self::True => String::from("true"),
            Self::False => String::from("false"),
            Self::Nil => String::from("nil"),
        };
        write!(f, "{}", str)
    }
}

impl LiteralValue {
    pub fn to_type(&self) -> &str {
        match self {
            IntValue(_) => "Int",
            FValue(_) => "Float",
            Self::True | Self::False => "Bool",
            StringValue(_) => "String",
            Self::Nil => "Nil",
        }
    }

    pub fn not_implemented_error(
        token_type: &str,
        left: &LiteralValue,
        right: &LiteralValue,
    ) -> Result<LiteralValue, String> {
        Err(format!(
            "{:?} operation is not implemented for: {} and {}",
            token_type.parse::<TokenType>()?,
            left,
            right
        ))
    }
}
