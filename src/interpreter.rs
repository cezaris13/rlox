use crate::environment::Environment;
use crate::expression::Expression;
use crate::expression_literal_value::LiteralValue;
use crate::statement::Statement;

use std::cell::RefCell;
use std::rc::Rc;
use std::time::SystemTime;

#[cfg(test)]
#[path = "./tests/interpreter_tests.rs"]
mod tests;

pub struct Interpreter {
    pub environment: Rc<RefCell<Environment>>,
}

fn clock_impl(
    _env: Rc<RefCell<Environment>>,
    _args: &Vec<LiteralValue>,
) -> Result<LiteralValue, String> {
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Could not get time")
        .as_secs();

    Ok(LiteralValue::IntValue(now as i64))
}

impl Interpreter {
    pub fn new() -> Self {
        let mut environment = Environment::new();
        environment.define(
            String::from("clock"),
            LiteralValue::Callable {
                name: String::from("clock"),
                arity: 0,
                fun: Rc::new(clock_impl),
            },
        );

        Self {
            environment: Rc::new(RefCell::new(environment)),
        }
    }

    fn for_closure(parent: Rc<RefCell<Environment>>) -> Self {
        let environment = Rc::new(RefCell::new(Environment::new()));
        environment.borrow_mut().enclosing = Some(parent.clone());

        Self { environment }
    }

    pub fn interpret_statements(&mut self, statements: Vec<Statement>) -> Result<(), String> {
        for statement in statements {
            match statement {
                Statement::Expression { expression } => {
                    expression.evaluate(self.environment.clone())?;
                }
                Statement::Print { expression } => {
                    let result = expression.evaluate(self.environment.clone())?;
                    println!("{}", result);
                }
                Statement::Variable { token, initializer } => {
                    let value = match initializer {
                        Expression::Literal {
                            value: LiteralValue::Nil,
                        } => LiteralValue::Nil,
                        _ => initializer.evaluate(self.environment.clone())?,
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
                    let condition_value = condition.evaluate(self.environment.clone())?;

                    if bool::from(condition_value) {
                        self.interpret_statements(vec![*then_branch])?;
                    } else if let Some(else_branch_value) = else_branch {
                        self.interpret_statements(vec![*else_branch_value])?;
                    }
                }
                Statement::While { condition, body } => {
                    while bool::from(condition.evaluate(self.environment.clone())?) {
                        self.interpret_statements(vec![*body.clone()])?; // fix here??
                    }
                }
                Statement::Function {
                    name,
                    parameters,
                    body,
                } => {
                    let arity = parameters.len();

                    let closure = move |parent_environment: Rc<RefCell<Environment>>,
                                        arguments: &Vec<LiteralValue>|
                          -> Result<LiteralValue, String> {
                        let mut closure_interpreter = Interpreter::for_closure(parent_environment);

                        for (i, argument) in arguments.iter().enumerate() {
                            closure_interpreter
                                .environment
                                .borrow_mut()
                                .define(String::from(&parameters[i].lexeme), argument.clone());
                        }

                        closure_interpreter.interpret_statements(body.clone())?;

                        Ok(LiteralValue::Nil)
                    };

                    self.environment.borrow_mut().define(
                        String::from(&name.lexeme),
                        LiteralValue::Callable {
                            name: String::from(&name.lexeme),
                            arity,
                            fun: Rc::new(closure),
                        },
                    );
                }
            };
        }

        Ok(())
    }
}
