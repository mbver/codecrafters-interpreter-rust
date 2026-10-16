```
program        → statement* EOF ;

statement      → exprStmt
               | printStmt ;

exprStmt       → expression ";" ;
printStmt      → "print" expression ";" ;
```

```java
List<Stmt> parse() {
  List<Stmt> statements = new ArrayList<>();
  while (!isAtEnd()) {
    statements.add(statement());
  }

  return statements;
}

  private Stmt statement() {
    if (match(PRINT)) return printStatement();

    return expressionStatement();
  }

  private Stmt printStatement() {
    Expr value = expression();
    consume(SEMICOLON, "Expect ';' after value.");
    return new Stmt.Print(value);
  }

  private Stmt expressionStatement() {
    Expr expr = expression();
    consume(SEMICOLON, "Expect ';' after expression.");
    return new Stmt.Expression(expr);
  }

  public Void visitExpressionStmt(Stmt.Expression stmt) {
    evaluate(stmt.expression);
    return null;
  }

  public Void visitPrintStmt(Stmt.Print stmt) {
    Object value = evaluate(stmt.expression);
    System.out.println(stringify(value));
    return null;
  }

  void interpret(List<Stmt> statements) {
    try {
      for (Stmt statement : statements) {
        execute(statement);
      }
    } catch (RuntimeError error) {
      Lox.runtimeError(error);
    }
  }

  private void execute(Stmt stmt) {
    stmt.accept(this);
  }


  ...
  Parser parser = new Parser(tokens);
  List<Stmt> statements = parser.parse();

  ...
    if (hadError) return;
    interpreter.interpret(statements);
  }

  ...

```

```
program        → declaration* EOF ;

declaration    → varDecl
               | statement ;

statement      → exprStmt
               | printStmt ;

varDecl        → "var" IDENTIFIER ( "=" expression )? ";" ;

primary        → "true" | "false" | "nil"
               | NUMBER | STRING
               | "(" expression ")"
               | IDENTIFIER ;
```

```java
List<Stmt> parse() {
  List<Stmt> statements = new ArrayList<>();
  while (!isAtEnd()) {
    statements.add(declaration());
  }

  return statements;
}

  private Stmt declaration() {
    try {
      if (match(VAR)) return varDeclaration();

      return statement();
    } catch (ParseError error) {
      synchronize();
      return null;
    }
  }

  private Stmt varDeclaration() {
    Token name = consume(IDENTIFIER, "Expect variable name.");

    Expr initializer = null;
    if (match(EQUAL)) {
      initializer = expression();
    }

    consume(SEMICOLON, "Expect ';' after variable declaration.");
    return new Stmt.Var(name, initializer);
  }

  ...
  return new Expr.Literal(previous().literal);
      }

      if (match(IDENTIFIER)) {
        return new Expr.Variable(previous());
      }

      if (match(LEFT_PAREN)) {

```

```java
package com.craftinginterpreters.lox;

import java.util.HashMap;
import java.util.Map;

class Environment {
  private final Map<String, Object> values = new HashMap<>();
}

  void define(String name, Object value) {
    values.put(name, value);
  }

```

```java
class Environment {
  private final Map<String, Object> values = new HashMap<>();

  Object get(Token name) {
    if (values.containsKey(name.lexeme)) {
      return values.get(name.lexeme);
    }

    throw new RuntimeError(name,
        "Undefined variable '" + name.lexeme + "'.");
  }

  void define(String name, Object value) {
```

```java
class Interpreter implements Expr.Visitor<Object>,
                             Stmt.Visitor<Void> {
  private Environment environment = new Environment();

  void interpret(List<Stmt> statements) {
```

```java
public Void visitVarStmt(Stmt.Var stmt) {
  Object value = null;
  if (stmt.initializer != null) {
    value = evaluate(stmt.initializer);
  }

  environment.define(stmt.name.lexeme, value);
  return null;
}
```

```java
public Object visitVariableExpr(Expr.Variable expr) {
  return environment.get(expr.name);
}
```

```
expression     → assignment ;
assignment     → IDENTIFIER "=" assignment
               | equality ;
```

```
private Expr expression() {
  return assignment();
}
```

```java
private Expr assignment() {
  Expr expr = equality();

  if (match(EQUAL)) {
    Token equals = previous();
    Expr value = assignment();

    if (expr instanceof Expr.Variable) {
      Token name = ((Expr.Variable)expr).name;
      return new Expr.Assign(name, value);
    }

    error(equals, "Invalid assignment target.");
  }

  return expr;
}
```

```
public Object visitAssignExpr(Expr.Assign expr) {
  Object value = evaluate(expr.value);
  environment.assign(expr.name, value);
  return value;
}

  void assign(Token name, Object value) {
    if (values.containsKey(name.lexeme)) {
      values.put(name.lexeme, value);
      return;
    }

    throw new RuntimeError(name,
        "Undefined variable '" + name.lexeme + "'.");
  }

```

```
class Environment {
  final Environment enclosing;
  private final Map<String, Object> values = new HashMap<>();

  Environment() {
    enclosing = null;
  }

  Environment(Environment enclosing) {
    this.enclosing = enclosing;
  }

```

```
return values.get(name.lexeme);
}

if (enclosing != null) return enclosing.get(name);

throw new RuntimeError(name,
  "Undefined variable '" + name.lexeme + "'.");
```

```
values.put(name.lexeme, value);
return;
}

if (enclosing != null) {
enclosing.assign(name, value);
return;
}

throw new RuntimeError(name,
```

```
statement      → exprStmt
               | printStmt
               | block ;

block          → "{" declaration* "}" ;
```

```
if (match(PRINT)) return printStatement();
if (match(LEFT_BRACE)) return new Stmt.Block(block());

return expressionStatement();
```

```
private List<Stmt> block() {
  List<Stmt> statements = new ArrayList<>();

  while (!check(RIGHT_BRACE) && !isAtEnd()) {
    statements.add(declaration());
  }

  consume(RIGHT_BRACE, "Expect '}' after block.");
  return statements;
}
```

```
public Void visitBlockStmt(Stmt.Block stmt) {
  executeBlock(stmt.statements, new Environment(environment));
  return null;
}
```

```
void executeBlock(List<Stmt> statements,
                  Environment environment) {
  Environment previous = this.environment;
  try {
    this.environment = environment;

    for (Stmt statement : statements) {
      execute(statement);
    }
  } finally {
    this.environment = previous;
  }
}
```
