use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    runtime_error::{RealError, RuntimeError},
    runtime_value::RuntimeValue,
    token::Token,
};

pub type EnvRef = Rc<RefCell<Environment>>;

pub fn ancestor(environment: EnvRef, distance: usize) -> EnvRef {
    let mut current = environment;
    for _ in 0..distance {
        let enclosing = current.borrow().enclosing.clone();
        current = enclosing.expect("Missing enclosing environment");
    }
    current
}
pub fn get_at(
    environment: EnvRef,
    distance: usize,
    name: &Token,
) -> Result<RuntimeValue, RuntimeError> {
    ancestor(environment, distance).borrow().get_local(name)
}
pub fn assign_at(
    environment: EnvRef,
    distance: usize,
    name: &Token,
    value: RuntimeValue,
) -> Result<(), RuntimeError> {
    ancestor(environment, distance)
        .borrow_mut()
        .assign_local(name, value)
}
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
    // TODO: only use get_local because now variable is resolved?
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
    pub fn get_local(
        &self,
        name: &Token,
    ) -> Result<RuntimeValue, RuntimeError> {
        let Some(value) = self.values.get(&name.lexeme) else {
            return Err(RuntimeError::Error(RealError::undefined_variable(
                name,
            )));
        };
        Ok(value.clone())
    }
    pub fn assign_local(
        &mut self,
        name: &Token,
        value: RuntimeValue,
    ) -> Result<(), RuntimeError> {
        let Some(existing) = self.values.get_mut(&name.lexeme) else {
            return Err(RuntimeError::Error(RealError::undefined_variable(
                name,
            )));
        };
        *existing = value;
        Ok(())
    }
    // TODO: only use assign_local because variable is resolved?
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
