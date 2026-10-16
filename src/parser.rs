use crate::{
    expr::Expr,
    stmt::Stmt,
    token::{Literal, Token, TokenType},
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    pub has_error: bool,
}

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
    pub fn parse(&mut self) -> Option<Vec<Stmt>> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.statement().ok()?);
        }
        Some(statements)
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
    fn statement(&mut self) -> Result<Stmt, ParseError> {
        if self.match_token(&[TokenType::PRINT]).is_some() {
            return self.print_statement();
        }
        self.expression_statement()
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
    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.equality()
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
        let stmts = parser.parse().unwrap();
        assert_eq!(stmts.len(), 1);
        let Stmt::Expression(expr) = &stmts[0] else {
            panic!("expect expression statement")
        };
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
        let stmts = parser.parse().unwrap();
        assert_eq!(stmts.len(), 1);
        let Stmt::Expression(expr) = &stmts[0] else {
            panic!("expect expression statement")
        };
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
        let stmts = parser.parse().unwrap();
        assert_eq!(stmts.len(), 1);
        let Stmt::Expression(expr) = &stmts[0] else {
            panic!("expect expression");
        };
        let s = printer::ast_print(&expr);
        assert_eq!(s, "nil");
        assert!(parser.has_error);
    }
}
