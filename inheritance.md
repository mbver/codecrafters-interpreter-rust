class Doughnut {
// General doughnut stuff...
}

class BostonCream < Doughnut {
// Boston Cream-specific stuff...
}

classDecl → "class" IDENTIFIER ( "<" IDENTIFIER )?
"{" function* "}" ;

Token name = consume(IDENTIFIER, "Expect class name.");

lox/Parser.java
in classDeclaration()

    Expr.Variable superclass = null;
    if (match(LESS)) {
      consume(IDENTIFIER, "Expect superclass name.");
      superclass = new Expr.Variable(previous());
    }

    consume(LEFT_BRACE, "Expect '{' before class body.");

---

consume(RIGHT_BRACE, "Expect '}' after class body.");

lox/Parser.java
in classDeclaration()
replace 1 line
return new Stmt.Class(name, superclass, methods);
}
---

    define(stmt.name);

lox/Resolver.java
in visitClassStmt()

    if (stmt.superclass != null) {
      resolve(stmt.superclass);
    }

    beginScope();

---

class Oops < Oops {}

---

define(stmt.name);

lox/Resolver.java
in visitClassStmt()
if (stmt.superclass != null &&
stmt.name.lexeme.equals(stmt.superclass.name.lexeme)) {
Lox.error(stmt.superclass.name,
"A class can't inherit from itself.");
}

if (stmt.superclass != null) {
---

public Void visitClassStmt(Stmt.Class stmt) {
lox/Interpreter.java
in visitClassStmt()
Object superclass = null;
if (stmt.superclass != null) {
superclass = evaluate(stmt.superclass);
if (!(superclass instanceof LoxClass)) {
throw new RuntimeError(stmt.superclass.name,
"Superclass must be a class.");
}
}

    environment.define(stmt.name.lexeme, null);

---

var NotAClass = "I am totally not a class";

class Subclass < NotAClass {} // ?!
---

      methods.put(method.name.lexeme, function);
    }

lox/Interpreter.java
in visitClassStmt()
replace 1 line
LoxClass klass = new LoxClass(stmt.name.lexeme,
(LoxClass)superclass, methods);

    environment.assign(stmt.name, klass);

---

LoxClass(String name, LoxClass superclass,
Map<String, LoxFunction> methods) {
this.superclass = superclass;
this.name = name;
---

final String name;
lox/LoxClass.java
in class LoxClass
final LoxClass superclass;
private final Map<String, LoxFunction> methods;
---

      return methods.get(name);
    }

lox/LoxClass.java
in findMethod()
if (superclass != null) {
return superclass.findMethod(name);
}

    return null;

---

class Doughnut {
cook() {
print "Fry until golden brown.";
}
}

class BostonCream < Doughnut {}

BostonCream().cook();
---

class Doughnut {
cook() {
print "Fry until golden brown.";
}
}

class BostonCream < Doughnut {
cook() {
super.cook();
print "Pipe full of custard and coat with chocolate.";
}
}

BostonCream().cook();
---

print super; // Syntax error.
--

primary → "true" | "false" | "nil" | "this"
| NUMBER | STRING | IDENTIFIER | "(" expression ")"
| "super" "." IDENTIFIER ;
---

var method = super.cook;
method();
---

      return new Expr.Literal(previous().literal);
    }

lox/Parser.java
in primary()

    if (match(SUPER)) {
      Token keyword = previous();
      consume(DOT, "Expect '.' after 'super'.");
      Token method = consume(IDENTIFIER,
          "Expect superclass method name.");
      return new Expr.Super(keyword, method);
    }

    if (match(THIS)) return new Expr.This(previous());

---

class A {
method() {
print "A method";
}
}

class B < A {
method() {
print "B method";
}

test() {
super.method();
}
}

class C < B {}

C().test();
---

      resolve(stmt.superclass);
    }

lox/Resolver.java
in visitClassStmt()

    if (stmt.superclass != null) {
      beginScope();
      scopes.peek().put("super", true);
    }

    beginScope();

---

    endScope();

lox/Resolver.java
in visitClassStmt()
if (stmt.superclass != null) endScope();

    currentClass = enclosingClass;

---

lox/Resolver.java
add after visitSetExpr()

public Void visitSuperExpr(Expr.Super expr) {
resolveLocal(expr, expr.keyword);
return null;
}

---

        throw new RuntimeError(stmt.superclass.name,
            "Superclass must be a class.");
      }
    }

    environment.define(stmt.name.lexeme, null);

lox/Interpreter.java
in visitClassStmt()

    if (stmt.superclass != null) {
      environment = new Environment(environment);
      environment.define("super", superclass);
    }

    Map<String, LoxFunction> methods = new HashMap<>();

---

LoxClass klass = new LoxClass(stmt.name.lexeme,
(LoxClass)superclass, methods);
lox/Interpreter.java
in visitClassStmt()

if (superclass != null) {
environment = environment.enclosing;
}

environment.assign(stmt.name, klass);
---

public Object visitSuperExpr(Expr.Super expr) {
int distance = locals.get(expr);
LoxClass superclass = (LoxClass)environment.getAt(
distance, "super");
}
---

LoxClass superclass = (LoxClass)environment.getAt(
distance, "super");
lox/Interpreter.java
in visitSuperExpr()

LoxInstance object = (LoxInstance)environment.getAt(
distance - 1, "this");
}
---

LoxInstance object = (LoxInstance)environment.getAt(
distance - 1, "this");
lox/Interpreter.java
in visitSuperExpr()

LoxFunction method = superclass.findMethod(expr.method.lexeme);
return method.bind(object);
}
---

LoxFunction method = superclass.findMethod(expr.method.lexeme);
lox/Interpreter.java
in visitSuperExpr()

if (method == null) {
throw new RuntimeError(expr.method,
"Undefined property '" + expr.method.lexeme + "'.");
}

return method.bind(object);
}
---

class Eclair {
cook() {
super.cook();
print "Pipe full of crème pâtissière.";
}
}
---

super.notEvenInAClass();
---

NONE,
CLASS,
lox/Resolver.java
in enum ClassType
add “,” to previous line
SUBCLASS
}
---

if (stmt.superclass != null) {
lox/Resolver.java
in visitClassStmt()
currentClass = ClassType.SUBCLASS;
resolve(stmt.superclass);
---

public Void visitSuperExpr(Expr.Super expr) {
lox/Resolver.java
in visitSuperExpr()
if (currentClass == ClassType.NONE) {
Lox.error(expr.keyword,
"Can't use 'super' outside of a class.");
} else if (currentClass != ClassType.SUBCLASS) {
Lox.error(expr.keyword,
"Can't use 'super' in a class with no superclass.");
}

resolveLocal(expr, expr.keyword);
---
