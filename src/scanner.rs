use super::token_type::*;
use super::Lox;

pub struct Scanner {
    source: String,
    pub tokens: Vec<Token>,
    // These three fields keeps track of what, and where the scanner is looking at
    column: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source,
            tokens: vec![],
            column: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) {
        let mut lox = Lox::new();
        while !self.is_at_end() {
            let ch = self.advance();
            let token = self.tokenize(ch);
            if token == TokenType::Error {
                let message = format!("Unexpected character '{}'", ch);
                lox.error(self.line, self.column, message);
            }
            self.add_token(token, ch.to_string());
        }

        let text = "".to_string();
        let token = Token::new(TokenType::Eof, text.clone(), text, self.line);
        self.tokens.push(token);
    }

    // matches on character to return `TokenType`
    fn tokenize(&mut self, ch: char) -> TokenType {
        let token = match ch {
            '(' => TokenType::LeftParen,
            ')' => TokenType::RightParen,
            '{' => TokenType::LeftBrace,
            '}' => TokenType::RightBrace,

            ',' => TokenType::Comma,
            '.' => TokenType::Dot,
            '-' => TokenType::Minus,
            '+' => TokenType::Plus,

            ';' => TokenType::Semicolon,
            '*' => TokenType::Star,
            '\n' => {
                self.line += 1;
                self.column = 0;
                TokenType::Newline
            },
            '!' => self.lookahead('=', TokenType::BangEqual, TokenType::Bang),

            '=' => self.lookahead('=', TokenType::EqualEqual, TokenType::Equal),
            '>' => self.lookahead('=', TokenType::GreaterEqual, TokenType::Greater),
            '<' => self.lookahead('=', TokenType::LessEqual, TokenType::Less),
            '/' => {
                if self.r#match('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                    TokenType::Comment
                } else {
                    TokenType::Slash
                }
            },

            '"' => self.string(),

            ' ' => TokenType::Whitespace,
            '\r' => TokenType::Whitespace,
            '\t' => TokenType::Whitespace,

            _ => {
                TokenType::Error
            }
        };

        token
    }

    fn string(&mut self) -> TokenType {
        let opening_quote = self.column;

        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            } 

            self.advance();
        }

        // FIXME: It doesn't actually work
        if (self.is_at_end()) {
            let mut lox = Lox::new();
            let message = "Unterminated string.".to_string();
            lox.error(self.line, self.column, message);
        }

        self.advance();

        let string = &self.source[opening_quote .. self.column - 1];
        TokenType::String(string.to_string())
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let current = self.current;
        let next = self.current + 1;

        self.column += 1;
        self.current += 1;

        let byte = self.source.as_bytes()
            .get(self.column - 1)
            .copied()
            .unwrap_or(b'\0') as char;

        byte
    }

    /// Returns the next character
    fn peek(&self) -> char {
        let byte = self.source.as_bytes()
            .get(self.column + 1)
            .copied()
            .unwrap_or(b'\0') as char;

        byte
    }

    /// Checks the next if the next character is equal to `expected`
    fn r#match(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            false
        } else if self.advance() != expected {
            false
        } else {
            self.current += 1;
            true
        }
    }

    /// if `expected` is the next character returns `v1` else `v2`
    fn lookahead(&mut self, expected: char, v1: TokenType, v2: TokenType) -> TokenType {
        if self.r#match(expected) {
            v1
        } else {
            v2
        }
    }

    fn add_token(&mut self, kind: TokenType, literal: String) {
        let token = Token::new(kind, literal.clone(), literal, self.line);
        self.tokens.push(token);
    }
}
