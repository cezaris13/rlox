use crate::compare_values;
use crate::environment::Environment;
use crate::expression_literal_value::LiteralValue::{self, *};
use crate::token::{Token, TokenType::*};

use std::fmt::{Display, Formatter};
use std::string::String;

#[cfg(test)]
#[path = "./tests/expression_tests.rs"]
mod tests;

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
    Call {
        callee: Box<Expression>,
        paren: Token,
        arguments: Vec<Expression>,
    },
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Self::Binary {
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
            Self::Logical {
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
            Self::Grouping { group } => {
                format!("(group {})", group.to_string())
            }
            Self::Literal { value } => value.to_string(),
            Self::Unary { operator, right } => {
                format!("({} {})", operator.lexeme, right.to_string())
            }
            Self::Variable { token } => {
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
            Self::Assign { name, value } => format!("(= {} {})", name, value.to_string()),
            Self::Call {
                callee,
                paren: _,
                arguments,
            } => {
                let comma_separated = arguments
                    .iter()
                    .map(|val| val.to_string())
                    .collect::<Vec<String>>()
                    .join(",");

                format!("({} [{}])", callee, comma_separated)
            }
        };
        write!(f, "{}", str)
    }
}

impl Expression {
    pub fn evaluate(&self, environment: &mut Environment) -> Result<LiteralValue, String> {
        match self {
            Self::Literal { value } => Ok(value.clone()),
            Self::Grouping { group } => group.evaluate(environment),
            Self::Unary { operator, right } => {
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
            Self::Binary {
                left,
                operator,
                right,
            } => {
                let left = (*left).evaluate(environment)?;
                let right = (*right).evaluate(environment)?;

                match &operator.token_type {
                    Plus => left + right,
                    Minus => left - right,
                    Star => left * right,
                    Slash => left / right,
                    Greater => compare_values!(>, left, right),
                    GreaterEqual => compare_values!(>=, left, right),
                    Less => compare_values!(<, left, right),
                    LessEqual => compare_values!(<=, left, right),
                    BangEqual => compare_values!(!=, left, right),
                    EqualEqual => compare_values!(==, left, right),
                    _ => LiteralValue::not_implemented_error(
                        &stringify!(operator.token_type),
                        &left,
                        &right,
                    ),
                }
            }
            Self::Variable { token } => environment.get(&token.lexeme),
            Self::Assign { name, value } => {
                let value = value.evaluate(environment)?;
                environment.assign(name.clone(), value.clone())?; // temp fix
                Ok(value)
            }
            Self::Logical {
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
            Self::Call {
                callee,
                paren: _,
                arguments,
            } => {
                let callable = (*callee).evaluate(environment)?;
                match callable {
                    Callable { name, arity, fun } => {
                        if arity != arguments.len() {
                            return Err(format!(
                                "Expected {} arguments but got {}.",
                                arity,
                                arguments.len()
                            ));
                        }

                        let mut parameters = vec![];
                        for argument in arguments {
                            let literal = argument.evaluate(environment)?;
                            parameters.push(literal);
                        }

                        // figure out if the variable can be the same name as the function??
                        if let Err(_) = environment.get(&name) {
                            return Err(format!("undefined function {}", name));
                        }

                        Ok(fun(&parameters))
                    }
                    _ => Err(format!("Cannot use {} as callable", callable.to_type())),
                }
            }
        }
    }
}
