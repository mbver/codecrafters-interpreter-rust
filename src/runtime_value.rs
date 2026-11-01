use std::{cell::RefCell, rc::Rc};

use crate::{
    callable::LoxCallable, class::LoxClass, instance::LoxInstance,
    token::Literal,
};

pub struct WrongTypeError;
#[derive(Clone, Debug)]
pub enum RuntimeValue {
    Literal(Literal),
    Callable(LoxCallable),
    Class(LoxClass),
    Instance(Rc<RefCell<LoxInstance>>),
}

impl RuntimeValue {
    pub fn to_literal(self) -> Result<Literal, WrongTypeError> {
        match self {
            RuntimeValue::Literal(v) => Ok(v),
            _ => Err(WrongTypeError),
        }
    }
}
