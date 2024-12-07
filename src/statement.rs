use crate::expression;
use crate::token::Token;

#[derive(Clone, Debug, PartialEq)]
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

    If {
        condition: expression::Expression,
        then_branch: Box<Statement>,
        else_branch: Option<Box<Statement>>,
    },

    While {
        condition: expression::Expression,
        body: Box<Statement>,
    },
}
