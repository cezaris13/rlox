use crate::token::Token;
use crate::token::TokenType::*;

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
            LiteralValue::IntValue(integer) => integer.to_string(),
            LiteralValue::FValue(float) => float.to_string(),
            LiteralValue::StringValue(string) => string.clone(),
            LiteralValue::True => String::from("true"),
            LiteralValue::False => String::from("false"),
            LiteralValue::Nil => String::from("nil"),
        }
    }

    pub fn from_token(token: Token) -> Self {
        match token.token_type {
            NUMBER => match token.literal {
                Some(crate::token::LiteralValue::IntValue(int_value)) => Self::IntValue(int_value),
                Some(crate::token::LiteralValue::FValue(float_value)) => Self::FValue(float_value),
                _ => panic!("Could not unwrap as number"),
            },
            STRING => match token.literal {
                Some(crate::token::LiteralValue::StringValue(string_value)) => {
                    Self::StringValue(string_value)
                }
                Some(crate::token::LiteralValue::IdentifierValue(id_value)) => {
                    Self::StringValue(id_value)
                }
                _ => panic!("Could not unwrap as String"),
            },
            FALSE => Self::False,
            TRUE => Self::True,
            NIL => Self::Nil,
            _ => panic!("Could not create literal calue from {:?}", token),
        }
    }
}

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
            Expression::Grouping { group } => {
                format!("(group {})", group.to_string())
            }
            Expression::Literal { value } => value.to_string(),
            Expression::Unary { operator, right } => {
                format!("({} {})", operator.lexeme, right.to_string())
            }
        }
    }

    pub fn print(&self) {
        println!("{}", self.to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr::Expression;
    use crate::expr::Expression::*;
    use crate::expr::LiteralValue::*;
    use crate::token::TokenType::*;

    #[test]
    fn pretty_print() {
        let expression: Expression = Binary {
            left: Box::new(Unary {
                operator: Token::new(MINUS, String::from("-"), None, 1),
                right: Box::new(Literal {
                    value: IntValue(123),
                }),
            }),
            operator: Token::new(STAR, String::from("*"), None, 1),
            right: Box::new(Grouping {
                group: Box::new(Literal {
                    value: FValue(45.67),
                }),
            }),
        };

        let result = expression.to_string();
        assert_eq!(result, "(* (- 123) (group 45.67))");
    }
}
