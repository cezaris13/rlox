use crate::expression;
use crate::token::Token;

#[derive(Debug, PartialEq)]
pub enum Statement {
    Expression {
        expression: expression::Expression,
    },
    Print {
        expression: expression::Expression,
    },
    Variable {
        token: Token,
        initializer: expression::Expression,
    },

    Block {
        statements: Vec<Statement>,
    },
}
