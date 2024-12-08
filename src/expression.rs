use crate::environment::Environment;
use crate::expression_literal_value::LiteralValue::{self, *};
use crate::token::{Token, TokenType::*};

use std::fmt::{Display, Formatter};
use std::string::String;

#[cfg(test)]
#[path = "./tests/expression_tests.rs"]
mod tests;

macro_rules! compare_values {
    ($self:ident, $op_symbol:tt, $left:expr, $right:expr) => {
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
    ($self:ident, $op_symbol:tt, $left: expr, $right: expr) => {
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

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    Binary {
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>,
    },
    Unary {
        operator: Token,
        right: Box<Expression>,
    },
    Grouping {
        group: Box<Expression>,
    },
    Literal {
        value: LiteralValue,
    },
    Variable {
        token: Token,
    },
    Assign {
        name: String,
        value: Box<Expression>,
    },
    Logical {
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>,
    },
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Expression::Binary {
                left,
                operator,
                right,
            } => {
                format!(
                    "({} {} {})",
                    operator.lexeme,
                    left.to_string(),
                    right.to_string()
                )
            }
            Expression::Logical {
                left,
                operator,
                right,
            } => {
                format!(
                    "({} {} {})",
                    operator.lexeme,
                    left.to_string(),
                    right.to_string()
                )
            }
            Expression::Grouping { group } => {
                format!("(group {})", group.to_string())
            }
            Expression::Literal { value } => value.to_string(),
            Expression::Unary { operator, right } => {
                format!("({} {})", operator.lexeme, right.to_string())
            }
            Expression::Variable { token } => {
                if let Some(_) = &token.literal {
                    format!(
                        "(defvar {} {})",
                        token.lexeme,
                        LiteralValue::from(token.clone())
                    )
                } else {
                    format!("(defvar {})", token.lexeme)
                }
            }
            Expression::Assign { name, value } => format!("(= {} {})", name, value.to_string()),
        };
        write!(f, "{}", str)
    }
}

impl Expression {
    pub fn evaluate(&self, environment: &mut Environment) -> Result<LiteralValue, String> {
        match self {
            Expression::Literal { value } => Ok(value.clone()),
            Expression::Grouping { group } => group.evaluate(environment),
            Expression::Unary { operator, right } => {
                let right = (*right).evaluate(environment)?;

                match (&right, &operator.token_type) {
                    (IntValue(value), Minus) => Ok(IntValue(-value)),
                    (FValue(value), Minus) => Ok(FValue(-value)),
                    (_, Minus) => Err(format!("Minus not implemented for {}", right.to_type())),
                    (any, Bang) => Ok(LiteralValue::from(!bool::from(any))),
                    _ => Err(format!(
                        "Non unary operator {:?} is not implemented for {}",
                        operator.token_type,
                        right.to_type(),
                    )),
                }
            }
            Expression::Binary {
                left,
                operator,
                right,
            } => {
                let left = (*left).evaluate(environment)?;
                let right = (*right).evaluate(environment)?;

                match &operator.token_type {
                    Plus => arithmetic_operation!(self, +, left, right),
                    Minus => arithmetic_operation!(self, -, left, right),
                    Star => arithmetic_operation!(self, *, left, right),
                    Slash => arithmetic_operation!(self, /, left, right),
                    Greater => compare_values!(self, >, left, right),
                    GreaterEqual => compare_values!(self, >=, left, right),
                    Less => compare_values!(self, <, left, right),
                    LessEqual => compare_values!(self, <=, left, right),
                    BangEqual => Ok(LiteralValue::from(left != right)),
                    EqualEqual => Ok(LiteralValue::from(left == right)),
                    _ => LiteralValue::not_implemented_error(
                        &stringify!(operator.token_type),
                        &left,
                        &right,
                    ),
                }
            }
            Expression::Variable { token } => environment.get(&token.lexeme),
            Expression::Assign { name, value } => {
                let value = value.evaluate(environment)?;
                environment.assign(name.clone(), value.clone())?; // temp fix
                Ok(value)
            }
            Expression::Logical {
                left,
                operator,
                right,
            } => {
                let left_value = left.evaluate(environment)?;

                if operator.token_type == Or {
                    if bool::from(&left_value) {
                        return Ok(left_value);
                    }
                } else {
                    if !bool::from(&left_value) {
                        return Ok(left_value);
                    }
                }

                right.evaluate(environment)
            }
        }
    }
}
