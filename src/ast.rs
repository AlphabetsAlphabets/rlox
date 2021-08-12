use super::token_type::Token;

#[derive(Clone)]
struct Binary {
    left: Box<Expr>,
    operation: Token,
    right: Box<Expr>,
}

#[derive(Clone)]
struct Unary {
    left: Box<Expr>,
    operation: Token,
}

#[derive(Clone)]
struct Grouping {
    grouping: Box<Expr>,
}

#[derive(Clone)]
enum Literal {
    String(String),
    Number(f64),
}

#[derive(Clone)]
enum Expr {
    Binary(Binary),
    Unary(Unary),
    Grouping(Grouping),
    Literal(Literal),
}

trait Visitor<T> {
    fn visit(&mut self, expr: &Expr) {
        match expr {
            Expr::Unary(unary) => todo!(),
            Expr::Binary(binary) => todo!(),
            Expr::Literal(literal) => todo!(),
            Expr::Grouping(grouping) => todo!(),
        }
    }
}
