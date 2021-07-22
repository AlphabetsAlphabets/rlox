// TODO
// Error handling is weird because it repeats with characters from the first line
// of the file

use super::token_type::*;
use super::Lox;

pub struct Scanner {
    source: String,
    pub tokens: Vec<Token>,
    /// The column the scanner is on
    column: usize,
    /// Where the scanner is looking at relative to the end of the file
    current: usize,
    /// The current line the scanner is on
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
                TokenType::Newline
            },
            '!' => self.lookahead('=', TokenType::BangEqual, TokenType::BangEqual),

            '=' => self.lookahead('=', TokenType::EqualEqual, TokenType::Equal),
            '>' => self.lookahead('=', TokenType::GreaterEqual, TokenType::Greater),
            '<' => self.lookahead('=', TokenType::LessEqual, TokenType::Less),
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
            };

            self.advance();
        }

        if (self.is_at_end()) {
            let mut lox = Lox::new();
            let message = "Unterminated string.".to_string();
            lox.error(self.line, self.column, message);
        }

        self.advance();

        let string = &self.source[opening_quote .. self.column - 1];
        TokenType::String(string.to_string())
    }

    fn is_digit(&self, ch: char) -> bool {
        ch >= '0' && ch <= '9'
    }

    fn number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && self.is_digit(self.peek()) {
            self.advance();

            while self.is_digit(self.peek()) {
                self.advance();
            }
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    // FIXME: Only keeps track of the first character in the entire file
    fn advance(&mut self) -> char {
        let current = self.current;
        let next = self.current + 1;

        let source_code = self.source.as_bytes();
        let byte = &source_code[current..next];

        self.column += 1;
        self.current += 1;

        char::from(byte[0])
    }

    fn peek(&self) -> char {
        let source_code = self.source.as_bytes();
        let byte = &source_code[self.current..self.current + 1];

        char::from(byte[0])
    }

    // this is equivalent to `match` in the tutorial
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
