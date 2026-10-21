use std::fs::OpenOptions;

use crate::{
    expr::Expr,
    stmt::{Stmt, null_stmt},
    token::{Literal, Token, TokenType},
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    pub has_error: bool,
}

#[derive(Debug)]
pub struct ParseError;

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
            has_error: false,
        }
    }
    // TODO: is Option needed? and how error is handled?
    // TODO: what happens when error occurs in the middle?
    // do we recover now?
    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            let stmt = self.declaration();
            if let Some(stmt) = stmt {
                statements.push(stmt);
            }
        }
        statements
    }
    pub fn parse_stmt(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            let stmt = self.statement();
            let Ok(stmt) = stmt else {
                return statements;
            };
            statements.push(stmt);
        }
        statements
    }
    pub fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        self.expression()
    }
    // current() is never called beyond EOF
    fn current(&self) -> &Token {
        &self.tokens[self.current]
    }
    // assumes current > 0
    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
    // when code reach EOF, nothing can proceed
    fn is_at_end(&self) -> bool {
        self.current().token_type == TokenType::EOF
    }
    fn check(&mut self, token_type: TokenType) -> bool {
        !self.is_at_end() && self.current().token_type == token_type
    }
    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous().clone()
    }
    fn match_token(&mut self, types: &[TokenType]) -> Option<Token> {
        for token_type in types {
            if self.check(token_type.clone()) {
                return Some(self.advance());
            }
        }
        None
    }
    fn declaration(&mut self) -> Option<Stmt> {
        let result = if self.match_token(&[TokenType::VAR]).is_some() {
            self.var_declaration()
        } else {
            self.statement()
        };
        match result {
            Ok(stmt) => Some(stmt),
            Err(_) => {
                self.synchronize();
                None
            }
        }
    }
    fn var_declaration(&mut self) -> Result<Stmt, ParseError> {
        let name =
            self.consume(TokenType::IDENTIFIER, "Expect variable name")?;
        let initializer = if self.match_token(&[TokenType::EQUAL]).is_some() {
            self.expression()?
        } else {
            Expr::Literal {
                value: Literal::NULL,
            }
        };
        self.consume(
            TokenType::SEMICOLON,
            "Expect ';' after variable declaration.",
        )?;
        Ok(Stmt::Variable { name, initializer })
    }
    fn statement(&mut self) -> Result<Stmt, ParseError> {
        if self.match_token(&[TokenType::PRINT]).is_some() {
            return self.print_statement();
        }
        if self.match_token(&[TokenType::WHILE]).is_some() {
            return self.while_statement();
        }
        if self.match_token(&[TokenType::LEFT_BRACE]).is_some() {
            return Ok(Stmt::Block(self.block()?));
        }
        if self.match_token(&[TokenType::FOR]).is_some() {
            return self.for_statement();
        }
        if self.match_token(&[TokenType::IF]).is_some() {
            return self.if_statement();
        }
        self.expression_statement()
    }
    fn block(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut stmts = Vec::new();
        while !self.check(TokenType::RIGHT_BRACE) && !self.is_at_end() {
            if let Some(stmt) = self.declaration() {
                stmts.push(stmt);
            }
        }
        self.consume(TokenType::RIGHT_BRACE, "Expect '}' after block.")?;
        Ok(stmts)
    }
    fn expression_statement(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.expression()?;
        self.consume(TokenType::SEMICOLON, "Expect ';' after expression.")?;
        Ok(Stmt::Expression(expr))
    }
    fn print_statement(&mut self) -> Result<Stmt, ParseError> {
        let value = self.expression()?;
        self.consume(TokenType::SEMICOLON, "Expect ';' afer value.")?;
        Ok(Stmt::Print(value))
    }
    fn while_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(TokenType::LEFT_PAREN, "Expect '(' after 'while'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RIGHT_PAREN, "Expect ')' after condition.")?;
        let body = self.statement()?;
        Ok(Stmt::While {
            condition,
            body: Box::new(body),
        })
    }
    fn for_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(TokenType::LEFT_PAREN, "Expect '(' after 'for'.")?;
        let initializer = if self.match_token(&[TokenType::SEMICOLON]).is_some()
        {
            null_stmt()
        } else if self.match_token(&[TokenType::VAR]).is_some() {
            self.var_declaration()?
        } else {
            self.expression_statement()?
        };
        let condition = if self.check(TokenType::SEMICOLON) {
            Expr::Literal {
                value: Literal::Boolean(true),
            }
        } else {
            self.expression()?
        };
        self.consume(TokenType::SEMICOLON, "Expect ';' after loop condition.")?;
        let increment = if self.check(TokenType::RIGHT_PAREN) {
            null_stmt()
        } else {
            Stmt::Expression(self.expression()?)
        };
        self.consume(TokenType::RIGHT_PAREN, "Expect ')' after 'for'.")?;
        let body = Stmt::Block(vec![initializer, self.statement()?, increment]);
        Ok(Stmt::While {
            condition,
            body: Box::new(body),
        })
    }
    fn if_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(TokenType::LEFT_PAREN, "Expect '(' after 'if'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RIGHT_PAREN, "Expect ')' after if condition.")?;
        let then_branch = self.statement()?;
        let else_branch = if self.match_token(&[TokenType::ELSE]).is_some() {
            self.statement()?
        } else {
            null_stmt()
        };
        Ok(Stmt::If {
            condition,
            then_branch: Box::new(then_branch),
            else_branch: Box::new(else_branch),
        })
    }
    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.assignment()
    }
    fn assignment(&mut self) -> Result<Expr, ParseError> {
        let expr = self.or()?;
        let Some(equal) = self.match_token(&[TokenType::EQUAL]) else {
            return Ok(expr);
        };
        let Expr::Variable { name } = expr else {
            self.error(equal, "Invalid assignment target.");
            return Err(ParseError);
        };
        let value = self.assignment()?;
        Ok(Expr::Assign {
            name,
            value: Box::new(value),
        })
    }
    fn or(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.and()?;
        while let Some(operator) = self.match_token(&[TokenType::OR]) {
            let right = self.and()?;
            expr = Expr::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }
        Ok(expr)
    }
    fn and(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.equality()?;
        while let Some(operator) = self.match_token(&[TokenType::AND]) {
            let right = self.equality()?;
            expr = Expr::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }
        Ok(expr)
    }
    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.comparison()?;
        while let Some(operator) =
            self.match_token(&[TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL])
        {
            let right = self.comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.term()?;
        while let Some(operator) = self.match_token(&[
            TokenType::GREATER,
            TokenType::GREATER_EQUAL,
            TokenType::LESS,
            TokenType::LESS_EQUAL,
        ]) {
            let right = self.term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor()?;
        while let Some(operator) =
            self.match_token(&[TokenType::PLUS, TokenType::MINUS])
        {
            let right = self.factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary()?;
        while let Some(operator) =
            self.match_token(&[TokenType::SLASH, TokenType::STAR])
        {
            let right = self.unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        if let Some(operator) =
            self.match_token(&[TokenType::BANG, TokenType::MINUS])
        {
            let right = self.unary()?;
            return Ok(Expr::Unary {
                operator,
                right: Box::new(right),
            });
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        // Return an error that will bubble up the call stack
        if let Some(token) = self.match_token(&[
            TokenType::FALSE,
            TokenType::TRUE,
            TokenType::NIL,
            TokenType::NUMBER,
            TokenType::STRING,
        ]) {
            match token.token_type {
                TokenType::FALSE => {
                    return Ok(Expr::Literal {
                        value: Literal::Boolean(false),
                    });
                }
                TokenType::TRUE => {
                    return Ok(Expr::Literal {
                        value: Literal::Boolean(true),
                    });
                }
                TokenType::NIL => {
                    return Ok(Expr::Literal {
                        value: Literal::NULL,
                    });
                }
                TokenType::STRING | TokenType::NUMBER => {
                    return Ok(Expr::Literal {
                        value: token.literal.clone(),
                    });
                }
                _ => unreachable!(),
            }
        }
        if let Some(name) = self.match_token(&[TokenType::IDENTIFIER]) {
            return Ok(Expr::Variable { name });
        }
        if self.match_token(&[TokenType::LEFT_PAREN]).is_some() {
            let e = self.expression()?;
            self.consume(
                TokenType::RIGHT_PAREN,
                "Expect ')' after expression.",
            )?;
            return Ok(Expr::Grouping { expr: Box::new(e) });
        }
        self.error(self.current().clone(), "Expect expression");
        Err(ParseError) // TODO: a clear error message then?
    }

    fn consume(
        &mut self,
        token_type: TokenType,
        msg: &str,
    ) -> Result<Token, ParseError> {
        if self.check(token_type) {
            return Ok(self.advance());
        }
        self.error(self.current().clone(), msg);
        Err(ParseError)
    }
    fn error(&mut self, token: Token, msg: &str) {
        self.has_error = true;
        eprintln!(
            "[line {}] Error at '{}': {}.",
            token.line, token.lexeme, msg
        );
    }
    fn synchronize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            if self.previous().token_type == TokenType::SEMICOLON {
                return;
            }

            match self.current().token_type {
                TokenType::CLASS
                | TokenType::FUN
                | TokenType::VAR
                | TokenType::FOR
                | TokenType::IF
                | TokenType::WHILE
                | TokenType::PRINT
                | TokenType::RETURN => return,
                _ => {}
            }

            self.advance();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{printer, scanner::Scanner, token};

    use super::*;

    #[test]
    fn parse_true() {
        let content = "true";
        let mut scanner = Scanner::new(content);
        scanner.scan_tokens();
        assert!(!scanner.has_error());
        let tokens = scanner.get_tokens();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].token_type, TokenType::TRUE);
        assert_eq!(tokens[1].token_type, TokenType::EOF);
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr().unwrap();
        printer::ast_print(&expr);
        assert!(matches!(
            expr,
            Expr::Literal {
                value: Literal::Boolean(true),
            }
        ));
        let s = printer::ast_print(&expr);
        assert_eq!(s, "true");
    }
    #[test]
    fn parse_multiply_divide() {
        let content = "30 * 75 / 32";
        let mut scanner = Scanner::new(content);
        scanner.scan_tokens();
        assert!(!scanner.has_error());
        let mut parser = Parser::new(scanner.get_tokens());
        let expr = parser.parse_expr().unwrap();
        let s = printer::ast_print(&expr);
        assert_eq!(s, "(/ (* 30.0 75.0) 32.0)");
    }
    #[test]
    fn parse_error() {
        let content = "(72 + )";
        let mut scanner = Scanner::new(content);
        scanner.scan_tokens();
        assert!(!scanner.has_error());
        let mut parser = Parser::new(scanner.get_tokens());
        let expr = parser.parse_expr();
        assert!(expr.is_err());
        assert!(parser.has_error);
    }
    #[test]
    fn parse_assign() {
        let content = "var baz; baz=1; print baz; print baz = 2; print baz;";
        let mut scanner = Scanner::new(content);
        scanner.scan_tokens();
        assert!(!scanner.has_error());
        let mut parser = Parser::new(scanner.get_tokens());
        let stmts = parser.parse();
        assert!(!parser.has_error);
    }
}
