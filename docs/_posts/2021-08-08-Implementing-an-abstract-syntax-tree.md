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
- Unary refers to one so single character operators, '-', '+', etc.
- Binary refers to two so two character opreators, '>=', '==', etc.
- the reason `NUMBER`, and `STRING` are in all caps is because they are both one lexeme that can carry different values.
    - The `STRING` token can carry both `String("Hi there.")`, and `String("Hello there.")`. Same goes for `Number`, it can carry different numbers.
    - Whereas the `EqualEqual` token ('==' character) will always be '==' never something else.
