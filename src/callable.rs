use std::{
    cell::RefCell,
    fmt::{Debug, Display},
    rc::Rc,
};

use crate::{
    class::LoxClass,
    environment::{EnvRef, Environment},
    instance::LoxInstance,
    runtime_error::RuntimeError,
    runtime_value::RuntimeValue,
    stmt::Function,
};

#[derive(Clone, Debug)]
pub enum LoxCallable {
    NativeFunction(NativeFunction),
    LoxFunction(LoxFunction),
    LoxClass(LoxClass),
}

#[derive(Clone, Debug)]
pub struct NativeFunction {
    pub arity: usize,
    pub function: fn(&[RuntimeValue]) -> Result<RuntimeValue, RuntimeError>,
}
#[derive(Clone)]
pub struct LoxFunction {
    pub declaration: Function,
    pub closure: EnvRef,
}

impl LoxFunction {
    pub fn bind(self, instance: Rc<RefCell<LoxInstance>>) -> Self {
        let mut environment = Environment::new_enclosed(self.closure);
        environment
            .define("this".to_string(), RuntimeValue::Instance(instance));
        LoxFunction {
            declaration: self.declaration,
            closure: Rc::new(RefCell::new(environment)),
        }
    }
}
impl Debug for LoxFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<function: {}>", self.declaration.name.lexeme)
    }
}
impl LoxCallable {
    pub fn arity(&self) -> usize {
        match self {
            LoxCallable::NativeFunction(f) => f.arity,
            LoxCallable::LoxFunction(f) => f.declaration.params.len(),
            LoxCallable::LoxClass(_) => 0,
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
            LoxCallable::LoxClass(c) => write!(f, "<class {}>", c),
        }
    }
}
