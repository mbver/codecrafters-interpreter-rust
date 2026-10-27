use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    callable::LoxCallable,
    runtime_error::{RealError, RuntimeError},
    runtime_value::RuntimeValue,
    token::{Literal, Token},
};

pub type EnvRef = Rc<RefCell<Environment>>;

pub struct Environment {
    values: HashMap<String, RuntimeValue>,
    enclosing: Option<EnvRef>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            enclosing: None,
        }
    }
    pub fn new_enclosed(environment: EnvRef) -> Self {
        Self {
            values: HashMap::new(),
            enclosing: Some(environment),
        }
    }
    pub fn define(&mut self, name: String, value: RuntimeValue) {
        self.values.insert(name, value);
    }
    pub fn get(&self, name: &Token) -> Result<RuntimeValue, RuntimeError> {
        if let Some(value) = self.values.get(&name.lexeme) {
            return Ok(value.clone());
        }
        let Some(enclosing) = &self.enclosing else {
            return Err(RuntimeError::Error(RealError::undefined_variable(
                name,
            )));
        };
        enclosing.borrow().get(name)
    }

    pub fn assign(
        &mut self,
        name: &Token,
        value: RuntimeValue,
    ) -> Result<(), RuntimeError> {
        if let Some(existing) = self.values.get_mut(&name.lexeme) {
            *existing = value;
            return Ok(());
        };
        let Some(enclosing) = &mut self.enclosing else {
            return Err(RuntimeError::Error(RealError::undefined_variable(
                name,
            )));
        };
        enclosing.borrow_mut().assign(name, value)
    }
}
