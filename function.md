```
average(1, 2);
getCallback()();
```

```
unary          → ( "!" | "-" ) unary | call ;
call           → primary ( "(" arguments? ")" )* ;
arguments      → expression ( "," expression )* ;

```

```
return new Expr.Unary(operator, right);
}

return call();
}
```

```
private Expr call() {
  Expr expr = primary();

  while (true) {
    if (match(LEFT_PAREN)) {
      expr = finishCall(expr);
    } else {
      break;
    }
  }

  return expr;
}
```

```
private Expr finishCall(Expr callee) {
  List<Expr> arguments = new ArrayList<>();
  if (!check(RIGHT_PAREN)) {
    do {
      arguments.add(expression());
    } while (match(COMMA));
  }

  Token paren = consume(RIGHT_PAREN,
                        "Expect ')' after arguments.");

  return new Expr.Call(callee, paren, arguments);
}
```

```
do {
  if (arguments.size() >= 255) {
    error(peek(), "Can't have more than 255 arguments.");
  }
  arguments.add(expression());
```

```
public Object visitCallExpr(Expr.Call expr) {
  Object callee = evaluate(expr.callee);

  List<Object> arguments = new ArrayList<>();
  for (Expr argument : expr.arguments) {
    arguments.add(evaluate(argument));
  }

  LoxCallable function = (LoxCallable)callee;
  return function.call(this, arguments);
}
```

```
package com.craftinginterpreters.lox;

import java.util.List;

interface LoxCallable {
  Object call(Interpreter interpreter, List<Object> arguments);
}
```

```
}
// TODO: WHERE TO PUT THIS STUFFS?
if (!(callee instanceof LoxCallable)) {
  throw new RuntimeError(expr.paren,
      "Can only call functions and classes.");
}

LoxCallable function = (LoxCallable)callee;
```

```
fun add(a, b, c) {
  print a + b + c;
}
add(1, 2, 3, 4); // Too many.
add(1, 2);       // Too few.
```

```
LoxCallable function = (LoxCallable)callee;
if (arguments.size() != function.arity()) {
  throw new RuntimeError(expr.paren, "Expected " +
      function.arity() + " arguments but got " +
      arguments.size() + ".");
}

return function.call(this, arguments);
```

```
interface LoxCallable {
  int arity();
  Object call(Interpreter interpreter, List<Object> arguments);
```

```
class Interpreter implements Expr.Visitor<Object>,
                             Stmt.Visitor<Void> {
  final Environment globals = new Environment();
  private Environment environment = globals;

  void interpret(List<Stmt> statements) {
```

```
private Environment environment = globals;
Interpreter() {
  globals.define("clock", new LoxCallable() {
    @Override
    public int arity() { return 0; }

    @Override
    public Object call(Interpreter interpreter,
                       List<Object> arguments) {
      return (double)System.currentTimeMillis() / 1000.0;
    }

    @Override
    public String toString() { return "<native fn>"; }
  });
}

void interpret(List<Stmt> statements) {
```

```
declaration    → funDecl
               | varDecl
               | statement ;

funDecl        → "fun" function ;
function       → IDENTIFIER "(" parameters? ")" block ;
parameters     → IDENTIFIER ( "," IDENTIFIER )* ;

```

```
try {
  if (match(FUN)) return function("function");
  if (match(VAR)) return varDeclaration();
```

```
private Stmt.Function function(String kind) {
  Token name = consume(IDENTIFIER, "Expect " + kind + " name.");
}
```

```
Token name = consume(IDENTIFIER, "Expect " + kind + " name.");
consume(LEFT_PAREN, "Expect '(' after " + kind + " name.");
List<Token> parameters = new ArrayList<>();
if (!check(RIGHT_PAREN)) {
  do {
    if (parameters.size() >= 255) {
      error(peek(), "Can't have more than 255 parameters.");
    }

    parameters.add(
        consume(IDENTIFIER, "Expect parameter name."));
  } while (match(COMMA));
}
consume(RIGHT_PAREN, "Expect ')' after parameters.");
}
```

```
consume(RIGHT_PAREN, "Expect ')' after parameters.");

consume(LEFT_BRACE, "Expect '{' before " + kind + " body.");
List<Stmt> body = block();
return new Stmt.Function(name, parameters, body);
}
```

```
package com.craftinginterpreters.lox;

import java.util.List;

class LoxFunction implements LoxCallable {
  private final Stmt.Function declaration;
  LoxFunction(Stmt.Function declaration) {
    this.declaration = declaration;
  }
}
```

```
// should initialize env in Function and then interpreter only execute block
public Object call(Interpreter interpreter,
                   List<Object> arguments) {
  Environment environment = new Environment(interpreter.globals);
  for (int i = 0; i < declaration.params.size(); i++) {
    environment.define(declaration.params.get(i).lexeme,
        arguments.get(i));
  }

  interpreter.executeBlock(declaration.body, environment);
  return null;
}
```

```
fun count(n) {
  if (n > 1) count(n - 1);
  print n;
}

count(3);
```

```
fun add(a, b, c) {
  print a + b + c;
}

add(1, 2, 3);
```

```
public int arity() {
  return declaration.params.size();
}
```

```
public String toString() {
  return "<fn " + declaration.name.lexeme + ">";
}
```

```
fun add(a, b) {
  print a + b;
}

print add; // "<fn add>".
```

// define func in env not in globals? because we might have local functions

