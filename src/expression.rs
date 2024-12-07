use crate::environment::Environment;
use crate::expression::LiteralValue::*;
use crate::token::TokenType::*;
use crate::token::{Token, TokenType};

use std::string::String;

#[cfg(test)]
#[path = "./tests/expression_tests.rs"]
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

impl LiteralValue {
    pub fn to_string(&self) -> String {
        match self {
            IntValue(integer) => integer.to_string(),
            FValue(float) => float.to_string(),
            StringValue(string) => string.clone(),
            Self::True => String::from("true"),
            Self::False => String::from("false"),
            Self::Nil => String::from("nil"),
        }
    }

    pub fn to_type(&self) -> &str {
        match self {
            IntValue(_) => "Int",
            FValue(_) => "Float",
            Self::True | Self::False => "Bool",
            StringValue(_) => "String",
            Self::Nil => "Nil",
        }
    }

    pub fn from_token(token: Token) -> Self {
        match token.token_type {
            TokenType::Number => match token.literal {
                Some(crate::token::LiteralValue::IntValue(int_value)) => IntValue(int_value),
                Some(crate::token::LiteralValue::FValue(float_value)) => FValue(float_value),
                _ => panic!("Could not unwrap as number"),
            },
            TokenType::String => match token.literal {
                Some(crate::token::LiteralValue::StringValue(string_value)) => {
                    StringValue(string_value)
                }
                Some(crate::token::LiteralValue::IdentifierValue(id_value)) => {
                    StringValue(id_value)
                }
                _ => panic!("Could not unwrap as String"),
            },
            TokenType::False => Self::False,
            TokenType::True => Self::True,
            TokenType::Nil => Self::Nil,
            _ => panic!("Could not create literal value from {:?}", token),
        }
    }

    pub fn is_falsy(&self) -> Self {
        match self {
            IntValue(x) => Self::bool_to_literal_bool(*x == 0),
            FValue(x) => Self::bool_to_literal_bool(*x == 0.0),
            StringValue(string) => Self::bool_to_literal_bool(string.len() == 0),
            Self::True => Self::False,
            Self::False => Self::True,
            Self::Nil => Self::True,
        }
    }

    pub fn literal_bool_to_bool(&self) -> bool {
        if *self == Self::True {
            true
        } else {
            false
        }
    }

    pub fn bool_to_literal_bool(expression: bool) -> Self {
        if expression {
            Self::True
        } else {
            Self::False
        }
    }
}

#[derive(Debug, PartialEq)]
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

