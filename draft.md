```rs
fn equality(&mut self) -> Expr {
    let mut expr = self.comparison();

    while let Some(operator) =
        self.match_token(&[TokenType::BangEqual, TokenType::EqualEqual])
    {
        let right = self.comparison();
        expr = Expr::Binary {
            left: Box::new(expr),
            operator,
            right: Box::new(right),
        };
    }

    expr
}

fn match_token(&mut self, types: &[TokenType]) -> Option<Token> {
    for &token_type in types {
        if self.check(token_type) {
            return Some(self.advance());
        }
    }

    None
}
```

```rs
struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    // Grammar rules ...

    fn check(&self, token_type: TokenType) -> bool {
        !self.is_at_end() && self.peek().token_type == token_type
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous().clone()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
}
```

```rs
fn primary(&mut self) -> Result<Expr, ParseError> {
    if let Some(token) = self.match_token(&[
        TokenType::False,
        TokenType::True,
        TokenType::Nil,
        TokenType::Number,
        TokenType::String,
    ]) {
        return Ok(Expr::Literal(token.literal));
    }

    if self.match_token(&[TokenType::LeftParen]).is_some() {
        let expr = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
        return Ok(Expr::Grouping(Box::new(expr)));
    }

    Err(self.error("Expect expression."))
}
fn consume(
    &mut self,
    token_type: TokenType,
    message: &str,
) -> Result<Token, ParseError> {
    if self.check(token_type) {
        return Ok(self.advance());
    }

    Err(self.error(message))
}

let expr = self.expression()?;
self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
Ok(expr)
```

```rs
private ParseError error(Token token, String message) {
    Lox.error(token, message);
    return new ParseError();
}
fn error(&self, message: &str) -> ParseError {
    let token = self.peek();

    self.lox_error(token, message);

    ParseError
}
struct ParseError;
// ....
let expr = self.expression()?;
self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
Ok(expr)
```

````rs
match self.declaration() {
    Ok(stmt) => statements.push(stmt),
    Err(_) => self.synchronize(),
}```
````

```rs
fn synchronize(&mut self) {
    self.advance();

    while !self.is_at_end() {
        if self.previous().token_type == TokenType::Semicolon {
            return;
        }

        match self.peek().token_type {
            TokenType::Class
            | TokenType::Fun
            | TokenType::Var
            | TokenType::For
            | TokenType::If
            | TokenType::While
            | TokenType::Print
            | TokenType::Return => return,
            _ => {}
        }

        self.advance();
    }
}
```

```rs
pub fn parse(&mut self) -> Result<Expr, ParseError> {
    self.expression()
}
pub struct ParseError;
fn error(&self, message: &str) -> ParseError {
    let token = self.peek();

    // Report the error at this token.
    // Use our existing error-reporting implementation.

    ParseError
}

let tokens = scanner.scan_tokens();

let mut parser = Parser::new(tokens);
let expression = match parser.parse() {
    Ok(expr) => expr,
    Err(_) => {
        self.had_error = true;
        return;
    }
};

println!("{}", AstPrinter::new().print(&expression));
```
