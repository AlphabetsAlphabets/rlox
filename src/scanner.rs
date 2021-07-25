use std::collections::HashMap;
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

            match &token {
                TokenType::Error(msg) => {
                    let unexpected = b'\0' as char;
                    if msg == &unexpected.to_string() {
                        let message = format!("Unexpected character '{}'", ch);
                        lox.error(self.line, self.column, message);
                    } else {
                        lox.error(self.line, self.column, msg.to_string())
                    }
                }
                _ => {
                    let text = format!("{}", token);
                    let token = Token::new(token, text.clone(), text, self.line);
                    self.tokens.push(token);
                }
            }
        }


        let text = "EOF".to_string();
        let eof_token = Token::new(TokenType::Eof, text.clone(), text, self.line);
        self.tokens.push(eof_token);
    }

    fn tokenize(&mut self, ch: char) -> TokenType {
        match ch {
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
            }
            '!' => self.if_contains('=', TokenType::BangEqual, TokenType::Bang),

            '=' => self.if_contains('=', TokenType::EqualEqual, TokenType::Equal),
            '>' => self.if_contains('=', TokenType::GreaterEqual, TokenType::Greater),
            '<' => self.if_contains('=', TokenType::LessEqual, TokenType::Less),

            ' ' => TokenType::Whitespace,
            '\r' => TokenType::Whitespace,
            '\t' => TokenType::Whitespace,
            '/' => {
                if self.r#match('/') {
                    while self.peek() != '\n' {
                        self.advance();
                    }
                    TokenType::Comment
                } else {
                    TokenType::Slash
                }
            }

            '"' => self.string(),
            ch if ch.is_ascii_digit() => {
                let num = self.number();
                // println!("Num: {}", num);
                TokenType::Number(num)
            },

            ch if ch.is_ascii_alphabetic() || ch == '_' => {
                let start = self.current;
                while self.peek().is_alphanumeric() {
                    self.advance();
                }

                let keywords = self.get_reserved_keywords();
                let text = &self.source[start - 1..self.current];
                match keywords.get(text) {
                    Some(keyword) => keyword.clone(),
                    None => TokenType::Identifier
                }
            }

            _ => {
                let msg = b'\0' as char;
                TokenType::Error(msg.to_string())
            }
        }
    }

    fn get_reserved_keywords<'a>(&self) -> HashMap<&'a str, TokenType> {
        let mut keywords = HashMap::new();
        keywords.insert("var", TokenType::Var);
        keywords.insert("and", TokenType::And);
        keywords.insert("else", TokenType::Else);
        keywords.insert("class", TokenType::Class);

        keywords
    }

    fn string(&mut self) -> TokenType {
        let opening_quote = self.column;

        while self.peek() != '"' && !self.is_at_end() {
            self.advance();
        }

        // consume the ending quote if these is one
        self.advance();

        let quote = self.source.as_bytes()
            .get(self.current - 1)
            .copied()
            .unwrap_or(b'\0') as char;

        if quote != '"' {
            let mut lox = Lox::new();
            let message = "Unterminated string. You forgot to end the string with a '\"'".to_string();
            lox.error(self.line, self.column, message);
        }

        let string = &self.source[opening_quote..self.current - 1];
        TokenType::String(string.to_string())
    }

    fn is_digit(&self, ch: char) -> bool {
        ch >= '0' && ch <= '9'
    }

    fn number(&mut self) -> f64 {
        let start = self.current;

        // keeps consuming characters until the next character isn't a digit.
        while self.is_digit(self.peek()) {
            self.advance();
        }

        // if the next character from the current position is a dot, and the next
        // character after the dot is a digit continue
        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            // consumes the dot
            self.advance();

            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        let num = &self.source[start - 1..self.current];
        let num = num
            .trim()
            .parse::<f64>()
            .expect("Unable to convert to f64.");

        num
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            b'\0' as char
        } else {
            self.look_forward_by(1)
        }
    }

    fn look_forward_by(&self, by: usize) -> char {
        let byte = self
            .source
            .as_bytes()
            .get(self.current + by)
            .copied()
            .unwrap_or(b'\0') as char;

        byte
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let byte = self
            .source
            .as_bytes()
            .get(self.current)
            .copied()
            .unwrap_or(b'\0') as char;

        self.current += 1;
        self.column += 1;
        byte
    }

    fn peek(&self) -> char {
        let byte = self
            .source
            .as_bytes()
            .get(self.current)
            .copied()
            .unwrap_or(b'\0') as char;

        byte
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
    fn if_contains(&mut self, expected: char, v1: TokenType, v2: TokenType) -> TokenType {
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
