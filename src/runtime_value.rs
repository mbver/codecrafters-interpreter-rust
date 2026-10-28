use crate::{callable::LoxCallable, token::Literal};

pub struct WrongTypeError;
#[derive(Clone)]
pub enum RuntimeValue {
    Literal(Literal),
    Callable(LoxCallable),
}

impl RuntimeValue {
    pub fn to_literal(self) -> Result<Literal, WrongTypeError> {
        match self {
            RuntimeValue::Literal(v) => Ok(v),
            _ => Err(WrongTypeError),
        }
    }
}
