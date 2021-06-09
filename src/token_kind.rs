use std::fmt;

pub enum TokenKind {
    // Single-character tokens.
    Left_paren,
    Right_paren,
    Left_brace,
    Right_brace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // one or two character tokens.
    Bang,
    Bang_equal,
    Equal,
    Equal_equal,
    Greater,
    Greater_equal,
    Less,
    Less_equal,

    // literals.
    Identifier,
    String,
    Number,
    Print,
    Return,

    Eof,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let token_kind = match self {
            Self::Print => "print",
            Self::Eof => "eof",
            _ => "something else"
        };

        write!(f, "{}", token_kind)
    }
}
