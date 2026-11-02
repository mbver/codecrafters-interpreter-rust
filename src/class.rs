use std::{collections::HashMap, fmt::Display};

use crate::callable::LoxFunction;

#[derive(Clone, Debug)]
pub struct LoxClass {
    pub name: String,
    pub superclass: Option<Box<LoxClass>>,
    pub methods: HashMap<String, LoxFunction>,
}

impl Display for LoxClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl LoxClass {
    pub fn new(
        name: String,
        superclass: Option<Box<LoxClass>>,
        methods: HashMap<String, LoxFunction>,
    ) -> Self {
        Self {
            name,
            superclass,
            methods,
        }
    }
    pub fn find_method(&self, name: &str) -> Option<LoxFunction> {
        self.methods.get(name).cloned().or_else(|| {
            self.superclass
                .as_ref()
                .and_then(|superclass| superclass.find_method(name))
        })
    }
}
