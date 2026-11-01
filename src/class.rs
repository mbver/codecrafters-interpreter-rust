use std::{collections::HashMap, fmt::Display};

use crate::callable::LoxFunction;

#[derive(Clone, Debug)]
pub struct LoxClass {
    pub name: String,
    pub methods: HashMap<String, LoxFunction>,
}

impl Display for LoxClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl LoxClass {
    pub fn new(name: String, methods: HashMap<String, LoxFunction>) -> Self {
        Self { name, methods }
    }
    pub fn find_method(&self, name: &str) -> Option<LoxFunction> {
        self.methods.get(name).cloned()
    }
}
