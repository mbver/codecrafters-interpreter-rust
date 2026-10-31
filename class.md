```
declaration    → classDecl
               | funDecl
               | varDecl
               | statement ;

classDecl      → "class" IDENTIFIER "{" function* "}" ;
```

```
function       → IDENTIFIER "(" parameters? ")" block ;
parameters     → IDENTIFIER ( "," IDENTIFIER )* ;
```

```
class Breakfast {
  cook() {
    print "Eggs a-fryin'!";
  }

  serve(who) {
    print "Enjoy your breakfast, " + who + ".";
  }
}
```

```
try {
// lox/Parser.java
// in declaration()
  if (match(CLASS)) return classDeclaration();
  if (match(FUN)) return function("function");
```

```
private Stmt classDeclaration() {
  Token name = consume(IDENTIFIER, "Expect class name.");
  consume(LEFT_BRACE, "Expect '{' before class body.");

  List<Stmt.Function> methods = new ArrayList<>();
  while (!check(RIGHT_BRACE) && !isAtEnd()) {
    methods.add(function("method"));
  }

  consume(RIGHT_BRACE, "Expect '}' after class body.");

  return new Stmt.Class(name, methods);
}
```

```
// lox/Resolver.java
// add after visitBlockStmt()
@Override
public Void visitClassStmt(Stmt.Class stmt) {
  declare(stmt.name);
  define(stmt.name);
  return null;
}
```

```
// lox/Interpreter.java
// add after visitBlockStmt()
@Override
public Void visitClassStmt(Stmt.Class stmt) {
  environment.define(stmt.name.lexeme, null); // why don't just define with class?
  LoxClass klass = new LoxClass(stmt.name.lexeme);
  environment.assign(stmt.name, klass);
  return null;
}
```

```
This looks similar to how we execute function declarations. We declare the class’s name in the current environment. Then we turn the class syntax node into a LoxClass, the runtime representation of a class. We circle back and store the class object in the variable we previously declared. That two-stage variable binding process allows references to the class inside its own methods.?????
```

```
// lox/LoxClass.java
// create new file

package com.craftinginterpreters.lox;

import java.util.List;
import java.util.Map;

class LoxClass {
  final String name;

  LoxClass(String name) {
    this.name = name;
  }

  @Override
  public String toString() {
    return name;
  }
}
```

```
class DevonshireCream {
  serveOn() {
    return "Scones";
  }
}

print DevonshireCream; // Prints "DevonshireCream".
```

```
import java.util.Map;

lox/LoxClass.java
replace 1 line
class LoxClass implements LoxCallable {
  final String name;
```

```
// lox/LoxClass.java
// add after toString()

public Object call(Interpreter interpreter,
                   List<Object> arguments) {
  LoxInstance instance = new LoxInstance(this);
  return instance;
}

@Override
public int arity() {
  return 0;
}
```

```
// lox/LoxInstance.java
// create new file

package com.craftinginterpreters.lox;

import java.util.HashMap;
import java.util.Map;

class LoxInstance {
  private LoxClass klass;

  LoxInstance(LoxClass klass) {
    this.klass = klass;
  }

  @Override
  public String toString() {
    return klass.name + " instance";
  }
}
```

```
class Bagel {}
var bagel = Bagel();
print bagel; // Prints "Bagel instance".
```

```
call           → primary ( "(" arguments? ")" | "." IDENTIFIER )* ;
```

```
while (true) {
  if (match(LEFT_PAREN)) {
    expr = finishCall(expr);
// lox/Parser.java
// in call()
  } else if (match(DOT)) {
    Token name = consume(IDENTIFIER,
        "Expect property name after '.'.");
    expr = new Expr.Get(expr, name);
  } else {
    break;
  }
}
```

```
// lox/Resolver.java
// add after visitCallExpr()

@Override
public Void visitGetExpr(Expr.Get expr) {
  resolve(expr.object);
  return null;
}
```

```
return new Expr.Assign(name, value);
// lox/Parser.java
// in assignment()
} else if (expr instanceof Expr.Get) {
Expr.Get get = (Expr.Get)expr;
return new Expr.Set(get.object, get.name, value);
```

```
// lox/Resolver.java
// add after visitLogicalExpr()
public Void visitSetExpr(Expr.Set expr) {
  resolve(expr.value);
  resolve(expr.object);
  return null;
}
```

