use std::fmt::Display;

use crate::{
    environment::EnvRef,
    runtime_error::RuntimeError,
    runtime_value::RuntimeValue,
    stmt::{Function, Stmt},
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
    pub declaration: Function,
    pub closure: EnvRef,
}

impl LoxCallable {
    pub fn arity(&self) -> usize {
        match self {
            LoxCallable::NativeFunction(f) => f.arity,
            LoxCallable::LoxFunction(f) => f.declaration.params.len(),
        }
    }
}

impl Display for LoxCallable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoxCallable::LoxFunction(fun) => {
                write!(f, "<fn {}>", fun.declaration.name.lexeme)
            }
            LoxCallable::NativeFunction(_) => write!(f, "<native function>"),
        }
    }
}
