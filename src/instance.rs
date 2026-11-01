use std::{collections::HashMap, fmt::Display};

use crate::{
    class::LoxClass,
    runtime_error::{RealError, RuntimeError},
    runtime_value::RuntimeValue,
    token::Token,
};

#[derive(Clone, Debug)]
pub struct LoxInstance {
    class: LoxClass,
    fields: HashMap<String, RuntimeValue>,
}

impl LoxInstance {
    pub fn new(class: LoxClass) -> Self {
        Self {
            class,
            fields: HashMap::new(),
        }
    }
    pub fn get(&self, name: &Token) -> Result<RuntimeValue, RuntimeError> {
        if let Some(value) = self.fields.get(&name.lexeme) {
            return Ok(value.clone());
        }
        Err(RuntimeError::Error(RealError {
            token: name.clone(),
            msg: format!("Undefined property '{}'.", name.lexeme),
        }))
    }
    pub fn set(&mut self, name: &Token, value: RuntimeValue) {
        self.fields.insert(name.lexeme.clone(), value);
    }
}

impl Display for LoxInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} instance", self.class.name)
    }
}
