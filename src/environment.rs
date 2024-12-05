use crate::expression::LiteralValue;

use std::collections::HashMap;
use std::rc::Rc;

#[cfg(test)]
#[path = "./tests/environment_tests.rs"]
mod tests;

pub struct Environment {
    values: HashMap<String, LiteralValue>,
    pub enclosing: Option<Rc<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        return Environment {
            values: HashMap::new(),
            enclosing: None,
        };
    }

    pub fn define(&mut self, name: String, value: LiteralValue) {
        self.values.insert(name, value);
    }

    pub fn assign(&mut self, name: String, value: LiteralValue) -> Result<(), String> {
        match self.values.get_key_value(&name) {
            Some(_) => {
                self.define(name, value);
                return Ok(());
            }

            None => match &self.enclosing {
                Some(env) => env.borrow_mut().assign(name, value),
                _ => Err(format!("Variable does not exist {}", name)),
            },
        }
    }

    pub fn get(&self, name: &str) -> Result<LiteralValue, String> {
        match self.values.get_key_value(name) {
            Some((_, value)) => Ok(value.clone()),
            None => match &self.enclosing {
                Some(enclosing_environment) => enclosing_environment.borrow().get(name),
                _ => Err(format!("Undefined variable {}", name)),
            },
        }
    }
}
