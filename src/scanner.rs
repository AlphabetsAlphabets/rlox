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

    /// Returns the token, lexeme and literal.
    pub fn to_string(self) -> String {
        format!("{} + {} + {}", self.token_kind.display(), self.lexeme, self.literal)
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
}

impl Scanner {
    pub fn new() -> Self {
        Scanner { tokens: vec![] }
    }

    pub fn eol(source: String, current: usize) -> bool {
        let con = current >= source.len();
        con
    }

    pub fn tokenize(&mut self, source: &str) {
        let Self { tokens } = self;
        let mut line = 0;

        // >= '>' -> '='
        let mut chars = source.chars().into_iter();
        for (count, literal) in chars.enumerate() {
            let token = match literal {
                '(' => Token::create_token(source, literal, TokenKind::Left_paren, line),
                ')' => Token::create_token(source, literal, TokenKind::Right_paren, line),
                ',' => Token::create_token(source, literal, TokenKind::Comma, line),
                '.' => Token::create_token(source, literal, TokenKind::Dot, line),
                '-' => Token::create_token(source, literal, TokenKind::Minus, line),
                '+' => Token::create_token(source, literal, TokenKind::Plus, line),
                ';' => Token::create_token(source, literal, TokenKind::Semicolon, line),
                '\\' => Token::create_token(source, literal, TokenKind::Backslash, line),
                '*' => Token::create_token(source, literal, TokenKind::Star, line),
                '\'' => Token::create_token(source, literal, TokenKind::Single_quote, line),
                '"' => Token::create_token(source, literal, TokenKind::Double_quote, line),
                '=' => Token::create_token(source, literal, TokenKind::Equal, line),
                '\n' => {
                    line += 1;
                    Token::create_token(source, literal, TokenKind::Newline, line)
                },
                '>' => {
                    let character = &source[count+1..count + 2];
                    let token = if character == "=" {
                        TokenKind::Greater_or_equal
                    } else {
                        TokenKind::Greater
                    };
                    Token::create_token(source, literal, token, line)
                }

                // TODO: Replace _ with error output instead
                _ => Token::create_token(source, literal, TokenKind::Error(literal, line), line),
            };

            tokens.push(token);
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
