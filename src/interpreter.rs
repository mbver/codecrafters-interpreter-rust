use std::{
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{
    callable::{LoxCallable, LoxFunction, NativeFunction},
    class::LoxClass,
    environment::{EnvRef, Environment, assign_at, get_at},
    expr::Expr,
    instance::LoxInstance,
    runtime_error::{RealError, RuntimeError},
    runtime_value::RuntimeValue,
    stmt::Stmt,
    token::{Literal, Token, TokenType},
};

pub struct Interpreter {
    pub has_error: bool,
    globals: EnvRef,
    environment: EnvRef,
    locals: HashMap<usize, usize>,
}

impl Interpreter {
    pub fn new(locals: HashMap<usize, usize>) -> Self {
        let globals = Rc::new(RefCell::new(Environment::new()));
        globals.borrow_mut().define(
            "clock".to_string(),
            RuntimeValue::Callable(LoxCallable::NativeFunction(
                NativeFunction {
                    arity: 0,
                    function: |_| {
                        let seconds = SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap()
                            .as_secs_f64();
                        Ok(RuntimeValue::Literal(Literal::Number(seconds)))
                    },
                },
            )),
        );
        let environment = globals.clone();
        Self {
            has_error: false,
            globals,
            environment,
            locals: locals,
        }
    }
    pub fn lookup_variable(
        &self,
        name: &Token,
    ) -> Result<RuntimeValue, RuntimeError> {
        let Some(distance) = self.locals.get(&name.offset) else {
            return self.globals.borrow().get_local(name);
        };
        get_at(Rc::clone(&self.environment), *distance, name)
    }

    pub fn evaluate(
        &mut self,
        expr: &Expr,
    ) -> Result<RuntimeValue, RuntimeError> {
        match expr {
            Expr::Literal { value } => Ok(RuntimeValue::Literal(value.clone())),
            Expr::Grouping { expr } => self.evaluate(expr),
            Expr::Unary { operator, right } => {
                let right = self.evaluate(right)?;
                match operator.token_type {
                    TokenType::MINUS => {
                        let RuntimeValue::Literal(Literal::Number(v)) = right
                        else {
                            // print error message here
                            return Err(RuntimeError::Error(RealError::new(
                                operator.clone(),
                                "Operand must be a number.".to_string(),
                            )));
                        };
                        Ok(RuntimeValue::Literal(Literal::Number(-v)))
                    }
                    TokenType::BANG => Ok(RuntimeValue::Literal(
                        Literal::Boolean(!is_truthy(&right)),
                    )),
                    _ => unreachable!(),
                }
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left = self.evaluate(left)?.to_literal().map_err(|e| {
                    RuntimeError::Error(RealError::wrong_type(
                        operator, "literal",
                    ))
                })?;
                let right =
                    self.evaluate(right)?.to_literal().map_err(|e| {
                        RuntimeError::Error(RealError::wrong_type(
                            operator, "literal",
                        ))
                    })?;
                match (left, right) {
                    (Literal::STRING(left), Literal::STRING(right))
                        if operator.token_type == TokenType::PLUS =>
                    {
                        Ok(RuntimeValue::Literal(Literal::STRING(
                            left + &right,
                        )))
                    }
                    (left, right)
                        if operator.token_type == TokenType::BANG_EQUAL =>
                    {
                        Ok(RuntimeValue::Literal(Literal::Boolean(
                            left != right,
                        )))
                    }
                    (left, right)
                        if operator.token_type == TokenType::EQUAL_EQUAL =>
                    {
                        Ok(RuntimeValue::Literal(Literal::Boolean(
                            left == right,
                        )))
                    }
                    (Literal::Number(left), Literal::Number(right)) => {
                        let value = match operator.token_type {
                            TokenType::PLUS => Literal::Number(left + right),
                            TokenType::MINUS => Literal::Number(left - right),
                            TokenType::SLASH => {
                                if right == 0.0 {
                                    return Err(RuntimeError::Error(
                                        RealError::new(
                                            operator.clone(),
                                            "Division by zero.".to_string(),
                                        ),
                                    ));
                                }
                                Literal::Number(left / right)
                            }
                            TokenType::STAR => Literal::Number(left * right),
                            TokenType::GREATER => {
                                Literal::Boolean(left > right)
                            }
                            TokenType::GREATER_EQUAL => {
                                Literal::Boolean(left >= right)
                            }
                            TokenType::LESS => Literal::Boolean(left < right),
                            TokenType::LESS_EQUAL => {
                                Literal::Boolean(left <= right)
                            }
                            _ => unreachable!(),
                        };
                        Ok(RuntimeValue::Literal(value))
                    }
                    _ => {
                        let msg = match operator.token_type {
                            TokenType::PLUS => {
                                "Operands must be two numbers or two strings."
                            }
                            _ => "Operands must be two numbers",
                        };
                        Err(RuntimeError::Error(RealError::new(
                            operator.clone(),
                            msg.to_string(),
                        )))
                    }
                }
            }
            Expr::Variable { name } => self.lookup_variable(name),
            Expr::Assign { name, value } => {
                let value = self.evaluate(value)?;
                if let Some(distance) = self.locals.get(&name.offset) {
                    assign_at(
                        Rc::clone(&self.environment),
                        *distance,
                        name,
                        value.clone(),
                    )?;
                } else {
                    self.globals
                        .borrow_mut()
                        .assign_local(name, value.clone())?;
                };
                Ok(value)
            }
            Expr::Logical {
                left,
                operator,
                right,
            } => {
                let left = self.evaluate(left)?;
                match operator.token_type {
                    TokenType::AND if !is_truthy(&left) => {
                        return Ok(left);
                    }
                    TokenType::OR if is_truthy(&left) => {
                        return Ok(left);
                    }
                    _ => {}
                };
                self.evaluate(right)
            }
            Expr::Call {
                callee,
                paren,
                args,
            } => {
                let callee = self.evaluate(callee)?;
                let callable = match callee {
                    RuntimeValue::Callable(callable) => callable,
                    RuntimeValue::Class(class) => LoxCallable::LoxClass(class),
                    _ => {
                        return Err(RuntimeError::Error(RealError::new(
                            paren.clone(),
                            "Can only call functions and classes".to_string(),
                        )));
                    }
                };

                let mut arguments = Vec::new();
                for arg in args {
                    arguments.push(self.evaluate(arg)?);
                }
                if arguments.len() != callable.arity() {
                    return Err(RuntimeError::Error(RealError::new(
                        paren.clone(),
                        format!(
                            "Expected {} arguments but got {}.",
                            callable.arity(),
                            arguments.len()
                        ),
                    )));
                }
                self.call(callable, arguments)
            }
            Expr::Get { object, name } => {
                let object = self.evaluate(object)?;
                let RuntimeValue::Instance(object) = object else {
                    return Err(RuntimeError::Error(RealError {
                        token: name.clone(),
                        msg: "Only instances have properties.".to_string(),
                    }));
                };
                LoxInstance::get(object, name)
            }
            Expr::Set {
                object,
                name,
                value,
            } => {
                let object = self.evaluate(object)?;
                let RuntimeValue::Instance(object) = object else {
                    return Err(RuntimeError::Error(RealError {
                        token: name.clone(),
                        msg: "Only instances have fields.".to_string(),
                    }));
                };
                let value = self.evaluate(value)?;
                object.borrow_mut().set(name, value.clone());
                Ok(value)
            }
            Expr::This { keyword } => self.lookup_variable(keyword),
            Expr::Super {
                keyword,
                method: method_name,
            } => {
                let Some(distance) = self.locals.get(&keyword.offset) else {
                    return Err(RuntimeError::Error(RealError {
                        token: keyword.clone(),
                        msg: "Superclass does not exist.".to_string(),
                    }));
                };
                let superclass =
                    get_at(Rc::clone(&self.environment), *distance, keyword)?;
                let RuntimeValue::Class(superclass) = superclass else {
                    return Err(RuntimeError::Error(RealError {
                        token: keyword.clone(),
                        msg: "Superclass must be a class.".to_string(),
                    }));
                };
                let this = Token::new(
                    TokenType::THIS,
                    "this".to_string(),
                    Literal::NULL,
                    0,
                    0,
                ); // TODO: this is temporary. how to use the real "this"
                let instance =
                    get_at(Rc::clone(&self.environment), distance - 1, &this)?;
                let RuntimeValue::Instance(instance) = instance else {
                    return Err(RuntimeError::Error(RealError {
                        token: keyword.clone(),
                        msg: "This must be an instance".to_string(),
                    }));
                };
                let method = superclass.find_method(&method_name.lexeme);
                let Some(method) = method else {
                    return Err(RuntimeError::Error(RealError {
                        token: method_name.clone(),
                        msg: format!(
                            "Undefined property '{}'.",
                            method_name.lexeme
                        ),
                    }));
                };
                Ok(RuntimeValue::Callable(LoxCallable::LoxFunction(
                    method.bind(instance),
                )))
            }
        }
    }
    fn call(
        &mut self,
        callable: LoxCallable,
        arguments: Vec<RuntimeValue>,
    ) -> Result<RuntimeValue, RuntimeError> {
        match callable {
            LoxCallable::LoxFunction(f) => {
                let mut environment =
                    Environment::new_enclosed(f.closure.clone());
                for (param, arg) in
                    f.declaration.params.iter().zip(arguments.iter())
                {
                    environment.define(param.lexeme.clone(), arg.clone());
                }
                let value = match self.execute_block(
                    &f.declaration.body,
                    Rc::new(RefCell::new(environment)),
                ) {
                    Ok(_) => RuntimeValue::Literal(Literal::NULL),
                    Err(e) => match e {
                        RuntimeError::Error(_) => return Err(e),
                        RuntimeError::Return(v) => {
                            v.unwrap_or(RuntimeValue::Literal(Literal::NULL))
                        }
                    },
                };
                if f.is_initializer {
                    let name = Token::new(
                        TokenType::IDENTIFIER,
                        "this".to_string(),
                        Literal::NULL,
                        f.declaration.name.line,
                        f.declaration.name.offset,
                    );
                    return get_at(f.closure, 0, &name);
                }
                Ok(value)
            }
            LoxCallable::NativeFunction(f) => (f.function)(&arguments),
            LoxCallable::LoxClass(c) => {
                let initializer = c.find_method("init");
                let instance = Rc::new(RefCell::new(LoxInstance::new(c)));
                if let Some(initializer) = initializer {
                    let initializer = initializer.bind(instance.clone());
                    let callable = LoxCallable::LoxFunction(initializer);
                    self.call(callable, arguments)?;
                }
                Ok(RuntimeValue::Instance(instance))
            }
        }
    }
    fn execute(&mut self, stmt: &Stmt) -> Result<(), RuntimeError> {
        match stmt {
            Stmt::Expression(expr) => {
                self.evaluate(expr)?;
                Ok(())
            }
            Stmt::Print(expr) => {
                let value = self.evaluate(expr)?;
                print_value(&value); // TODO: print callable
                Ok(())
            }
            Stmt::Variable { name, initializer } => {
                let value = self.evaluate(initializer)?;
                self.environment
                    .borrow_mut()
                    .define(name.lexeme.clone(), value);
                Ok(())
            }
            Stmt::Block(stmts) => {
                let environment =
                    Environment::new_enclosed(self.environment.clone());
                self.execute_block(stmts, Rc::new(RefCell::new(environment)))
            }
            Stmt::Class {
                name,
                superclass,
                methods,
            } => {
                let has_superclass = superclass.is_some();
                let superclass = match superclass {
                    Some(superclass) => {
                        let superclass = self.evaluate(superclass)?;
                        let RuntimeValue::Class(superclass) = superclass else {
                            return Err(RuntimeError::Error(RealError {
                                token: name.clone(),
                                msg: "Superclass must be a class.".to_string(),
                            }));
                        };
                        Some(superclass)
                    }
                    None => None,
                };
                // define the class in environment so later the methods' definition
                // can find where the class is
                self.environment.borrow_mut().define(
                    name.lexeme.clone(),
                    RuntimeValue::Literal(Literal::NULL),
                );
                if let Some(superclass) = &superclass {
                    self.environment = Rc::new(RefCell::new(
                        Environment::new_enclosed(self.environment.clone()),
                    ));
                    self.environment.borrow_mut().define(
                        "super".to_string(),
                        RuntimeValue::Class(superclass.clone()),
                    );
                }
                let mut method_map = HashMap::new();
                for method in methods {
                    let Stmt::Function(f) = method else {
                        return Err(RuntimeError::Error(RealError {
                            token: name.clone(),
                            msg: "Method is not a function statement."
                                .to_string(),
                        }));
                    };
                    let function = LoxFunction {
                        declaration: f.clone(),
                        closure: self.environment.clone(),
                        is_initializer: f.name.lexeme == "init",
                    };
                    method_map.insert(f.name.lexeme.clone(), function);
                }
                let class = LoxClass::new(
                    name.lexeme.clone(),
                    superclass.map(Box::new),
                    method_map,
                );
                if has_superclass {
                    assign_at(
                        self.environment.clone(),
                        1,
                        name,
                        RuntimeValue::Class(class),
                    )?;
                } else {
                    self.environment
                        .borrow_mut()
                        .assign_local(name, RuntimeValue::Class(class))?;
                }
                Ok(())
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                if is_truthy(&self.evaluate(condition)?) {
                    self.execute(then_branch)
                } else {
                    self.execute(else_branch)
                }
            }
            Stmt::While { condition, body } => {
                while is_truthy(&self.evaluate(condition)?) {
                    self.execute(body)?;
                }
                Ok(())
            }
            Stmt::Function(f) => {
                self.environment.borrow_mut().define(
                    f.name.lexeme.clone(),
                    RuntimeValue::Callable(LoxCallable::LoxFunction(
                        LoxFunction {
                            declaration: f.clone(),
                            closure: self.environment.clone(),
                            is_initializer: false,
                        },
                    )),
                );
                Ok(())
            }
            Stmt::Return { keyword, value } => {
                let value = match value {
                    Some(value) => Some(self.evaluate(value)?),
                    None => None,
                };
                Err(RuntimeError::Return(value))
            }
        }
    }
    fn execute_block(
        &mut self,
        stmts: &[Stmt],
        environment: EnvRef,
    ) -> Result<(), RuntimeError> {
        let previous = std::mem::replace(&mut self.environment, environment);
        let result = stmts.iter().try_for_each(|s| self.execute(s));
        self.environment = previous;
        result
    }
    pub fn interpret(&mut self, stmts: &[Stmt]) {
        for stmt in stmts {
            if let Err(RuntimeError::Error(e)) = self.execute(stmt) {
                eprintln!("{}\n[line {}]", e.msg, e.token.line);
                self.has_error = true;
                break;
            }
        }
    }
}

fn is_truthy(value: &RuntimeValue) -> bool {
    match value {
        RuntimeValue::Literal(Literal::NULL) => false,
        RuntimeValue::Literal(Literal::Boolean(v)) => *v,
        _ => true,
    }
}

pub fn print_value(value: &RuntimeValue) {
    match value {
        RuntimeValue::Callable(f) => {
            println!("{}", f)
        }
        RuntimeValue::Literal(value) => match value {
            Literal::NULL => println!("nil"),
            Literal::Number(v) => {
                let mut text = v.to_string();
                if text.ends_with(".0") {
                    text.truncate(text.len() - 2);
                }
                println!("{}", text);
            }
            _ => println!("{}", value),
        },
        RuntimeValue::Class(c) => {
            println!("{}", c)
        }
        RuntimeValue::Instance(a) => println!("{}", a.borrow()),
    }
}

#[cfg(test)]
mod tests {
    use crate::{parser::Parser, resolver::Resolver, scanner::Scanner};

    use super::*;

    #[test]
    fn function_with_arguments() {
        let content = "fun f1(a) {print a;}\nf1(59);";
        let mut scanner = Scanner::new(content);
        scanner.scan_tokens();
        assert!(!scanner.has_error());
        let mut parser = Parser::new(scanner.get_tokens());
        let stmts = parser.parse();
        assert!(!parser.has_error);
        let mut interpreter = Interpreter::new(HashMap::new());
        interpreter.interpret(&stmts);
    }
    #[test]
    fn resolve_variables() {
        let content = r#"
var variable = "global";

{
    fun f() {
        print variable;
    }

    f();

    var variable = "local";

    f();
}
"#;
        let mut scanner = Scanner::new(content);
        scanner.scan_tokens();
        assert!(!scanner.has_error());
        let mut parser = Parser::new(scanner.get_tokens());
        let stmts = parser.parse();
        assert!(!parser.has_error);
        let mut resolver = Resolver::new();
        let res = resolver.resolve_stmts(&stmts);
        let mut interpreter = Interpreter::new(resolver.into_locals());
        interpreter.interpret(&stmts);
    }

    #[test]
    fn higher_order_fun() {
        let content = r#"
fun returnArg(arg) {
    return arg;
}
fun returnFunCallWithArg(func, arg) {
    return returnArg(func)(arg);
}
fun printArg(arg) {
    print arg;
}
returnFunCallWithArg(printArg, "foo");
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
        let mut interpreter = Interpreter::new(resolver.into_locals());
        interpreter.interpret(&stmts);
        assert!(!interpreter.has_error);
    }

    #[test]
    fn already_a_variable_in_scope() {
        let content = r#"
var startTime = clock();
var lastCheck = startTime;
var running = true;
print "Starting timer for 0.2 seconds";
var startTime = clock();
while (running) {
    if (clock() > startTime + 0.2) {
    print "Timer ended";
    running = false;
    }
}
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
        let mut interpreter = Interpreter::new(resolver.into_locals());
        interpreter.interpret(&stmts);
        assert!(!interpreter.has_error);
    }
    #[test]
    fn instance() {
        let content = r#"class Spaceship {}
var falcon = Spaceship();
print falcon;"#;
        let mut scanner = Scanner::new(content);
        scanner.scan_tokens();
        assert!(!scanner.has_error());
        let mut parser = Parser::new(scanner.get_tokens());
        let stmts = parser.parse();
        assert!(!parser.has_error);
        let mut resolver = Resolver::new();
        let res = resolver.resolve_stmts(&stmts);
        assert!(res.is_ok());
        let mut interpreter = Interpreter::new(resolver.into_locals());
        interpreter.interpret(&stmts);
        assert!(!interpreter.has_error);
    }

    #[test]
    fn return_in_init() {
        let content = r#"
class Counter {
    init(startValue) {
        if (startValue < 0) {
            print "startValue can't be negative";
            this.count = 0;
        } else {
            this.count = startValue;
        }
    }
}
var instance = Counter(-19);
print instance.count;
print instance.init(19).count;"#;

        let mut scanner = Scanner::new(content);
        scanner.scan_tokens();
        assert!(!scanner.has_error());
        let mut parser = Parser::new(scanner.get_tokens());
        let stmts = parser.parse();
        assert!(!parser.has_error);
        let mut resolver = Resolver::new();
        let res = resolver.resolve_stmts(&stmts);
        assert!(res.is_ok());
        let mut interpreter = Interpreter::new(resolver.into_locals());
        interpreter.interpret(&stmts);
        assert!(!interpreter.has_error);
    }

    #[test]
    fn test_super() {
        let content = r#"
class Doughnut {
    cook() {
    print "Fry until golden brown.";
    }
}
class BostonCream < Doughnut {
    cook() {
    super.cook();
    }
}
BostonCream().cook();"#;

        let mut scanner = Scanner::new(content);
        scanner.scan_tokens();
        assert!(!scanner.has_error());
        let mut parser = Parser::new(scanner.get_tokens());
        let stmts = parser.parse();
        assert!(!parser.has_error);
        let mut resolver = Resolver::new();
        let res = resolver.resolve_stmts(&stmts);
        assert!(res.is_ok());
        let mut interpreter = Interpreter::new(resolver.into_locals());
        interpreter.interpret(&stmts);
        assert!(!interpreter.has_error);
    }
}
