// reference: https://gist.github.com/rust-play/3df0d4653d553f1deb9b211227fee840
use super::TokenKind;

/// literal is the string itself
/// lexeme is a character in a string
#[derive(Clone, Debug)]
pub struct Token {
    pub token_kind: TokenKind,
    pub literal: String,
    pub lexeme: String,
    pub line_number: usize,
}

impl Token {
    /// token_kind: TokenKind
    /// literal: String
    /// lexeme: String, should be a char but String is easier.
    /// line_number: usize
    pub fn new(token_kind: TokenKind, lexeme: String, literal: String, line_number: usize) -> Self {
        Token {
            token_kind,
            literal,
            lexeme,
            line_number,
        }
    }

    // Creates tokens during tokenization.
    fn create_token(literal: &str, lexeme: char, token: TokenKind, line_number: usize) -> Token {
        let literal = literal.to_string();
        let lexeme = lexeme.to_string();
        let token = Token::new(token, lexeme, literal, line_number);

        token
    }
}

/// Tokens: Vec<Token>, vec of tokens, to be used.
/// source: String, the entire lox file
/// column: usize, keeps track of which column the scanner is at in a line
/// line: usize, current line.
pub struct Scanner {
    pub tokens: Vec<Token>,
    pub source: String,
    column: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        let scanner = Scanner {
            tokens: vec![],
            source,
            column: 0,
            line: 1,
        };

        scanner
    }
}

impl Scanner {
    fn is_at_end(&self) -> bool {
        self.column >= self.source.len()
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
            '/' => {
                if self.peek() == '/' {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                        self.column += 1;
                    }
                    TokenKind::Comment
                } else {
                    TokenKind::Backslash
                }
            }

            ' ' => TokenKind::Whitespace,
            '*' => TokenKind::Star,
            '=' => TokenKind::Equal,

            literal if literal == '\n' || literal == '\r' => {
                self.line += 1;
                TokenKind::Newline
            }

            '>' => self.lookahead('=', TokenKind::GreaterOrEqual, TokenKind::Greater),
            '<' => self.lookahead('=', TokenKind::LessThanOrEqual, TokenKind::LessThan),
            '!' => self.lookahead('=', TokenKind::BangEqual, TokenKind::Bang),

            '{' => TokenKind::LeftBrace,
            '}' => TokenKind::RightBrace,

            // TODO: `>!{Hi_there` extract "Hi_there" from this.
            literal if literal.is_ascii_alphabetic() || literal == '_' => {
                let mut string: Vec<char> = vec![];

                // Checks if the next character is an alphabet, number, or underscore
                // then increments the column, to check the next character
                while self.peek().is_alphanumeric() || self.peek() == '_' {
                    let ch = self.peek();
                    string.push(ch);
                    self.column += 1;
                }

                TokenKind::String(string.iter().collect())
            }

            _ => TokenKind::Error(literal, self.line),
        }
    }

    fn advance(&mut self) -> char {
        self.column += 1;
        self.previous()
    }

    fn previous(&self) -> char {
        self.source
            .as_bytes()
            .get(self.column - 1)
            .copied()
            .unwrap_or(b'\0') as char
    }

    fn peek(&self) -> char {
        self.source
            .as_bytes()
            .get(self.column)
            .copied()
            .unwrap_or(b'\0') as char
    }

    pub fn tokenize(&mut self) {
        let source = self.source.clone();
        let chars = source.chars().into_iter();
        for literal in chars {
            let kind = self.scan_token(literal);

            let token = Token::create_token(&source, literal, kind, self.line);
            self.column += 1;
            self.tokens.push(token);
        }
    }

    pub fn print(&mut self) {
        let tokens = &self.tokens;
        for token in tokens {
            let s = token.token_kind.display();

            match s {
                Ok(text) => {
                    if !text.is_empty() {
                        println!("{}", text);
                    }
                }

                Err(err) => println!("{}", err),
            };
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
