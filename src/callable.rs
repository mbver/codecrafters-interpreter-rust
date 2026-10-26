use std::fmt::Display;

use crate::{
    runtime_error::RuntimeError,
    runtime_value::RuntimeValue,
    token::{Literal, Token},
};

#[derive(Clone)]
pub enum LoxCallable {
    NativeFunction(NativeFunction),
    LoxFunction(LoxFunction),
}

#[derive(Clone)]
pub struct NativeFunction {
    pub arity: usize,
    pub function: fn(&[RuntimeValue]) -> Result<RuntimeValue, RuntimeError>,
}
#[derive(Clone)]
pub struct LoxFunction {
    pub callee: Literal,
    pub args: Vec<Literal>,
    pub paren: Token,
}

impl LoxCallable {
    pub fn arity(&self) -> usize {
        match self {
            LoxCallable::NativeFunction(f) => f.arity,
            LoxCallable::LoxFunction(f) => f.args.len(),
        }
    }
}

impl Display for LoxCallable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoxCallable::LoxFunction(_) => write!(f, "<lox function>"),
            LoxCallable::NativeFunction(_) => write!(f, "<native function>"),
        }
    }
}
