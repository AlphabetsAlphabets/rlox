use super::token_type::Token;

struct Binary {
    left: Box<Expr>,
    operation: Token,
    right: Box<Expr>,
}

struct Unary {
    left: Box<Expr>,
    operation: Token,
}

struct Grouping {
    grouping: Box<Expr>,
}

enum Literal {
    String(String),
    Number(f64),
}

enum Expr {
    Binary(Binary),
    Unary(Unary),
    Grouping(Grouping),
    Literal(Literal),
}

// TODO
// The goal is for all operations to end up eventually matching in Expr::Literal(foo)
// in the visit fn. So recursion needs to be used to implement this change.
struct TreeWalker;

