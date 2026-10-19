use crate::{
    environment::Environment,
    expr::Expr,
    runtime_error::RuntimeError,
    stmt::Stmt,
    token::{Literal, Token, TokenType},
};

pub struct Interpreter {
    pub has_error: bool,
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            has_error: false,
            environment: Environment::new(),
        }
    }
    pub fn evaluate(&mut self, expr: &Expr) -> Result<Literal, RuntimeError> {
        match expr {
            Expr::Literal { value } => Ok(value.clone()),
            Expr::Grouping { expr } => self.evaluate(expr),
            Expr::Unary { operator, right } => {
                let right = self.evaluate(right)?;
                match operator.token_type {
                    TokenType::MINUS => {
                        let Literal::Number(v) = right else {
                            // print error message here
                            return Err(RuntimeError::new(
                                operator.clone(),
                                "Operand must be a number.",
                            ));
                        };
                        Ok(Literal::Number(-v))
                    }
                    TokenType::BANG => Ok(Literal::Boolean(!is_truthy(&right))),
                    _ => unreachable!(),
                }
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left = self.evaluate(left)?;
                let right = self.evaluate(right)?;
                match (left, right) {
                    (Literal::STRING(left), Literal::STRING(right))
                        if operator.token_type == TokenType::PLUS =>
                    {
                        Ok(Literal::STRING(left + &right))
                    }
                    (left, right)
                        if operator.token_type == TokenType::BANG_EQUAL =>
                    {
                        Ok(Literal::Boolean(left != right))
                    }
                    (left, right)
                        if operator.token_type == TokenType::EQUAL_EQUAL =>
                    {
                        Ok(Literal::Boolean(left == right))
                    }
                    (Literal::Number(left), Literal::Number(right)) => {
                        match operator.token_type {
                            TokenType::PLUS => {
                                Ok(Literal::Number(left + right))
                            }
                            TokenType::MINUS => {
                                Ok(Literal::Number(left - right))
                            }
                            TokenType::SLASH => {
                                if right == 0.0 {
                                    return Err(RuntimeError::new(
                                        operator.clone(),
                                        "Division by zero.",
                                    ));
                                }
                                Ok(Literal::Number(left / right))
                            }
                            TokenType::STAR => {
                                Ok(Literal::Number(left * right))
                            }
                            TokenType::GREATER => {
                                Ok(Literal::Boolean(left > right))
                            }
                            TokenType::GREATER_EQUAL => {
                                Ok(Literal::Boolean(left >= right))
                            }
                            TokenType::LESS => {
                                Ok(Literal::Boolean(left < right))
                            }
                            TokenType::LESS_EQUAL => {
                                Ok(Literal::Boolean(left <= right))
                            }
                            _ => unreachable!(),
                        }
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
            Expr::Variable { name } => self.environment.get(name),
            Expr::Assign { name, value } => {
                let value = self.evaluate(value)?;
                self.environment.assign(name, value.clone())?;
                Ok(value)
            }
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
                print_value(&value);
                Ok(())
            }
            Stmt::Variable { name, initializer } => {
                let value = self.evaluate(initializer)?;
                self.environment.define(name.lexeme.clone(), value);
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

fn is_truthy(value: &Literal) -> bool {
    match value {
        Literal::NULL => false,
        Literal::Boolean(v) => *v,
        _ => true,
    }
}

pub fn print_value(value: &Literal) {
    match value {
        Literal::NULL => println!("nil"),
        Literal::Number(v) => {
            let mut text = v.to_string();
            if text.ends_with(".0") {
                text.truncate(text.len() - 2);
            }
            println!("{}", text);
        }
        _ => println!("{}", value),
    }
}