impl Expression {
    pub fn to_string(&self) -> String {
        match self {
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
                        "(defvar {} {:?})",
                        token.lexeme,
                        LiteralValue::from_token(token.clone()).to_string()
                    )
                } else {
                    format!("(defvar {})", token.lexeme)
                }
            }
            Expression::Assign { name, value } => format!("(= {} {})", name, value.to_string()),
        }
    }

    // region evaluation

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
                    (any, Bang) => Ok(any.is_falsy()),
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
                    Plus => self.process_plus_operator(left, right),
                    Minus => self.process_minus_operator(left, right),
                    Star => self.process_star_operator(left, right),
                    Slash => self.process_slash_operator(left, right),
                    Greater => self.process_greater_operator(left, right),
                    GreaterEqual => self.process_greater_equal_operator(left, right),
                    Less => self.process_less_operator(left, right),
                    LessEqual => self.process_less_equal_operator(left, right),
                    BangEqual => Ok(LiteralValue::bool_to_literal_bool(left != right)),
                    EqualEqual => Ok(LiteralValue::bool_to_literal_bool(left == right)),
                    _ => self.not_implemented_error(&operator.token_type, &left, &right),
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
                    if !left_value.is_falsy().literal_bool_to_bool() {
                        return Ok(left_value);
                    }
                } else {
                    if left_value.is_falsy().literal_bool_to_bool() {
                        return Ok(left_value);
                    }
                }

                right.evaluate(environment)
            }
        }
    }

    fn process_plus_operator(
        &self,
        left: LiteralValue,
        right: LiteralValue,
    ) -> Result<LiteralValue, String> {
        match (&left, &right) {
            (IntValue(x), IntValue(y)) => Ok(IntValue(x + y)),
            (FValue(x), FValue(y)) => Ok(FValue(x + y)),
            (IntValue(x), FValue(y)) => Ok(FValue((*x as f64) + y)),
            (FValue(x), IntValue(y)) => Ok(FValue(x + (*y as f64))),
            (StringValue(string), any) => {
                Ok(StringValue(format!("{0}{1}", string, any.to_string())))
            }
            (any, StringValue(string)) => {
                Ok(StringValue(format!("{0}{1}", any.to_string(), string)))
            }
            _ => self.not_implemented_error(&Plus, &left, &right),
        }
    }

    fn process_minus_operator(
        &self,
        left: LiteralValue,
        right: LiteralValue,
    ) -> Result<LiteralValue, String> {
        match (&left, &right) {
            (IntValue(x), IntValue(y)) => Ok(IntValue(x - y)),
            (FValue(x), FValue(y)) => Ok(FValue(x - y)),
            (IntValue(x), FValue(y)) => Ok(FValue((*x as f64) - y)),
            (FValue(x), IntValue(y)) => Ok(FValue(x - (*y as f64))),
            _ => self.not_implemented_error(&Minus, &left, &right),
        }
    }

    fn process_star_operator(
        &self,
        left: LiteralValue,
        right: LiteralValue,
    ) -> Result<LiteralValue, String> {
        match (&left, &right) {
            (IntValue(x), IntValue(y)) => Ok(IntValue(x * y)),
            (FValue(x), FValue(y)) => Ok(FValue(x * y)),
            (IntValue(x), FValue(y)) => Ok(FValue((*x as f64) * y)),
            (FValue(x), IntValue(y)) => Ok(FValue(x * (*y as f64))),
            _ => self.not_implemented_error(&Star, &left, &right),
        }
    }

    fn process_slash_operator(
        &self,
        left: LiteralValue,
        right: LiteralValue,
    ) -> Result<LiteralValue, String> {
        if matches!(right, IntValue(0) | FValue(0.0)) {
            return Err(String::from("Division by 0"));
        }

        match (&left, &right) {
            (IntValue(x), IntValue(y)) => Ok(IntValue(x / y)),
            (FValue(x), FValue(y)) => Ok(FValue(x / y)),
            (IntValue(x), FValue(y)) => Ok(FValue((*x as f64) / y)),
            (FValue(x), IntValue(y)) => Ok(FValue(x / (*y as f64))),
            _ => self.not_implemented_error(&Slash, &left, &right),
        }
    }

    fn process_greater_operator(
        &self,
        left: LiteralValue,
        right: LiteralValue,
    ) -> Result<LiteralValue, String> {
        match (&left, &right) {
            (IntValue(x), IntValue(y)) => Ok(LiteralValue::bool_to_literal_bool(x > y)),
            (FValue(x), FValue(y)) => Ok(LiteralValue::bool_to_literal_bool(x > y)),
            (IntValue(x), FValue(y)) => Ok(LiteralValue::bool_to_literal_bool(*x as f64 > *y)),
            (FValue(x), IntValue(y)) => Ok(LiteralValue::bool_to_literal_bool(*x > *y as f64)),
            (StringValue(x), StringValue(y)) => Ok(LiteralValue::bool_to_literal_bool(x > y)),
            _ => self.not_implemented_error(&Greater, &left, &right),
        }
    }

    fn process_greater_equal_operator(
        &self,
        left: LiteralValue,
        right: LiteralValue,
    ) -> Result<LiteralValue, String> {
        match (&left, &right) {
            (IntValue(x), IntValue(y)) => Ok(LiteralValue::bool_to_literal_bool(x >= y)),
            (FValue(x), FValue(y)) => Ok(LiteralValue::bool_to_literal_bool(x >= y)),
            (IntValue(x), FValue(y)) => Ok(LiteralValue::bool_to_literal_bool(*x as f64 >= *y)),
            (FValue(x), IntValue(y)) => Ok(LiteralValue::bool_to_literal_bool(*x >= *y as f64)),
            (StringValue(x), StringValue(y)) => Ok(LiteralValue::bool_to_literal_bool(x >= y)),
            _ => self.not_implemented_error(&GreaterEqual, &left, &right),
        }
    }

    fn process_less_operator(
        &self,
        left: LiteralValue,
        right: LiteralValue,
    ) -> Result<LiteralValue, String> {
        match (&left, &right) {
            (IntValue(x), IntValue(y)) => Ok(LiteralValue::bool_to_literal_bool(x < y)),
            (FValue(x), FValue(y)) => Ok(LiteralValue::bool_to_literal_bool(x < y)),
            (IntValue(x), FValue(y)) => Ok(LiteralValue::bool_to_literal_bool((*x as f64) < *y)),
            (FValue(x), IntValue(y)) => Ok(LiteralValue::bool_to_literal_bool(*x < *y as f64)),
            (StringValue(x), StringValue(y)) => Ok(LiteralValue::bool_to_literal_bool(x < y)),
            _ => self.not_implemented_error(&Less, &left, &right),
        }
    }

    fn process_less_equal_operator(
        &self,
        left: LiteralValue,
        right: LiteralValue,
    ) -> Result<LiteralValue, String> {
        match (&left, &right) {
            (IntValue(x), IntValue(y)) => Ok(LiteralValue::bool_to_literal_bool(x <= y)),
            (FValue(x), FValue(y)) => Ok(LiteralValue::bool_to_literal_bool(x <= y)),
            (IntValue(x), FValue(y)) => Ok(LiteralValue::bool_to_literal_bool(*x as f64 <= *y)),
            (FValue(x), IntValue(y)) => Ok(LiteralValue::bool_to_literal_bool(*x <= *y as f64)),
            (StringValue(x), StringValue(y)) => Ok(LiteralValue::bool_to_literal_bool(x <= y)),
            _ => self.not_implemented_error(&LessEqual, &left, &right),
        }
    }

    fn not_implemented_error(
        &self,
        token_type: &TokenType,
        left: &LiteralValue,
        right: &LiteralValue,
    ) -> Result<LiteralValue, String> {
        Err(format!(
            "{:?} operation is not implemented for: {:?} and {:?}",
            token_type,
            left.to_string(),
            right.to_string()
        ))
    }

    // endregion
}
