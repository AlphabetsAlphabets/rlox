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

    // keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
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
        Token::new(TokenType::Dot, text, text, 0)
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
