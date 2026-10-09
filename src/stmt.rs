use crate::{expr::Expr, token::Token};

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
