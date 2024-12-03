use crate::expr::{Expression, LiteralValue};
use crate::stmt::Statement;

pub struct Interpreter {
    // global state
    //
}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn interpret_expression(&mut self, expression: Expression) -> Result<LiteralValue, String> {
        expression.evaluate()
    }

    pub fn interpret_statements(&mut self, statements: Vec<Statement>) -> Result<(), String> {
        for statement in statements {
            match statement {
                Statement::Expression { expression } => {
                    let _ = expression.evaluate()?;
                }
                Statement::Print { expression } => {
                    let result = expression.evaluate()?;
                    println!("{}", result.to_string());
                }
                Statement::Variable { token, initializer } => {
                    println!("token: {:?}, initializer: {:?}", token, initializer);
                }
            };
        }

        Ok(())
    }
}
