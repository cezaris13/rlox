use crate::expr::{Expression, LiteralValue};

pub struct Interpreter {
    // global state
    //
}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn interpret(&mut self, expression: Expression) -> Result<LiteralValue, String> {
        expression.evaluate()
    }
}
