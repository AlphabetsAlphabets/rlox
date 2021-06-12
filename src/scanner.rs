use std::fmt;

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

/// Fields: tokens, source.
/// Tokens: Vec<Token>
/// source: String
pub struct Scanner {
    pub tokens: Vec<Token>,
    source: String,
    count: usize,
    line: usize
}

impl Scanner {
    pub fn new() -> Self {
        Scanner { 
            tokens: vec![],
            source: "".to_string(),
            count: 0,
            line: 0,
        }
    }

    pub fn eol(source: String, current: usize) -> bool {
        let con = current >= source.len();
        con
    }

    fn scan_token(&mut self, literal: char) -> TokenKind {
        let count = self.count;
        let line = self.line;
        let source = &self.source;

        match literal {
            '\'' => TokenKind::Single_quote,
            '"' => TokenKind::Double_quote,

            '(' => TokenKind::Left_paren,
            ')' => TokenKind::Right_paren,

            ',' => TokenKind::Comma,
            '.' => TokenKind::Dot,

            '-' => TokenKind::Minus,
            '+' => TokenKind::Plus,
            ';' => TokenKind::Semicolon,
            '\\' => self.lookahead(),
            '*' => TokenKind::Star,
            '=' => TokenKind::Equal,
            '\n' => {
                self.line += 1;
                TokenKind::Newline
            },

            '>' => {
                let character = &source[count+1..count + 2];
                if character == "=" {
                    TokenKind::Greater_or_equal
                } else {
                    TokenKind::Greater
                }
            },
            '<' => {
                let character = &source[count+1..count + 2];
                 if character == "=" {
                    TokenKind::Less_than_or_equal
                } else {
                    TokenKind::Less_than
                }
            },

            '!' => {
                let character = &source[count+1..count + 2];
                 if character == "=" {
                    TokenKind::Bang_equal
                } else {
                    TokenKind::Bang
                }
            },

            '{' => TokenKind::Left_brace,
            '}' => TokenKind::Left_brace,

            _ => TokenKind::Error(literal, self.line),
        }
    }

    pub fn tokenize(&mut self, source: &str) {
        let mut chars = source.chars().into_iter();
        for (count, literal) in chars.enumerate() {
            let kind = self.scan_token(literal);

            let token = Token::create_token(source, literal, kind, self.line);
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
}
