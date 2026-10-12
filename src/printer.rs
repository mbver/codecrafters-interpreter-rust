use crate::{expr::Expr, token::Literal};

pub fn ast_print(expr: &Expr) -> String {
    match expr {
        Expr::Binary {
            left,
            operator,
            right,
        } => parenthesize(&operator.lexeme, &[left, right]),
        Expr::Grouping { expr } => parenthesize("group", &[expr]),
        Expr::Literal { value } => match value {
            Literal::NULL => "".to_string(),
            v => v.to_string(),
        },
        Expr::Unary { operator, right } => {
            parenthesize(&operator.lexeme, &[right])
        }
        _ => todo!(), // panic if called!
    }
}

fn parenthesize(op: &str, exprs: &[&Expr]) -> String {
    let mut res = String::new();
    res.push('(');
    res.push_str(op);
    for e in exprs {
        res.push(' ');
        res.push_str(&ast_print(e));
    }
    res.push(')');
    res
}

#[cfg(test)]
mod tests {
    use crate::token::{Token, TokenType};

    use super::*;

    #[test]
    fn prints_expression() {
        let expr = Expr::Binary {
            left: Box::new(Expr::Unary {
                operator: Token::new(
                    TokenType::MINUS,
                    "-".to_string(),
                    Literal::NULL,
                    1,
                ),
                right: Box::new(Expr::Literal {
                    value: Literal::Number(123.0),
                }),
            }),
            operator: Token {
                token_type: TokenType::STAR,
                lexeme: "*".to_string(),
                literal: Literal::NULL,
                line: 1,
            },
            right: Box::new(Expr::Grouping {
                expr: Box::new(Expr::Literal {
                    value: Literal::Number(45.67),
                }),
            }),
        };
        assert_eq!(ast_print(&expr), "(* (- 123.0) (group 45.67))")
    }
}
