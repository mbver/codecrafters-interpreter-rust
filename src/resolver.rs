use std::collections::HashMap;

use crate::{
    expr::Expr,
    stmt::{Function, Stmt},
    token::Token,
};

#[derive(PartialEq)]
enum FunctionType {
    NONE,
    FUNCTION,
    METHOD,
    INITIALIZER,
}

#[derive(PartialEq)]
enum ClassType {
    NONE,
    CLASS,
    SUBCLASS,
}
pub struct Resolver {
    locals: HashMap<usize, usize>,
    scopes: Vec<HashMap<String, bool>>,
    current_function: FunctionType,
    current_class: ClassType,
}

#[derive(Debug)]
pub struct ResolveError {
    pub token: Token,
    pub msg: String,
}
impl Resolver {
    pub fn new() -> Self {
        Self {
            locals: HashMap::new(),
            scopes: Vec::new(),
            current_function: FunctionType::NONE,
            current_class: ClassType::NONE,
        }
    }
    pub fn into_locals(self) -> HashMap<usize, usize> {
        self.locals
    }
    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }
    fn end_scope(&mut self) {
        self.scopes.pop();
    }
    fn declare(&mut self, name: &Token) -> Result<(), ResolveError> {
        let Some(scope) = self.scopes.last_mut() else {
            return Ok(());
        };
        if scope.contains_key(&name.lexeme) {
            return Err(ResolveError {
                token: name.clone(),
                msg: "Already a variable with this name in this scope."
                    .to_string(),
            });
        }
        scope.insert(name.lexeme.clone(), false);
        Ok(())
    }
    fn define(&mut self, name: &Token) {
        let Some(scope) = self.scopes.last_mut() else {
            return;
        };
        scope.insert(name.lexeme.clone(), true);
    }
    pub fn resolve_stmts(
        &mut self,
        stmts: &[Stmt],
    ) -> Result<(), ResolveError> {
        for stmt in stmts {
            self.resolve_stmt(stmt)?;
        }
        Ok(())
    }
    fn resolve_function(
        &mut self,
        params: &[Token],
        stmts: &[Stmt],
        function_type: FunctionType,
    ) -> Result<(), ResolveError> {
        let enclosing_function =
            std::mem::replace(&mut self.current_function, function_type);
        self.begin_scope();
        let res = (|| {
            for param in params {
                self.declare(param)?;
                self.define(param);
            }
            self.resolve_stmts(stmts)
        })();
        self.end_scope();
        self.current_function = enclosing_function;
        res?;
        Ok(())
    }
    fn resolve_stmt(&mut self, stmt: &Stmt) -> Result<(), ResolveError> {
        match stmt {
            Stmt::Block(stmts) => {
                self.begin_scope();
                let result = self.resolve_stmts(stmts);
                self.end_scope();
                result?;
            }
            Stmt::Class {
                name,
                superclass,
                methods,
            } => {
                let enclosing_class = std::mem::replace(
                    &mut self.current_class,
                    ClassType::CLASS,
                );
                self.declare(name)?;
                self.define(name);
                if let Some(superclass) = superclass {
                    if let Expr::Variable {
                        name: superclass_name,
                    } = superclass
                    {
                        if name.lexeme == superclass_name.lexeme {
                            return Err(ResolveError {
                                token: superclass_name.clone(),
                                msg: "A class can't inherit from itself."
                                    .to_string(),
                            });
                        }
                    }
                    self.current_class = ClassType::SUBCLASS;
                    self.resolve_expr(superclass)?;
                    self.begin_scope();
                    self.scopes
                        .last_mut()
                        .unwrap()
                        .insert("super".to_string(), true);
                }
                self.begin_scope();
                self.scopes
                    .last_mut()
                    .unwrap()
                    .insert("this".to_string(), true);
                let res = (|| {
                    for stmt in methods {
                        let Stmt::Function(Function { name, params, body }) =
                            stmt
                        else {
                            return Err(ResolveError {
                                token: name.clone(),
                                msg: "Method is not a function statement."
                                    .to_string(),
                            });
                        };
                        let function_type = if name.lexeme == "init" {
                            FunctionType::INITIALIZER
                        } else {
                            FunctionType::METHOD
                        };
                        self.resolve_function(params, body, function_type)?;
                    }
                    Ok(())
                })();
                self.end_scope();
                if superclass.is_some() {
                    self.end_scope();
                }
                self.current_class = enclosing_class;
                res?;
            }
            Stmt::Variable { name, initializer } => {
                self.declare(name)?;
                self.resolve_expr(initializer)?;
                self.define(name);
            }
            Stmt::Function(f) => {
                self.declare(&f.name)?;
                self.define(&f.name);
                self.resolve_function(
                    &f.params,
                    &f.body,
                    FunctionType::FUNCTION,
                )?;
            }
            Stmt::Expression(expr) => {
                self.resolve_expr(expr)?;
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.resolve_expr(condition)?;
                self.resolve_stmt(then_branch)?;
                self.resolve_stmt(else_branch)?;
            }
            Stmt::Print(expr) => {
                self.resolve_expr(expr)?;
            }
            Stmt::Return { keyword, value } => {
                match self.current_function {
                    FunctionType::NONE => {
                        return Err(ResolveError {
                            token: keyword.clone(),
                            msg: "Can't return from top-level code."
                                .to_string(),
                        });
                    }
                    FunctionType::INITIALIZER => {
                        if value.is_some() {
                            return Err(ResolveError {
                                token: keyword.clone(),
                                msg:
                                    "Can't return a value from an initializer."
                                        .to_string(),
                            });
                        }
                    }
                    _ => {}
                }
                if let Some(value) = value {
                    self.resolve_expr(value)?;
                }
            }
            Stmt::While { condition, body } => {
                self.resolve_expr(condition)?;
                self.resolve_stmt(body)?;
            }
        }
        Ok(())
    }
    fn resolve_local(&mut self, name: &Token) {
        for (depth, scope) in self.scopes.iter().rev().enumerate() {
            if scope.contains_key(&name.lexeme) {
                self.register_local(name.offset, depth);
                return;
            }
        }
    }
    fn register_local(&mut self, offset: usize, depth: usize) {
        self.locals.insert(offset, depth);
    }
    fn resolve_expr(&mut self, expr: &Expr) -> Result<(), ResolveError> {
        match expr {
            Expr::Assign { name, value } => {
                self.resolve_expr(value)?;
                self.resolve_local(name);
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                self.resolve_expr(left)?;
                self.resolve_expr(right)?;
            }
            Expr::Call {
                callee,
                paren,
                args,
            } => {
                self.resolve_expr(callee)?;
                for arg in args {
                    self.resolve_expr(arg)?;
                }
            }
            Expr::Get { object, name } => {
                self.resolve_expr(object)?;
            }
            Expr::Grouping { expr } => {
                self.resolve_expr(expr)?;
            }
            Expr::Literal { value } => {}
            Expr::Logical {
                left,
                operator,
                right,
            } => {
                self.resolve_expr(left)?;
                self.resolve_expr(right)?;
            }
            Expr::Set {
                object,
                name,
                value,
            } => {
                self.resolve_expr(value)?;
                self.resolve_expr(object)?;
            }
            Expr::Super { keyword, method } => {
                match self.current_class {
                    ClassType::NONE => {
                        return Err(ResolveError {
                            token: keyword.clone(),
                            msg: "Can't use 'super' outside of a class."
                                .to_string(),
                        });
                    }
                    ClassType::CLASS => {
                        return Err(ResolveError { token: keyword.clone(), msg: "Can't use 'super' in a class with no superclass.".to_string() });
                    }
                    _ => {}
                }
                self.resolve_local(keyword);
            }
            Expr::This { keyword } => {
                if self.current_class == ClassType::NONE {
                    return Err(ResolveError {
                        token: keyword.clone(),
                        msg: "Can't use 'this outside of a class.".to_string(),
                    });
                }
                self.resolve_local(keyword);
            }
            Expr::Unary { operator, right } => {
                self.resolve_expr(right)?;
            }
            Expr::Variable { name } => {
                if let Some(scope) = self.scopes.last() {
                    if let Some(false) = scope.get(&name.lexeme) {
                        return Err(ResolveError {
                            token: name.clone(),
                            msg:
                                "Can't read local variable in its own initializer."
                                    .to_string(),
                        });
                    }
                }
                self.resolve_local(name);
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{parser::Parser, scanner::Scanner};

    use super::*;
    #[test]
    fn allow_redeclaration_in_global_scope() {
        let content = r#"
// First declaration of variable 'a' in global
// scope
var a = "value";
// Redeclaring 'a' with its own value should be
// allowed in global scope
var a = a;
print a; // this should print "value"
"#;
        let mut scanner = Scanner::new(content);
        scanner.scan_tokens();
        assert!(!scanner.has_error());
        let mut parser = Parser::new(scanner.get_tokens());
        let stmts = parser.parse();
        assert!(!parser.has_error);
        let mut resolver = Resolver::new();
        let res = resolver.resolve_stmts(&stmts);
        assert!(res.is_ok());
    }
}
