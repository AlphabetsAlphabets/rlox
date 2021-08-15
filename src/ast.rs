use super::token_type::Token;

type Ptr<T> = Box<T>;
type ExprPtr = Ptr<Expr>;

enum Literal {
    String(String),
    Number(f64)
}

enum Expr {
    Binary(ExprPtr, Token, ExprPtr),
    Unary(ExprPtr, Token),
    Grouping(ExprPtr),
    Literal(Literal),
}

fn stringify(expr: &Expr) -> String {
    match expr {
        Expr::Binary(left, op, right) => {
            format!("({} {} {})", op.lexeme, stringify(left), stringify(right))
        }
        Expr::Unary(operand, op) => format!("({} {})", op.lexeme, stringify(operand)),
        Expr::Grouping(expr) => format!("(group {})", stringify(expr)),
        Expr::Literal(lit) => match lit {
            Literal::String(s) => s.clone(),
            Literal::Number(n) => n.to_string(),
        },
    }
}

// TODO
// The goal is for all operations to end up eventually matching in Expr::Literal(foo)
// in the visit fn. So recursion needs to be used to implement this change.

