```
statement → exprStmt
| ifStmt
| printStmt
| block ;

ifStmt → "if" "(" expression ")" statement
( "else" statement )? ;

```

```
"Expression : Expr expression",
"If : Expr condition, Stmt thenBranch," +
" Stmt elseBranch",
"Print : Expr expression",
```

```
private Stmt statement() {
  if (match(IF)) return ifStatement();
  if (match(PRINT)) return printStatement();
```

```
private Stmt ifStatement() {
  consume(LEFT_PAREN, "Expect '(' after 'if'.");
  Expr condition = expression();
  consume(RIGHT_PAREN, "Expect ')' after if condition.");

  Stmt thenBranch = statement();
  Stmt elseBranch = null;
  if (match(ELSE)) {
    elseBranch = statement();
  }

  return new Stmt.If(condition, thenBranch, elseBranch);
}
```

```
public Void visitIfStmt(Stmt.If stmt) {
  if (isTruthy(evaluate(stmt.condition))) {
    execute(stmt.thenBranch);
  } else if (stmt.elseBranch != null) {
    execute(stmt.elseBranch);
  }
  return null;
}
```

```
expression     → assignment ;
assignment     → IDENTIFIER "=" assignment
               | logic_or ;
logic_or       → logic_and ( "or" logic_and )* ;
logic_and      → equality ( "and" equality )* ;
```

```
"Literal  : Object value",
"Logical  : Expr left, Token operator, Expr right",
"Unary    : Token operator, Expr right",
```

```
private Expr assignment() {
  Expr expr = or();

  if (match(EQUAL)) {
```

```
private Expr or() {
  Expr expr = and();

  while (match(OR)) {
    Token operator = previous();
    Expr right = and();
    expr = new Expr.Logical(expr, operator, right);
  }

  return expr;
}
```

```
private Expr and() {
  Expr expr = equality();

  while (match(AND)) {
    Token operator = previous();
    Expr right = equality();
    expr = new Expr.Logical(expr, operator, right);
  }

  return expr;
}
```

```
public Object visitLogicalExpr(Expr.Logical expr) {
  Object left = evaluate(expr.left);

  if (expr.operator.type == TokenType.OR) {
    if (isTruthy(left)) return left;
  } else {
    if (!isTruthy(left)) return left;
  }

  return evaluate(expr.right);
}
```

```
print "hi" or 2; // "hi".
print nil or "yes"; // "yes".
```

```
statement      → exprStmt
               | ifStmt
               | printStmt
               | whileStmt
               | block ;

whileStmt      → "while" "(" expression ")" statement ;
```

```
"Print      : Expr expression",
"Var        : Token name, Expr initializer",
"While      : Expr condition, Stmt body"
));
```

```
if (match(PRINT)) return printStatement();
if (match(WHILE)) return whileStatement();
if (match(LEFT_BRACE)) return new Stmt.Block(block());
```

```
private Stmt whileStatement() {
  consume(LEFT_PAREN, "Expect '(' after 'while'.");
  Expr condition = expression();
  consume(RIGHT_PAREN, "Expect ')' after condition.");
  Stmt body = statement();

  return new Stmt.While(condition, body);
}
```

```
public Void visitWhileStmt(Stmt.While stmt) {
  while (isTruthy(evaluate(stmt.condition))) {
    execute(stmt.body);
  }
  return null;
}
```

```
statement      → exprStmt
               | forStmt
               | ifStmt
               | printStmt
               | whileStmt
               | block ;

forStmt        → "for" "(" ( varDecl | exprStmt | ";" )
                 expression? ";"
                 expression? ")" statement ;
```

```
private Stmt statement() {
  if (match(FOR)) return forStatement();
  if (match(IF)) return ifStatement();
```

```
private Stmt forStatement() {
  consume(LEFT_PAREN, "Expect '(' after 'for'.");

  // More here...
}
```

```
consume(LEFT_PAREN, "Expect '(' after 'for'.");

Stmt initializer;
if (match(SEMICOLON)) {
  initializer = null;
} else if (match(VAR)) {
  initializer = varDeclaration();
} else {
  initializer = expressionStatement();
}
}
```

```
initializer = expressionStatement();
}

Expr condition = null;
if (!check(SEMICOLON)) {
condition = expression();
}
consume(SEMICOLON, "Expect ';' after loop condition.");
}
```

```
consume(SEMICOLON, "Expect ';' after loop condition.");

Expr increment = null;
if (!check(RIGHT_PAREN)) {
  increment = expression();
}
consume(RIGHT_PAREN, "Expect ')' after for clauses.");
}
```

```
consume(RIGHT_PAREN, "Expect ')' after for clauses.");
Stmt body = statement();

return body;
}
```

```
Stmt body = statement();

if (increment != null) {
  body = new Stmt.Block(
      Arrays.asList(
          body,
          new Stmt.Expression(increment)));
}

return body;
```

```
}

if (condition == null) condition = new Expr.Literal(true);
body = new Stmt.While(condition, body);

return body;
```

```
body = new Stmt.While(condition, body);

if (initializer != null) {
  body = new Stmt.Block(Arrays.asList(initializer, body));
}

return body;
```
