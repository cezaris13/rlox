use crate::environment::Environment;
use crate::expression::Expression;
use crate::expression_literal_value::LiteralValue;
use crate::statement::Statement;

use std::cell::RefCell;
use std::rc::Rc;

#[cfg(test)]
#[path = "./tests/interpreter_tests.rs"]
mod tests;

pub struct Interpreter {
    pub environment: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Rc::new(RefCell::new(Environment::new())),
        }
    }

    pub fn interpret_statements(&mut self, statements: Vec<Statement>) -> Result<(), String> {
        for statement in statements {
            match statement {
                Statement::Expression { expression } => {
                    expression.evaluate(&mut self.environment.borrow_mut())?;
                }
                Statement::Print { expression } => {
                    let result = expression.evaluate(&mut self.environment.borrow_mut())?;
                    println!("{}", result);
                }
                Statement::Variable { token, initializer } => {
                    let nil = Expression::Literal {
                        value: LiteralValue::Nil,
                    };

                    let value = if initializer != nil {
                        initializer.evaluate(&mut self.environment.borrow_mut())?
                    } else {
                        LiteralValue::Nil
                    };
                    self.environment.borrow_mut().define(token.lexeme, value);
                }
                Statement::Block { statements } => {
                    let mut new_environment = Environment::new();
                    new_environment.enclosing = Some(self.environment.clone());

                    let old_environment = self.environment.clone();
                    self.environment = Rc::new(RefCell::new(new_environment));
                    let block_result = self.interpret_statements(statements);
                    self.environment = old_environment;

                    block_result?
                }
                Statement::If {
                    condition,
                    then_branch,
                    else_branch,
                } => {
                    let condition_value = condition.evaluate(&mut self.environment.borrow_mut())?;

                    if bool::from(condition_value) {
                        self.interpret_statements(vec![*then_branch])?;
                    } else if let Some(else_branch_value) = else_branch {
                        self.interpret_statements(vec![*else_branch_value])?;
                    }
                }
                Statement::While { condition, body } => {
                    while bool::from(condition.evaluate(&mut self.environment.borrow_mut())?) {
                        self.interpret_statements(vec![*body.clone()])?; // fix here??
                    }
                }
            };
        }

        Ok(())
    }
}
