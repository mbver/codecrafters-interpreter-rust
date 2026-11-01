use std::fmt::Display;

#[derive(Clone, Debug)]
pub struct LoxClass {
    pub name: String,
}

impl Display for LoxClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl LoxClass {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