```
public Object visitSetExpr(Expr.Set expr) {
  Object object = evaluate(expr.object);

  if (!(object instanceof LoxInstance)) {
    throw new RuntimeError(expr.name,
                           "Only instances have fields.");
  }

  Object value = evaluate(expr.value);
  ((LoxInstance)object).set(expr.name, value);
  return value;
}
```

```
// lox/LoxInstance.java
// add after get()

void set(Token name, Object value) {
  fields.put(name.lexeme, value);
}
```

```
var m = object.method;
m(argument);

class Box {}

fun notMethod(argument) {
  print "called function with " + argument;
}

var box = Box();
box.function = notMethod;
box.function("argument");


var eggs = omelette.filledWith(cheese);
breakfast(eggs, sausage);
```

```
class Person {
  sayName() {
    print this.name;
  }
}

var jane = Person();
jane.name = "Jane";

var method = jane.sayName;
method(); // ?

class Person {
  sayName() {
    print this.name;
  }
}

var jane = Person();
jane.name = "Jane";

var bill = Person();
bill.name = "Bill";

bill.sayName = jane.sayName;
bill.sayName(); // ?
```

```
define(stmt.name);
// lox/Resolver.java
// in visitClassStmt()

for (Stmt.Function method : stmt.methods) {
  FunctionType declaration = FunctionType.METHOD;
  resolveFunction(method, declaration);
}

return null;
```

```
NONE,
FUNCTION,
lox/Resolver.java
in enum FunctionType
add “,” to previous line
METHOD
}
```

```
environment.define(stmt.name.lexeme, null);
// lox/Interpreter.java
// in visitClassStmt()
// replace 1 line

Map<String, LoxFunction> methods = new HashMap<>();
for (Stmt.Function method : stmt.methods) {
  LoxFunction function = new LoxFunction(method, environment);
  methods.put(method.name.lexeme, function);
}

LoxClass klass = new LoxClass(stmt.name.lexeme, methods);
environment.assign(stmt.name, klass);
```

```
final String name;
// lox/LoxClass.java
// in class LoxClass
// replace 4 lines
private final Map<String, LoxFunction> methods;

LoxClass(String name, Map<String, LoxFunction> methods) {
  this.name = name;
  this.methods = methods;
}

@Override
public String toString() {
```

```
Object get(Token name) {
  if (fields.containsKey(name.lexeme)) {
    return fields.get(name.lexeme);
  }

// lox/LoxInstance.java
// in get()
  LoxFunction method = klass.findMethod(name.lexeme);
  if (method != null) return method;

  throw new RuntimeError(name,
      "Undefined property '" + name.lexeme + "'.");
```

```
LoxFunction findMethod(String name) {
  if (methods.containsKey(name)) {
    return methods.get(name);
  }

  return null;
}
```

```
class Bacon {
  eat() {
    print "Crunch crunch crunch!";
  }
}

Bacon().eat(); // Prints "Crunch crunch crunch!".
```

```
class Egotist {
  speak() {
    print this;
  }
}

var method = Egotist().speak;
method();
```

```
class Cake {
  taste() {
    var adjective = "delicious";
    print "The " + this.flavor + " cake is " + adjective + "!";
  }
}

var cake = Cake();
cake.flavor = "German chocolate";
cake.taste(); // Prints "The German chocolate cake is delicious!".
```

```
class Thing {
  getCallback() {
    fun localFunction() {
      print this;
    }

    return localFunction;
  }
}

var callback = Thing().getCallback();
callback();
```

```
return new Expr.Literal(previous().literal);
}
// lox/Parser.java
// in primary()

if (match(THIS)) return new Expr.This(previous());

if (match(IDENTIFIER)) {
```

```
lox/Resolver.java
add after visitSetExpr()

public Void visitThisExpr(Expr.This expr) {
  resolveLocal(expr, expr.keyword);
  return null;
}
```

```
define(stmt.name);

// lox/Resolver.java
// in visitClassStmt()
beginScope();
scopes.peek().put("this", true);

for (Stmt.Function method : stmt.methods) {
```

```
}

// lox/Resolver.java
// in visitClassStmt()
endScope();

return null;
```

