use crate::environment::Environment;
use crate::expr::{Expression, LiteralValue};
use crate::stmt::Statement;

pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
        }
    }

    pub fn interpret_expression(&mut self, expression: Expression) -> Result<LiteralValue, String> {
        expression.evaluate(&mut self.environment)
    }

    pub fn interpret_statements(&mut self, statements: Vec<Statement>) -> Result<(), String> {
        for statement in statements {
            match statement {
                Statement::Expression { expression } => {
                    let _ = expression.evaluate(&mut self.environment)?;
                }
                Statement::Print { expression } => {
                    let result = expression.evaluate(&mut self.environment)?;
                    println!("{}", result.to_string());
                }
                Statement::Variable { token, initializer } => {
                    let nil = Expression::Literal {
                        value: LiteralValue::Nil,
                    };

                    let value = if initializer != nil {
                        initializer.evaluate(&mut self.environment)?
                    } else {
                        LiteralValue::Nil
                    };

                    self.environment.define(&token.lexeme, &value);
                }
                Statement::Block { statements } => {
                    for statement in statements {
                        println!("{:?}", statement);
                    }
                }
            };
        }

        Ok(())
    }
}
