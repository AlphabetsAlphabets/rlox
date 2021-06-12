#[derive(Clone)]
pub enum TokenKind {
    Single_quote,
    Double_quote,
    Left_paren,
    Right_paren,
    Left_brace,
    Right_brance,
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
    Bang_equal,

    Less_than,
    Less_than_or_equal,
    Greater_or_equal,
    Greater,

    Comment,
    Space,

    Eof,
    Error(char, usize),
}

impl TokenKind {
    pub fn display(&self) -> String {
        // TODO: Complete the match statement with all of the possible enum variants.
        let token_kind = match self {
            Self::Single_quote => "".to_string(),
            Self::Double_quote => "".to_string(),

            Self::Left_paren => "".to_string(),
            Self::Right_paren => "".to_string(),
            Self::Left_brace => "".to_string(),
            Self::Right_brance => "".to_string(),

            Self::Comma => "".to_string(),
            Self::Dot => "".to_string(),

            Self::Minus => "".to_string(),
            Self::Plus => "".to_string(),
            Self::Semicolon => "".to_string(),
            Self::Backslash => "".to_string(),
            Self::Bang => "".to_string(),
            Self::Bang_equal => "".to_string(),

            Self::Star => "".to_string(),
            Self::Equal => "".to_string(),
            Self::Newline => "".to_string(),
            Self::Comment => "".to_string(),
            Self::Space => "".to_string(),

            Self::Greater => "".to_string(),
            Self::Greater_or_equal => "".to_string(),
            Self::Less_than => "".to_string(),
            Self::Less_than_or_equal => "".to_string(),

            Self::Error(character, line) => {
                format!("Unexpected character: '{}' at line {}.", character, line)
            }
            Self::Eof => "".to_string(),
        };

        token_kind
    }
}