```
LoxFunction method = klass.findMethod(name.lexeme);
// lox/LoxInstance.java
// in get()
// replace 1 line
if (method != null) return method.bind(this);

throw new RuntimeError(name,
    "Undefined property '" + name.lexeme + "'.");
```

```
// lox/LoxFunction.java
// add after LoxFunction()

LoxFunction bind(LoxInstance instance) {
  Environment environment = new Environment(closure);
  environment.define("this", instance);
  return new LoxFunction(declaration, environment);
}
```

```
// lox/Interpreter.java
// add after visitSetExpr()

public Object visitThisExpr(Expr.This expr) {
  return lookUpVariable(expr.keyword, expr);
}
```

```
// lox/Resolver.java
// add after enum FunctionType

private enum ClassType {
  NONE,
  CLASS
}

private ClassType currentClass = ClassType.NONE;

void resolve(List<Stmt> statements) {
```

```
public Void visitClassStmt(Stmt.Class stmt) {
// lox/Resolver.java
// in visitClassStmt()
  ClassType enclosingClass = currentClass;
  currentClass = ClassType.CLASS;

  declare(stmt.name);
```

```
endScope();

// lox/Resolver.java
// in visitClassStmt()
currentClass = enclosingClass;
return null;
```

```
public Void visitThisExpr(Expr.This expr) {
lox/Resolver.java
in visitThisExpr()
  if (currentClass == ClassType.NONE) {
    Lox.error(expr.keyword,
        "Can't use 'this' outside of a class.");
    return null;
  }

  resolveLocal(expr, expr.keyword);
```

```
LoxInstance instance = new LoxInstance(this);
// lox/LoxClass.java
// in call()
LoxFunction initializer = findMethod("init");
if (initializer != null) {
  initializer.bind(instance).call(interpreter, arguments);
}

return instance;
```

```
public int arity() {
lox/LoxClass.java
in arity()
replace 1 line
  LoxFunction initializer = findMethod("init");
  if (initializer == null) return 0;
  return initializer.arity();
}
```

```
class Foo {
  init() {
    print this;
  }
}

var foo = Foo();
print foo.init();
```

```
return returnValue.value;
}
// lox/LoxFunction.java
// in call()

if (isInitializer) return closure.getAt(0, "this");
return null;
```

```
private final Environment closure;

// lox/LoxFunction.java
// in class LoxFunction
// replace 1 line
private final boolean isInitializer;

LoxFunction(Stmt.Function declaration, Environment closure,
            boolean isInitializer) {
  this.isInitializer = isInitializer;
  this.closure = closure;
  this.declaration = declaration;
```

```
public Void visitFunctionStmt(Stmt.Function stmt) {
// lox/Interpreter.java
// in visitFunctionStmt()
replace 1 line
  LoxFunction function = new LoxFunction(stmt, environment,
                                         false);
  environment.define(stmt.name.lexeme, function);
```

```
for (Stmt.Function method : stmt.methods) {
// lox/Interpreter.java
// in visitClassStmt()
// replace 1 line
  LoxFunction function = new LoxFunction(method, environment,
      method.name.lexeme.equals("init"));
  methods.put(method.name.lexeme, function);
```

```
environment.define("this", instance);
// lox/LoxFunction.java
// in bind()
// replace 1 line
return new LoxFunction(declaration, environment,
                       isInitializer);
}
```

```
class Foo {
  init() {
    return "something else";
  }
}
```

```
FUNCTION,
lox/Resolver.java
in enum FunctionType
INITIALIZER,
METHOD
```

```
FunctionType declaration = FunctionType.METHOD;
lox/Resolver.java
in visitClassStmt()
if (method.name.lexeme.equals("init")) {
  declaration = FunctionType.INITIALIZER;
}

resolveFunction(method, declaration);
```

```
if (stmt.value != null) {
lox/Resolver.java
in visitReturnStmt()
  if (currentFunction == FunctionType.INITIALIZER) {
    Lox.error(stmt.keyword,
        "Can't return a value from an initializer.");
  }

  resolve(stmt.value);
```

```
class Foo {
  init() {
    return;
  }
}
```

```
} catch (Return returnValue) {
lox/LoxFunction.java
in call()
  if (isInitializer) return closure.getAt(0, "this");

  return returnValue.value;
```
