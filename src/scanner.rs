// reference: https://gist.github.com/rust-play/3df0d4653d553f1deb9b211227fee840
use super::TokenKind;

/// literal is the string itself
/// lexeme is a character in a string
#[derive(Clone)]
pub struct Token {
    pub has_error: bool,
    pub token_kind: TokenKind,
    pub literal: String,
    pub lexeme: String,
    pub line_number: usize,
}

// Used for tests
impl Default for Token {
    fn default() -> Self {
        Token {
            has_error: false,
            token_kind: TokenKind::Eof,
            lexeme: "lexeme".to_string(),
            literal: "literal".to_string(),
            line_number: 0,
        }
    }
}

impl Token {
    /// token_kind: TokenKind
    /// literal: String
    /// lexeme: String, should be a char but String is easier.
    /// line_number: usize
    pub fn new(token_kind: TokenKind, lexeme: String, literal: String, line_number: usize) -> Self {
        Token {
            has_error: false,
            token_kind,
            literal,
            lexeme,
            line_number,
        }
    }

    // Error "handling" or "reporting"
    fn error(mut self, line_number: usize, message: &str) {
        self.report(line_number, "", message);
    }

    fn report(mut self, line_number: usize, at: &str, message: &str) {
        let error = format!("[line {}] {}: {}", line_number, at, message);
        println!("{}", error);
        self.has_error = true;
    }

    // Creates tokens during tokenization.
    fn create_token(literal: &str, lexeme: char, token: TokenKind, line_number: usize) -> Token {
        let literal = literal.to_string();
        let lexeme = lexeme.to_string();
        let token = Token::new(token, lexeme, literal, line_number);

        token
    }
}

/// Tokens: Vec<Token>, vec of tokens
/// source: String, the entire lox file
/// count: usize, character in a line
/// line: usize, current line.
pub struct Scanner {
    pub tokens: Vec<Token>,
    pub source: String,
    count: usize,
    line: usize
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner { 
            tokens: vec![],
            source,
            count: 0,
            line: 0,
        }
    }
}

impl Scanner {
    fn is_at_end(&self) -> bool {
        self.count >= self.source.len()
    }

    fn scan_token(&mut self, literal: char) -> TokenKind {
        match literal {
            '\'' => TokenKind::SingleQuote,
            '"' => TokenKind::DoubleQuote,

            '(' => TokenKind::LeftParen,
            ')' => TokenKind::RightParen,

            ',' => TokenKind::Comma,
            '.' => TokenKind::Dot,

            '-' => TokenKind::Minus,
            '+' => TokenKind::Plus,
            ';' => TokenKind::Semicolon,
            '\\' => {
                if self.peek() == '\\' {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    };
                    TokenKind::Comment
                } else {
                    TokenKind::Backslash
                }
            },
            '*' => TokenKind::Star,
            '=' => TokenKind::Equal,
            literal if literal == '\n' || literal == '\r' => {
                self.line += 1;
                TokenKind::Newline
            },

            '>' => self.lookahead('=', TokenKind::GreaterOrEqual, TokenKind::Greater),
            '<' => self.lookahead('=', TokenKind::LessThanOrEqual, TokenKind::LessThan),
            '!' => self.lookahead('=', TokenKind::BangEqual, TokenKind::Bang),

            '{' => TokenKind::LeftBrace,
            '}' => TokenKind::RightBrace,

            literal if literal.is_ascii_alphabetic() || literal == '_' => {
                let start = self.count - 1;
                while self.peek().is_alphanumeric() || self.peek() == '_' {
                    self.count += 1;
                }
                let lexeme = &self.source[start..self.count];
                TokenKind::String
            },

            _ => TokenKind::Error(literal, self.line),
        }
    }

    fn advance(&mut self) -> char {
        self.count += 1;
        self.previous()
    }

    fn previous(&self) -> char {
        self.source
            .as_bytes()
            .get(self.count - 1)
            .copied()
            .unwrap_or(b'\0') as char
    }

    fn peek(&self) -> char {
        self.source
            .as_bytes()
            .get(self.count)
            .copied()
            .unwrap_or(b'\0') as char
    }

    fn eol(&self) -> bool {
        let ch = &self.source[self.count + 1..self.count + 2];
        if ch == "\n" {
            true
        } else {
            false
        }
    }

    pub fn tokenize(&mut self) {
        let source = self.source.clone();
        let chars = source.chars().into_iter();
        for literal in chars {
            let kind = self.scan_token(literal);

            let token = Token::create_token(&source, literal, kind, self.line);
            self.count += 1;
            self.tokens.push(token);
        }
    }

    pub fn print(&self) {
        let tokens = &self.tokens;
        for token in tokens {
            let s = token.token_kind.display();

            if !s.is_empty() {
                println!("{}", s);
            }
        }
    }

    /// if `look_for` is found, then return `v1`, else `v2`
    fn lookahead(&mut self, look_for: char, v1: TokenKind, v2: TokenKind) -> TokenKind {
        let character = self.advance();

        if character == look_for {
            v1
        } else {
            v2
        }
    }
}
