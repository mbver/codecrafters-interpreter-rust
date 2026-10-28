use crate::{runtime_value::RuntimeValue, token::Token};

pub enum RuntimeError {
    Error(RealError),
    Return(RuntimeValue),
}

pub struct RealError {
    pub token: Token,
    pub msg: String,
}

impl RealError {
    pub fn new(token: Token, msg: String) -> Self {
        Self { token, msg }
    }
    pub fn undefined_variable(name: &Token) -> Self {
        Self {
            token: name.clone(),
            msg: format!("Undefined variable '{}'.", &name.lexeme),
        }
    }
    pub fn wrong_type(name: &Token, expected_type: &str) -> Self {
        Self {
            token: name.clone(),
            msg: format!(
                "Wrong type: '{} is not {}.",
                &name.lexeme, expected_type
            ),
        }
    }
}
