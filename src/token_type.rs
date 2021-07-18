use std::fmt;

pub enum TokenType {
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
    // Slash,
    Star,
    Eof,
    Error,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            TokenType::Left_paren => "(",
            TokenType::Right_paren => ")",
            TokenType::Left_brace => "{",
            TokenType::Right_brace => "}",
            TokenType::Comma => ",",
            TokenType::Dot => ".",
            TokenType::Minus => "-",
            TokenType::Plus => "+",
            TokenType::Semicolon => ";",
            TokenType::Star => "*",
            TokenType::Error => "Error",
            TokenType::Eof => "Eof",
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
