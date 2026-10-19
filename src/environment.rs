use std::collections::HashMap;

use crate::{
    runtime_error::RuntimeError,
    token::{Literal, Token},
};

pub struct Environment {
    values: HashMap<String, Literal>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }
    pub fn define(&mut self, name: String, value: Literal) {
        self.values.insert(name, value);
    }
    pub fn get(&self, name: &Token) -> Result<Literal, RuntimeError> {
        self.values
            .get(&name.lexeme)
            .cloned()
            .ok_or_else(|| RuntimeError::undefined_variable(name))
    }
    pub fn assign(
        &mut self,
        name: &Token,
        value: Literal,
    ) -> Result<(), RuntimeError> {
        let Some(existing) = self.values.get_mut(&name.lexeme) else {
            return Err(RuntimeError::undefined_variable(name));
        };
        *existing = value;
        Ok(())
    }
}
