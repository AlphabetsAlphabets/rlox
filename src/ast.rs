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

struct Identifier {
    name: String
}

enum Stmt {
    Expr(Expr),
    Let(Identifier, Expr)
}

// TODO
// The goal is for all operations to end up eventually matching in Expr::Literal(foo)
// in the visit fn. So recursion needs to be used to implement this change.
trait Visitor<T> {
    fn visit(&mut self, expr: &Expr) -> T {
        match expr {
            Expr::Unary(unary) => self.visit_unary(unary),
            Expr::Binary(binary) => self.visit_binary(binary),
            Expr::Literal(literal) => self.visit_literal(literal),
            Expr::Grouping(grouping) => self.visit_grouping(grouping)
        }
    }

    fn visit_unary(&mut self, expr: &Unary) -> T {
        let left = expr.left.clone();
        let operation = expr.operation;

        self.visit(&*left)
    }

    fn visit_binary(&mut self, expr: &Binary) {
        let left = expr.left.clone();
        let operation = &expr.operation.literal;
        let right = expr.right.clone();

        // * is to get the value out of Box.
        // then & is to pass it as a reference
        // because the visit func needs it to be a ref
        self.visit(&*left);
        self.visit(&*right);
    }

    fn visit_literal(&mut self, expr: &Literal) -> T {
        match expr {
            Literal::String(text) => *text,
            Literal::Number(num) => num,
        }
    }

    fn visit_grouping(&mut self, expr: &Grouping) {}
}
