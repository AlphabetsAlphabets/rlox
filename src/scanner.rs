use super::token_type::*;
use super::Lox;

pub struct Scanner {
    source: String,
    pub tokens: Vec<Token>,
    // Keeps track of what, and where the scanner is looking at
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
            '(' => TokenType::Left_paren,
            ')' => TokenType::Right_paren,
            '{' => TokenType::Left_brace,
            '}' => TokenType::Right_brace,
            ',' => TokenType::Comma,
            '.' => TokenType::Dot,
            '-' => TokenType::Minus,
            '+' => TokenType::Plus,
            ';' => TokenType::Semicolon,
            '*' => TokenType::Star,
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

    // r# can let you use reserved keywords as function names.
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

    fn add_token(&mut self, kind: TokenType, literal: String) {
        let text = &self.source[self.start..self.current];
        let token = Token::new(kind, text.to_string(), literal, self.line);
        self.tokens.push(token);
    }
}
