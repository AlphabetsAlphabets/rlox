---
layout: post
title:  "Implementing an abstract syntax tree"
categories: Notes about source
---

After learning about how the [productions](https://yjh16120.github.io/rlox/notes/about/source/2021/08/08/Representing-code.html) work in general it's time to implement productions specifically for rlox.

*Production of rlox*
```
expression     -> literal | unary | binary | grouping ;
literal        -> NUMBER | STRING | "true" | "false" | "nil" ;
grouping       -> "(" expression ")" ;
unary          -> ( "-" | "!" ) expression ;
binary         -> expression operator expression ;
operator       -> "==" | "!=" | "<" | "<=" | ">" | ">=" | "+"  | "-"  | "*" | "/" ;
```
- Unary, only two terminals are in there '-', and '!'. To negate and perform the logical not operation respectively.
- Binary. Infix artihmetic (+, -, *, /), and logical operators (==, !=, <, etc.) .
- the reason `NUMBER`, and `STRING` are in all caps is because they are both one lexeme that can carry different values.
    - The `STRING` token can carry both `String("Hi there.")`, and `String("Hello there.")`. Same goes for `Number`, it can carry different numbers.
    - Whereas the `EqualEqual` token ('==' character) will always be '==' never something else.
- Literals are numbers, strings, booleans, nil/none.

Since `literal`, `unary`, `binary`, and `grouping` fall under expression. Usually classes of each of those four will be created, and inherits from an expression class. But because this is Rust, classes don't exist. Instead tuple structs and enums will be used to replicate this behaviour.

```rust
// in ast.rs
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
```
> *If this was done with an OOP language, it would've easily taken over 40 lines to do the samething in Rust.*  

The next step is to perform operations based on the variant. With something called **The Visitor Pattern**. Since the variants are under the same enum, they will share methods. To avoid implementing the same thing 4 times. The Visitor pattern is used.
