use crate::{
    expr::Expr,
    token::{Literal, Token},
};

#[derive(Debug)]
pub enum Stmt {
    Block(Vec<Stmt>),
    Class {
        name: Token,
        superclass: Expr,   // parser will ensure it is variable
        methods: Vec<Stmt>, // parser will ensure it is function
    },
    Expression(Expr),
    Function {
        name: Token,
        params: Vec<Token>,
        body: Vec<Stmt>,
    },
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Box<Stmt>,
    },
    Print(Expr),
    Return {
        keyword: Token,
        value: Expr,
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
