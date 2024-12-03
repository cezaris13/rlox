use crate::expr;
use crate::token::Token;

#[derive(Debug)]
pub enum Statement {
    Expression {
        expression: expr::Expression,
    },
    Print {
        expression: expr::Expression,
    },
    Variable {
        token: Token,
        initializer: expr::Expression,
    },
}
