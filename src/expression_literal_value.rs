use crate::expression_literal_value::LiteralValue::*;
use crate::token::{LiteralValue as TokenLiteralValue, Token, TokenType};

use std::fmt::{Display, Formatter};
use std::ops;
use std::string::String;

#[cfg(test)]
#[path = "./tests/expression_literal_value_tests.rs"]
mod tests;

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
                Some(TokenLiteralValue::StringValue(string_value)) => StringValue(string_value),
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

#[macro_export]
macro_rules! compare_values {
    ($op_symbol:tt, $left:expr, $right:expr) => {
        match (&$left, &$right) {
            (IntValue(x), IntValue(y)) => Ok(LiteralValue::from(x $op_symbol y)),
            (FValue(x), FValue(y)) => Ok(LiteralValue::from(x $op_symbol y)),
            (IntValue(x), FValue(y)) => Ok(LiteralValue::from((*x as f64) $op_symbol *y)),
            (FValue(x), IntValue(y)) => Ok(LiteralValue::from(*x $op_symbol (*y as f64))),
            (StringValue(x), StringValue(y)) => Ok(LiteralValue::from(x $op_symbol y)),
            _ => LiteralValue::not_implemented_error(&stringify!($op_symbol), &$left, &$right),
        }
    };
}

macro_rules! arithmetic_operation {
    ($left: expr, $op_symbol:tt, $right: expr) => {
        {
            if stringify!($op_symbol) == "/" && matches!($right, IntValue(0) | FValue(0.0)) {
                return Err(String::from("Division by 0"));
            }

            match (&$left, &$right) {
                (IntValue(x), IntValue(y)) => Ok(IntValue(x $op_symbol y)),
                (FValue(x), FValue(y)) => Ok(FValue(x $op_symbol y)),
                (IntValue(x), FValue(y)) => Ok(FValue((*x as f64) $op_symbol y)),
                (FValue(x), IntValue(y)) => Ok(FValue(x $op_symbol (*y as f64))),
                (StringValue(string), any) if stringify!($op_symbol) == "+" => {
                    Ok(StringValue(format!("{0}{1}", string, any)))
                }
                (any, StringValue(string)) if stringify!($op_symbol) == "+" => {
                    Ok(StringValue(format!("{0}{1}", any, string)))
                }
                _ => LiteralValue::not_implemented_error(&stringify!($op_symbol), &$left, &$right),
            }
        }
    };
}

impl ops::Add<LiteralValue> for LiteralValue {
    type Output = Result<Self, String>;

    fn add(self, _rhs: Self) -> Self::Output {
        arithmetic_operation!(&self, +, &_rhs)
    }
}

impl ops::Sub<LiteralValue> for LiteralValue {
    type Output = Result<Self, String>;

    fn sub(self, _rhs: Self) -> Self::Output {
        arithmetic_operation!(&self, -, &_rhs)
    }
}

impl ops::Mul<LiteralValue> for LiteralValue {
    type Output = Result<Self, String>;

    fn mul(self, _rhs: Self) -> Self::Output {
        arithmetic_operation!(&self, *, &_rhs)
    }
}

impl ops::Div<LiteralValue> for LiteralValue {
    type Output = Result<Self, String>;

    fn div(self, _rhs: Self) -> Self::Output {
        arithmetic_operation!(&self, /, &_rhs)
    }
}