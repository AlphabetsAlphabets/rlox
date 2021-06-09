use super::TokenKind;

/// Contains the token of each character
pub struct Token {
    pub has_error: bool,
    pub token: TokenKind,
    pub lexeme: String,
    pub literal: String,
    pub line: usize,
}

impl Default for Token {
    fn default() -> Self {
        Token {
            has_error: false,
            token: TokenKind::Eof,
            lexeme: "lexeme".to_string(),
            literal: "literal".to_string(),
            line: 0,
        }
    }
}

impl Token {
    /// token: TokenKind
    /// lexeme: String
    /// literal: String
    /// line: usize
    pub fn new(token: TokenKind, lexeme: String, literal: String, line: usize) -> Self {
        Token {
            has_error: false,
            token,
            lexeme,
            literal,
            line,
        }
    }

    /// Returns the token, lexeme and literal.
    pub fn to_string(self) -> String {
        format!("{} + {} + {}", self.token, self.lexeme, self.literal)
    }

    // Error "handling" or "reporting"
    fn error(mut self, line: usize, message: &str) {
        self.report(line, "", message);
    }

    fn report(mut self, line: usize, at: &str, message: &str) {
        let error = format!("[line {}] {}: {}", line, at, message);
        println!("{}", error);
        self.has_error = true;
    }
}

/// Fields: tokens, source.
/// Tokens: Vec<Token>
/// source: String
struct Scanner {
    tokens: Vec<Token>,
    source: String,
}

impl Scanner {
    fn new(source: String) -> Self {
        Scanner {
            tokens: vec![Token::new(
                TokenKind::Eof,
                "".to_string(),
                "".to_string(),
                20,
            )],
            source,
        }
    }
}

impl Scanner {
    fn eof(source: String, start: usize, current: usize, line: usize) -> bool {
       let con = current >= source.len();
       con
    }

    fn tokenize(self) {
        // TODO: Make a match function, to match against chars, and return the
        // correct token kind.
        let source = self.source;
        for literal  in source.chars() {
            let token_kind = match literal {
                '(' => TokenKind::Left_paren,
                ')' => TokenKind::Right_paren,
                _ => TokenKind::Eof,
            };

            println!("{}\n", token_kind);
        }
    }
}
