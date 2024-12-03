use crate::expr;

#[derive(Debug)]
pub enum Statement {
    Expression { expression: expr::Expression },
    Print { expression: expr::Expression },
}
