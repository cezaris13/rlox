use crate::token::Token;

pub enum LiteralValue {
    IntValue(i64),
    FValue(i64),
    True,
    False,
    Nil,
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
        operator: Box<Expression>,
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
            Expression::Grouping { operator } => {
                format!("({})", operator.to_string())
            }
            Expression::Literal { value } => match value {
                LiteralValue::IntValue(integer) => integer.to_string(),
                LiteralValue::FValue(float) => float.to_string(),
                LiteralValue::True => String::from("true"),
                LiteralValue::False => String::from("false"),
                LiteralValue::Nil => String::from("nil"),
            },
            Expression::Unary { operator, right } => {
                format!("({} {})", operator.lexeme, right.to_string())
            }
        }
    }

    pub fn print(&self) {
        println!("{}", self.to_string());
    }
}
