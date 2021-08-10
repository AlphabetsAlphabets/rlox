use super::token_type::Token;

struct Binary(Box<Expr>, Token, Box<Expr>);
struct Unary(Box<Expr>, Token);
struct Grouping(Box<Expr>);
struct Literal(Token);

enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Unary(Box<Expr>, Token),
    Grouping(Box<Expr>),
    Literal(Token),
}

