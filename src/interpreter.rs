use std::{
    cell::RefCell,
    rc::Rc,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{
    callable::{LoxCallable, LoxFunction, NativeFunction},
    environment::{EnvRef, Environment},
    expr::Expr,
    runtime_error::RuntimeError,
    runtime_value::RuntimeValue,
    stmt::Stmt,
    token::{Literal, TokenType},
};

pub struct Interpreter {
    pub has_error: bool,
    globals: EnvRef,
    environment: EnvRef,
}

impl Interpreter {
    pub fn new() -> Self {
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
        }
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
                            return Err(RuntimeError::new(
                                operator.clone(),
                                "Operand must be a number.",
                            ));
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
                    RuntimeError::wrong_type(operator, "literal")
                })?;
                let right =
                    self.evaluate(right)?.to_literal().map_err(|e| {
                        RuntimeError::wrong_type(operator, "literal")
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
                                    return Err(RuntimeError::new(
                                        operator.clone(),
                                        "Division by zero.",
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
                        Err(RuntimeError::new(operator.clone(), msg))
                    }
                }
            }
            // TODO: this can evaluate to a Callable or Literal
            Expr::Variable { name } => self.environment.borrow().get(name),
            Expr::Assign { name, value } => {
                let value = self.evaluate(value)?;
                self.environment.borrow_mut().assign(name, value.clone())?;
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
            //             Expr::Call {
            //                 callee,
            //                 paren,
            //                 args,
            //             } => {
            //                 let callee = self.evaluate(callee)?;
            //                 let mut arguments = Vec::new();
            //                 for arg in args {
            //                     arguments.push(self.evaluate(arg)?);
            //                 }
            //                 // TODO: call it directly. Callable is stored in environment
            //                 let function = Callable::LoxFunction(LoxFunction {
            //                     callee,
            //                     args: arguments,
            //                     paren: paren.clone(),
            //                 });
            //                 // TODO: execute it in interpreter
            //
            //                 Ok(Literal::NULL)
            //             }
            _ => todo!(),
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
                let enclosed =
                    Environment::new_enclosed(Rc::clone(&self.environment));
                let previous = std::mem::replace(
                    &mut self.environment,
                    Rc::new(RefCell::new(enclosed)),
                );
                let result = stmts.iter().try_for_each(|s| self.execute(s));
                self.environment = previous;
                result
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
            _ => todo!(),
        }
    }
    pub fn interpret(&mut self, stmts: &[Stmt]) {
        for stmt in stmts {
            if let Err(e) = self.execute(stmt) {
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
    }
}
