use super::token_type::*;

struct Scanner {
    source: String,
    tokens: Vec<Token>,
    // Keeps track of what the scanner is looking at
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    fn new(source: String) -> Self {
        let text = "".to_string();
        let token = Token::new(TokenType::Dot, text, text, 0);
        Scanner {
            source,
            tokens: vec![token],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            let ch = self.advance();
            vec![Token::default()]
        }

        let text = "".to_string();
        let token = Token::new(TokenType::Eof, text, text, 0,);

        self.tokens.push(token);
        vec![Token::default()]
    }

    fn tokenize(&self, ch: char) -> TokenType {
        match ch {
            '(' => TokenType::Left_paren,
            _ => TokenType::Eof
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let current = self.current;
        let next = self.current + 1;

        let source_code = self.source.as_bytes();
        let byte = &source_code[current..next];
        char::from(byte[0])
    }

    fn add_token(&mut self, kind: TokenType, literal: String) {
        let text = &self.source[self.start..self.current];
        let token = Token::new(kind, text.to_string(), literal, self.line);
        self.tokens.push(token);
    }
}
