```
var a = "outer";
{
  var a = "inner";
  print a;
}
```

```var a = "global";
{
  fun showA() {
    print a;
  }

  showA();
  var a = "block";
  showA();
}
```

```
package com.craftinginterpreters.lox;

import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Stack;

class Resolver implements Expr.Visitor<Void>, Stmt.Visitor<Void> {
  private final Interpreter interpreter;

  Resolver(Interpreter interpreter) {
    this.interpreter = interpreter;
  }
}
```

```
@Override


  void resolve(List<Stmt> statements) {
    for (Stmt statement : statements) {
      resolve(statement);
    }
  }

  private void resolve(Stmt stmt) {
    stmt.accept(this);
  }

  private void resolve(Expr expr) {
    expr.accept(this);
  }

  private void beginScope() {
    scopes.push(new HashMap<String, Boolean>());
  }

  private final Interpreter interpreter;
  // lox/Resolver.java
  // in class Resolver
  private final Stack<Map<String, Boolean>> scopes = new Stack<>();

  Resolver(Interpreter interpreter) {
  ------
    private void endScope() {
      scopes.pop();
    }

private void declare(Token name) {
  if (scopes.isEmpty()) return;

  Map<String, Boolean> scope = scopes.peek();
  scope.put(name.lexeme, false);
}

  private void define(Token name) {
    if (scopes.isEmpty()) return;
    scopes.peek().put(name.lexeme, true);
  }


  private void resolveLocal(Expr expr, Token name) {
    for (int i = scopes.size() - 1; i >= 0; i--) {
      if (scopes.get(i).containsKey(name.lexeme)) {
        interpreter.resolve(expr, scopes.size() - 1 - i);
        return;
      }
    }
  }

public Void visitBlockStmt(Stmt.Block stmt) {
  beginScope();
  resolve(stmt.statements);
  endScope();
  return null;
}
    public Void visitVarStmt(Stmt.Var stmt) {
      declare(stmt.name);
      if (stmt.initializer != null) {
        resolve(stmt.initializer);
      }
      define(stmt.name);
      return null;
    }
```

```
var a = "outer";
{
  var a = a;
}
```

```


  public Void visitFunctionStmt(Stmt.Function stmt) {
    declare(stmt.name);
    define(stmt.name);

    resolveFunction(stmt);
    return null;
  }

  private void resolveFunction(Stmt.Function function) {
    beginScope();
    for (Token param : function.params) {
      declare(param);
      define(param);
    }
    resolve(function.body);
    endScope();
  }

  public Void visitExpressionStmt(Stmt.Expression stmt) {
    resolve(stmt.expression);
    return null;
  }


  public Void visitIfStmt(Stmt.If stmt) {
    resolve(stmt.condition);
    resolve(stmt.thenBranch);
    if (stmt.elseBranch != null) resolve(stmt.elseBranch);
    return null;
  }

  public Void visitPrintStmt(Stmt.Print stmt) {
    resolve(stmt.expression);
    return null;
  }

  public Void visitReturnStmt(Stmt.Return stmt) {
    if (stmt.value != null) {
      resolve(stmt.value);
    }

    return null;
  }

  public Void visitWhileStmt(Stmt.While stmt) {
    resolve(stmt.condition);
    resolve(stmt.body);
    return null;
  }

  public Void visitAssignExpr(Expr.Assign expr) {
    resolve(expr.value);
    resolveLocal(expr, expr.name);
    return null;
  }

  public Void visitVariableExpr(Expr.Variable expr) {
    if (!scopes.isEmpty() &&
        scopes.peek().get(expr.name.lexeme) == Boolean.FALSE) {
      Lox.error(expr.name,
          "Can't read local variable in its own initializer.");
    }

    resolveLocal(expr, expr.name);
    return null;
  }


  public Void visitBinaryExpr(Expr.Binary expr) {
    resolve(expr.left);
    resolve(expr.right);
    return null;
  }


  public Void visitCallExpr(Expr.Call expr) {
    resolve(expr.callee);

    for (Expr argument : expr.arguments) {
      resolve(argument);
    }

    return null;
  }

  public Void visitGroupingExpr(Expr.Grouping expr) {
    resolve(expr.expression);
    return null;
  }


  public Void visitLiteralExpr(Expr.Literal expr) {
    return null;
  }

  public Void visitLogicalExpr(Expr.Logical expr) {
    resolve(expr.left);
    resolve(expr.right);
    return null;
  }

  public Void visitUnaryExpr(Expr.Unary expr) {
    resolve(expr.right);
    return null;
  }

```

