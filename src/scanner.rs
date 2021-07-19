use super::token_type::*;
use super::Lox;

pub struct Scanner {
    source: String,
    pub tokens: Vec<Token>,
    // These three fields keeps track of what, and where the scanner is looking at
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            let ch = self.advance();
            let token = self.tokenize(ch);
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
                TokenType::Newline
            },
            '!' => self.lookahead('=', TokenType::BangEqual, TokenType::BangEqual),

            '=' => self.lookahead('=', TokenType::EqualEqual, TokenType::Equal),
            '>' => self.lookahead('=', TokenType::GreaterEqual, TokenType::Greater),
            '<' => self.lookahead('=', TokenType::LessEqual, TokenType::Less),
            _ => {
                // error reporting here
                TokenType::Error
            }
        };

        token
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let current = self.current;
        let next = self.current + 1;

        let source_code = self.source.as_bytes();
        let byte = &source_code[current..next];

        self.current += 1;
        char::from(byte[0])
    }

    fn char_at(&self) -> char {
        let current = self.current;
        let next = self.current + 1;

        let source_code = self.source.as_bytes();
        let byte = &source_code[current..next];
        char::from(byte[0])
    }

    // this is equivalent to `match` in the tutorial
    fn peek(&mut self, expected: char) -> bool {
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
        if self.peek(expected) {
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
