#[derive(Clone)]
pub enum TokenKind {
    SingleQuote,
    DoubleQuote,

    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Backslash,
    Star,
    Newline,

    Equal,
    Bang,
    BangEqual,

    LessThan,
    LessThanOrEqual,
    GreaterOrEqual,
    Greater,

    Comment,
    Space,

    String,
    Whitespace,
    Eof,
    Error(char, usize),
}

impl TokenKind {
    pub fn display(&self) -> String {
        // TODO: Complete the match statement with all of the possible enum variants.
        let token_kind = match self {
            Self::SingleQuote => "".to_string(),
            Self::DoubleQuote => "".to_string(),

            Self::LeftParen => "".to_string(),
            Self::RightParen => "".to_string(),
            Self::LeftBrace => "".to_string(),
            Self::RightBrace => "".to_string(),

            Self::Comma => "".to_string(),
            Self::Dot => "".to_string(),

            Self::Minus => "".to_string(),
            Self::Plus => "".to_string(),
            Self::Semicolon => "".to_string(),
            Self::Backslash => "".to_string(),
            Self::Bang => "".to_string(),
            Self::BangEqual => "".to_string(),

            Self::Star => "".to_string(),
            Self::Equal => "".to_string(),
            Self::Newline => "".to_string(),
            Self::Comment => "".to_string(),
            Self::Space => "".to_string(),

            Self::Greater => "".to_string(),
            Self::GreaterOrEqual => "".to_string(),
            Self::LessThan => "".to_string(),
            Self::LessThanOrEqual => "".to_string(),

            Self::String => "".to_string(),
            Self::Whitespace => "".to_string(),
            Self::Error(character, line) => {
                format!("Unexpected character: '{}' at line {}.", character, line)
            }
            Self::Eof => "".to_string(),
        };

        token_kind
    }
}