```
private Environment environment = globals;
// lox/Interpreter.java
// in class Interpreter
private final Map<Expr, Integer> locals = new HashMap<>();

Interpreter() {
void resolve(Expr expr, int depth) {
  locals.put(expr, depth);
}

public Object visitVariableExpr(Expr.Variable expr) {
// lox/Interpreter.java
// in visitVariableExpr()
// replace 1 line
    return lookUpVariable(expr.name, expr);
  }

  private Object lookUpVariable(Token name, Expr expr) {
    Integer distance = locals.get(expr);
    if (distance != null) {
      return environment.getAt(distance, name.lexeme);
    } else {
      return globals.get(name);
    }
  }

  Object getAt(int distance, String name) {
    return ancestor(distance).values.get(name);
  }

    Environment ancestor(int distance) {
      Environment environment = this;
      for (int i = 0; i < distance; i++) {
        environment = environment.enclosing;
      }

      return environment;
    }


  public Object visitAssignExpr(Expr.Assign expr) {
    Object value = evaluate(expr.value);
  // lox/Interpreter.java
  // in visitAssignExpr()
  // replace 1 line

    Integer distance = locals.get(expr);
    if (distance != null) {
      environment.assignAt(distance, expr.name, value);
    } else {
      globals.assign(expr.name, value);
    }

    return value;

  void assignAt(int distance, Token name, Object value) {
    ancestor(distance).values.put(name.lexeme, value);
  }

    // Stop if there was a syntax error.
    if (hadError) return;

    // lox/Lox.java
    // in run()
    Resolver resolver = new Resolver(interpreter);
    resolver.resolve(statements);

    interpreter.interpret(statements);

```

```
fun bad() {
  var a = "first";
  var a = "second";
}
```

```
Map<String, Boolean> scope = scopes.peek();
// lox/Resolver.java
// in declare()
if (scope.containsKey(name.lexeme)) {
  Lox.error(name,
      "Already a variable with this name in this scope.");
}

scope.put(name.lexeme, false);
```

```
return "at top level";
This executes a return statement, but it’s not even inside a function at all. It’s top-level code.
```

```
private final Stack<Map<String, Boolean>> scopes = new Stack<>();
// lox/Resolver.java
// in class Resolver
private FunctionType currentFunction = FunctionType.NONE;

Resolver(Interpreter interpreter) {
```

```
private enum FunctionType {
  NONE,
  FUNCTION
}
```

```
define(stmt.name);

// lox/Resolver.java
// in visitFunctionStmt()
// replace 1 line
resolveFunction(stmt, FunctionType.FUNCTION);
return null;
```

```
private void resolveFunction(
    Stmt.Function function, FunctionType type) {
  FunctionType enclosingFunction = currentFunction;
  currentFunction = type;

  beginScope();
```

```
endScope();
// lox/Resolver.java
// in resolveFunction()
currentFunction = enclosingFunction;
}
```

```
public Void visitReturnStmt(Stmt.Return stmt) {
// lox/Resolver.java
// in visitReturnStmt()
  if (currentFunction == FunctionType.NONE) {
    Lox.error(stmt.keyword, "Can't return from top-level code.");
  }

  if (stmt.value != null) {
```

```
resolver.resolve(statements);
// lox/Lox.java
// in run()

// Stop if there was a resolution error.
if (hadError) return;

interpreter.interpret(statements);
```

```
var variable = "global";

{
  fun f() {
    print variable;
  }

  f();

  var variable = "local";

  f();
}
```

```
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
```

```
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
```

```
// First declaration of variable 'a' in global
// scope
var a = "value";
// Redeclaring 'a' with its own value should be
// allowed in global scope
var a = a;
print a; // this should print "value"
```
