use crate::environment::Environment;
use crate::expression::{Expression, LiteralValue};
use crate::statement::Statement;

use std::rc::Rc;

pub struct Interpreter {
    environment: Rc<Environment>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Rc::new(Environment::new()),
        }
    }

    pub fn interpret_statements(&mut self, statements: Vec<Statement>) -> Result<(), String> {
        for statement in statements {
            match statement {
                Statement::Expression { expression } => {
                    let _ = expression.evaluate(
                        Rc::get_mut(&mut self.environment)
                            .expect("Could not get mutable reference to environment"),
                    )?;
                }
                Statement::Print { expression } => {
                    let result = expression.evaluate(
                        Rc::get_mut(&mut self.environment)
                            .expect("Could not get mutable reference to environment"),
                    )?;
                    println!("{}", result.to_string());
                }
                Statement::Variable { token, initializer } => {
                    let nil = Expression::Literal {
                        value: LiteralValue::Nil,
                    };

                    let value = if initializer != nil {
                        initializer.evaluate(
                            Rc::get_mut(&mut self.environment)
                                .expect("Could not get mutable reference to environment"),
                        )?
                    } else {
                        LiteralValue::Nil
                    };
                    Rc::get_mut(&mut self.environment)
                        .expect("Could not get mutable reference to environment")
                        .define(token.lexeme, value);
                }
                Statement::Block { statements } => {
                    let mut new_environment = Environment::new();
                    new_environment.enclosing = Some(self.environment.clone());

                    let old_environment = self.environment.clone();
                    self.environment = Rc::new(new_environment);
                    let block_result = self.interpret_statements(statements);
                    self.environment = old_environment;

                    block_result?
                }
            };
        }

        Ok(())
    }
}
