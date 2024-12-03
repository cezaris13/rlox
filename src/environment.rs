use crate::expr::LiteralValue;
use crate::token::Token;

use std::collections::HashMap;

pub struct Environment {
    values: HashMap<String, LiteralValue>,
    enclosing: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        return Environment {
            values: HashMap::new(),
            enclosing: None,
        };
    }

    pub fn new_enclosing(enclosing: Environment) -> Self {
        return Environment {
            values: HashMap::new(),
            enclosing: Some(Box::new(enclosing)),
        };
    }

    pub fn define(&mut self, name: &String, value: &LiteralValue) {
        self.values.insert(name.clone(), value.clone());
    }

    pub fn assign(&mut self, name: &String, value: &LiteralValue) -> Result<(), String> {
        match self.values.get_key_value(name) {
            Some(_) => {
                self.define(name, value);
                return Ok(());
            }

            None => match &self.enclosing {
                Some(eclosing_environment) => self.assign(name, value),
                _ => Err(format!("Variable does not exist {}", name)),
            },
        }
    }

    pub fn get(&self, name: &Token) -> Result<LiteralValue, String> {
        match self.values.get_key_value(&name.lexeme) {
            Some((_, value)) => Ok(value.clone()),
            None => match &self.enclosing {
                Some(enclosing_environent) => enclosing_environent.get(name),
                _ => Err(format!("Undefined variable {}", name.lexeme)),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_init() {
        let environment = Environment::new();
    }
}
mod tests {
    use super::*;

    #[test]
    fn try_init() {
        let environment = Environment::new();
    }
}
