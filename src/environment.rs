use crate::expr::LiteralValue;
use crate::token::Token;

use std::collections::HashMap;

pub struct Environment {
    values: HashMap<String, LiteralValue>,
}

impl Environment {
    pub fn new() -> Self {
        return Environment {
            values: HashMap::new(),
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
            None => Err(format!("Variable does not exist {}", name)),
        }
    }

    pub fn get(&self, name: &Token) -> Result<LiteralValue, String> {
        match self.values.get_key_value(&name.lexeme) {
            Some((_, value)) => Ok(value.clone()),
            None => Err(format!("Undefined variable {}", name.lexeme)),
        }
    }
}
