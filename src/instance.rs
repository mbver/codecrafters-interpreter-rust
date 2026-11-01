use std::{cell::RefCell, collections::HashMap, fmt::Display, rc::Rc};

use crate::{
    callable::{LoxCallable, LoxFunction},
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
    pub fn get(
        instance: Rc<RefCell<Self>>,
        name: &Token,
    ) -> Result<RuntimeValue, RuntimeError> {
        let instance_ref = instance.borrow();
        if let Some(value) = instance_ref.fields.get(&name.lexeme) {
            return Ok(value.clone());
        }
        if let Some(method) = instance_ref.class.find_method(&name.lexeme) {
            let method = method.bind(instance.clone());
            return Ok(RuntimeValue::Callable(LoxCallable::LoxFunction(
                method,
            )));
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
