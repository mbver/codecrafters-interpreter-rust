use crate::{
    expr::Expr,
    token::{Literal, Token},
};

#[derive(Debug, Clone)]
pub struct Function {
    pub name: Token,
    pub params: Vec<Token>,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Block(Vec<Stmt>),
    Class {
        name: Token,
        superclass: Option<Expr>, // parser will ensure it is variable
        methods: Vec<Stmt>,       // parser will ensure it is function
    },
    Expression(Expr),
    Function(Function),
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Box<Stmt>,
    },
    Print(Expr),
    Return {
        keyword: Token,
        value: Option<Expr>,
    },
    Variable {
        name: Token,
        initializer: Expr,
    },
    While {
        condition: Expr,
        body: Box<Stmt>,
    },
}

pub fn null_stmt() -> Stmt {
    Stmt::Expression(Expr::Literal {
        value: Literal::NULL,
    })
}
