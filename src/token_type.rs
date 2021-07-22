use std::fmt;

#[derive(PartialEq)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,

    Comma,
    Dot,
    Minus,
    Plus,

    Semicolon,
    Bang,
    Star,
    Equal,

    Greater,
    Less,
    Slash,
    Whitespace,

    String(String),
    Number(usize),
    Comment,

    // Two-character tokens.
    BangEqual,
    GreaterEqual,
    LessEqual,
    EqualEqual,

    // Slash,
    Eof,
    Error,
    Newline,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            TokenType::LeftParen => "(",
            TokenType::RightParen => ")",
            TokenType::LeftBrace => "{",
            TokenType::RightBrace => "}",

            TokenType::Comma => ",",
            TokenType::Dot => ".",
            TokenType::Minus => "-",
            TokenType::Plus => "+",

            TokenType::Semicolon => ";",
            TokenType::Bang => "!",
            TokenType::Star => "*",
            TokenType::Equal => "=",

            TokenType::Greater => ">",
            TokenType::Less => "<",

            TokenType::BangEqual => "!=",
            TokenType::GreaterEqual => ">=",
            TokenType::LessEqual => "<=",
            TokenType::EqualEqual => "==",

            TokenType::Comment => "Comment",
            TokenType::String(_) => "String",
            TokenType::Number(_) => "Number",

            TokenType::Eof => "Eof",
            TokenType::Error => "Error",
            TokenType::Newline => "\\n",

        };

        write!(f, "{}", text)
    }
}

pub struct Token {
    kind: TokenType,
    lexeme: String,
    literal: String,
    line: usize,
}

impl Default for Token {
    fn default() -> Self {
        let text = "".to_string();
        Token::new(TokenType::Dot, text.clone(), text, 0)
    }
}

impl Token {
    pub fn new(kind: TokenType, lexeme: String, literal: String, line: usize) -> Self {
        Token {
            kind,
            lexeme,
            literal,
            line,
        }
    }

    pub fn to_string(&self) -> String {
        let str = format!("{} {} {}", self.kind, self.lexeme, self.literal);
        str.to_string()
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TokenKind: {}\nLexeme: {}\nLiteral: {}\nLine: {}",
            self.kind, self.lexeme, self.literal, self.line
        )
    }
}
