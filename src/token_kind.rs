use std::fmt;

pub enum TokenKind {
    // Single-character tokens.
    Single_quote,
    Double_quote,
    Left_paren,
    Right_paren,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Backslash,
    Star,
    Newline,

    Space,

    Eof,
    Error(char, usize),
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: Complete the match statement with all of the possible enum variants.

        let token_kind = match self {
            Self::Single_quote => "'".to_string(),
            Self::Double_quote => "\"".to_string(),
            Self::Left_paren => "(".to_string(),
            Self::Right_paren => ")".to_string(),
            Self::Comma => ",".to_string(),
            Self::Dot => ".".to_string(),
            Self::Minus => "-".to_string(),
            Self::Plus => "+".to_string(),
            Self::Semicolon => ";".to_string(),
            Self::Backslash => "\\".to_string(),
            Self::Star => "*".to_string(),

            Self::Newline => "".to_string(),
            Self::Space => "".to_string(),
            Self::Eof => "eof".to_string(),
            Self::Error(character, line) => {
                format!("Unexpected character: {} at {}.", character, line)
            }
        };

        write!(f, "{}", &token_kind)
    }
}
