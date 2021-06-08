use crate::Token;
use crate::TokenKind;

struct Scanner {
    tokens: Vec<Token>,
    source: String
}

// Token::new(TokenKind, String, String, usize)
// Token::new(token, lexeme, literal, line)
impl Scanner {
    fn new(source: String) {
        Scanner {
            tokens: vec![Token::new(TokenKind::EOF, "".to_string(), "".to_string(), 20)]
            source
        }
    }
}
