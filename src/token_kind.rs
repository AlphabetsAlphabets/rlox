#[derive(Clone, Debug)]
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

    String(String),
    Whitespace,
    Eof,
    Error(char, usize),
}

impl TokenKind {
    pub fn display(&self) -> Result<String, String> {
        // TODO: Change the return type to a `TokenKind` instead.
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

            Self::Greater => "".to_string(),
            Self::GreaterOrEqual => "".to_string(),
            Self::LessThan => "".to_string(),
            Self::LessThanOrEqual => "".to_string(),

            Self::String(text) => text.to_string(),
            Self::Whitespace => "".to_string(),
            Self::Error(character, line) => {
                format!("Unexpected character: '{}' at line {}.", character, line)
            }
            Self::Eof => "".to_string(),
        };

        if token_kind.contains("Unexpected") {
            Err(token_kind)
        } else {
            Ok(token_kind)
        }
    }
}
