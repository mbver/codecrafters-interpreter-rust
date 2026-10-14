```java
package com.craftinginterpreters.lox;

class Interpreter implements Expr.Visitor<Object> {
}

  @Override
  public Object visitLiteralExpr(Expr.Literal expr) {
    return expr.value;
  }

    public Object visitGroupingExpr(Expr.Grouping expr) {
      return evaluate(expr.expression);
    }

  private Object evaluate(Expr expr) {
    return expr.accept(this);
  }

  @Override
  public Object visitUnaryExpr(Expr.Unary expr) {
    Object right = evaluate(expr.right);

    switch (expr.operator.type) {
      case MINUS:
        return -(double)right;
    }

    // Unreachable.
    return null;
  }

    switch (expr.operator.type) {
      case BANG:
        return !isTruthy(right);
      case MINUS:

    private boolean isTruthy(Object object) {
      if (object == null) return false;
      if (object instanceof Boolean) return (boolean)object;
      return true;
    }

  public Object visitBinaryExpr(Expr.Binary expr) {
    Object left = evaluate(expr.left);
    Object right = evaluate(expr.right);

    switch (expr.operator.type) {
      case MINUS:
        return (double)left - (double)right;
      case SLASH:
        return (double)left / (double)right;
      case STAR:
        return (double)left * (double)right;
    }

    // Unreachable.
    return null;
  }

    switch (expr.operator.type) {
      case MINUS:
        return (double)left - (double)right;
      case PLUS:
        if (left instanceof Double && right instanceof Double) {
          return (double)left + (double)right;
        }

        if (left instanceof String && right instanceof String) {
          return (String)left + (String)right;
        }

        break;
      case SLASH:

    switch (expr.operator.type) {
      case GREATER:
        return (double)left > (double)right;
      case GREATER_EQUAL:
        return (double)left >= (double)right;
      case LESS:
        return (double)left < (double)right;
      case LESS_EQUAL:
        return (double)left <= (double)right;
      case MINUS:

    case BANG_EQUAL: return !isEqual(left, right);
    case EQUAL_EQUAL: return isEqual(left, right);

  private boolean isEqual(Object a, Object b) {
    if (a == null && b == null) return true;
    if (a == null) return false;

    return a.equals(b);
  }


case MINUS:
        checkNumberOperand(expr.operator, right);
        return -(double)right;

  private void checkNumberOperand(Token operator, Object operand) {
    if (operand instanceof Double) return;
    throw new RuntimeError(operator, "Operand must be a number.");
  }

package com.craftinginterpreters.lox;

class RuntimeError extends RuntimeException {
  final Token token;

  RuntimeError(Token token, String message) {
    super(message);
    this.token = token;
  }
}


      case GREATER:
        checkNumberOperands(expr.operator, left, right);
        return (double)left > (double)right;
      case GREATER_EQUAL:
        checkNumberOperands(expr.operator, left, right);
        return (double)left >= (double)right;

      case LESS:
        checkNumberOperands(expr.operator, left, right);
        return (double)left < (double)right;
      case LESS_EQUAL:
        checkNumberOperands(expr.operator, left, right);
        return (double)left <= (double)right;
      case MINUS:
        checkNumberOperands(expr.operator, left, right);
        return (double)left - (double)right;
      case SLASH:
        checkNumberOperands(expr.operator, left, right);
        return (double)left / (double)right;
      case STAR:
        checkNumberOperands(expr.operator, left, right);
        return (double)left * (double)right;

      private void checkNumberOperands(Token operator,
                                        Object left, Object right) {
        if (left instanceof Double && right instanceof Double) return;

        throw new RuntimeError(operator, "Operands must be numbers.");
      }


              .....
              return (String)left + (String)right;
            }
            throw new RuntimeError(expr.operator,
                "Operands must be two numbers or two strings.");
          case SLASH:


void interpret(Expr expression) {
  try {
    Object value = evaluate(expression);
    System.out.println(stringify(value));
  } catch (RuntimeError error) {
    Lox.runtimeError(error);
  }
}

  private String stringify(Object object) {
    if (object == null) return "nil";

    if (object instanceof Double) {
      String text = object.toString();
      if (text.endsWith(".0")) {
        text = text.substring(0, text.length() - 2);
      }
      return text;
    }

    return object.toString();
  }


  static void runtimeError(RuntimeError error) {
    System.err.println(error.getMessage() +
        "\n[line " + error.token.line + "]");
    hadRuntimeError = true;
  }


  static boolean hadError = false;
  static boolean hadRuntimeError = false;

  public static void main(String[] args) throws IOException {


      run(new String(bytes, Charset.defaultCharset()));

      // Indicate an error in the exit code.
      if (hadError) System.exit(65);
      if (hadRuntimeError) System.exit(70);
    }



public class Lox {
  private static final Interpreter interpreter = new Interpreter();
  static boolean hadError = false;

    // Stop if there was a syntax error.
    if (hadError) return;
    interpreter.interpret(expression);
  }

```