```
public Void visitFunctionStmt(Stmt.Function stmt) {
  LoxFunction function = new LoxFunction(stmt);
  environment.define(stmt.name.lexeme, function);
  return null;
}
```

```
fun sayHi(first, last) {
  print "Hi, " + first + " " + last + "!";
}

sayHi("Dear", "Reader");
```

```
statement      → exprStmt
               | forStmt
               | ifStmt
               | printStmt
               | returnStmt
               | whileStmt
               | block ;

returnStmt     → "return" expression? ";" ;
```

```
fun procedure() {
  print "don't return anything";
}

var result = procedure();
print result; // ?
```

```
return nil;
```

```
if (match(PRINT)) return printStatement();
if (match(RETURN)) return returnStatement();
if (match(WHILE)) return whileStatement();
```

```
private Stmt returnStatement() {
  Token keyword = previous();
  Expr value = null;
  if (!check(SEMICOLON)) {
    value = expression();
  }

  consume(SEMICOLON, "Expect ';' after return value.");
  return new Stmt.Return(keyword, value);
}
```

```
fun count(n) {
  while (n < 100) {
    if (n == 3) return n; // <--
    print n;
    n = n + 1;
  }
}

count(1);
```

```
public Void visitReturnStmt(Stmt.Return stmt) {
  Object value = null;
  if (stmt.value != null) value = evaluate(stmt.value);

  throw new Return(value);
}
```

```
package com.craftinginterpreters.lox;

class Return extends RuntimeException {
  final Object value;

  Return(Object value) {
    super(null, null, false, false);
    this.value = value;
  }
}
```

```
arguments.get(i));
}

try {
interpreter.executeBlock(declaration.body, environment);
} catch (Return returnValue) {
return returnValue.value;
}
return null;
```

```
fun fib(n) {
  if (n <= 1) return n;
  return fib(n - 2) + fib(n - 1);
}

for (var i = 0; i < 20; i = i + 1) {
  print fib(i);
}
```

```
fun makeCounter() {
  var i = 0;
  fun count() {
    i = i + 1;
    print i;
  }

  return count;
}

var counter = makeCounter();
counter(); // "1".
counter(); // "2".
```

```
private final Stmt.Function declaration;
private final Environment closure;

LoxFunction(Stmt.Function declaration) {
```

```
LoxFunction(Stmt.Function declaration, Environment closure) {
  this.closure = closure;
  this.declaration = declaration;
```

```
public Void visitFunctionStmt(Stmt.Function stmt) {
  LoxFunction function = new LoxFunction(stmt, environment);
  environment.define(stmt.name.lexeme, function);
```

```
List<Object> arguments) {
Environment environment = new Environment(closure);
for (int i = 0; i < declaration.params.size(); i++) {
```

```
match interpreter.execute_block(&self.declaration.body, environment) {
    Ok(()) => Ok(Literal::Null),
    Err(RuntimeError::Return(value)) => Ok(value),
    Err(error) => Err(error),
}
```

RuntimeValue can be Literal or Callable: Native function or LoxFunction

function is declared and will be stored in a variable
what form is it stored? what difference between native and user-defined?
function is called later in expression. we have function name and arguments passed to it. and we will retrieve the stored function and call it
the native function is called without interpreter. the user-defined will be interprete by the interpreter. it will store the variables in environment and interpreter will call execute block

- when the interpreter encounter a call Expr like f(), it actually will evaluate the Expr::Call.
  - first, it will evaluate callee, which is stored in the environment as Callable
  - then it will evaluate each argument to a literal
  - now, with the Callable and the arguments the interpreter will decide what to do next
    - for LoxFunction, it will set_env for arguments and call execute_block on the block body of the stored function
    - for native function, it will run the stored code directly without environment

- what we have the statement function there? why it is in statement? we have Call expression. so the remaining is function declaration. so the statement func is for declaration?

- the LoxFunction stores the whole Stmt::Function
  - the call() will first take the interpreter.globals as enclosing env and define variables in its own env
  - it then call interpreter.execute_block() with that env

- if we pass the execution to interpreter, it will need to know about the details of Callable
  - but if we pass the execution to the Callable, it has to know the details of interpreter?

- we will change the environment to store both callable and Literal. but evaluate will only return Literal
  - for variable, we will retrieve the Literal via get_var
  - for a callee, we will retrieve it via get_func and we will continue to evaluate the func via evaluate_func() in the interpreter

- evaluate now returns a RuntimeValue. but that doesn't reflect the true nature of it. everything must be finally evaluated to Literal so the expression can be resolved.

- but now, we store a Callable in a variable. and the callable isn't in Literal state. it needs to be further processed.
- we can't turn it into a Literal so we must return it in a Callable
- the branch for call expression will pick it up and further finish it to turn it to Literal
- all the branch will evaluate to Literal, only the Variable and Call branch has to deal with Callable.
- when they are parsed, they will be resolved into Identifier and they will be stored and retrieved in the environment.
- so they are already in Identifier case
- and ready to be retrieved
- should i further call evaluate for callable or i can retrieve it directly from environment. similarly, for variable, should i retrieve it as Literal and call it later?

- for language that support function pointer, i can actually retrieve a function from variable. if so, i must support comparsion and other things like Literal for it?

- we can return a function from a function. it means the Call expression can be evaluated into a function. and if we want anonymous function, we can support it because variable can store a function. it means evaluate should support Callable alongside Literal

- if we support evaluate to Callable, should we implement those for Literal with it? or we reject the operation if it isn't Literal?
