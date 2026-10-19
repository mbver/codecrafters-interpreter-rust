use crate::token::Token;

pub struct RuntimeError {
    pub token: Token,
    pub msg: String,
}

impl RuntimeError {
    pub fn new(token: Token, msg: &str) -> Self {
        Self {
            token,
            msg: msg.to_string(),
        }
    }
    pub fn undefined_variable(name: &Token) -> Self {
        Self {
            token: name.clone(),
            msg: format!("Undefined variable '{}'.", &name.lexeme),
        }
    }
}
