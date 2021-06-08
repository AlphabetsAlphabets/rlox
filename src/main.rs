// reference: https://gitlab.com/OptimalStrategy/rlox/-/blob/dev/src/lib/frontend/scanner.rs

use std::env;
use std::io;
use std::io::Write;
use std::path::Path;

pub enum TokenKind {
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One or two character tokens.
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,

    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
    Blank,
}

pub struct Token {
    has_error: bool,
    token: TokenKind,
    lexeme: String,
    literal: String,
    line: usize,
}

impl From<&str> for Token {
    fn from(s: &str) -> Self {
        Token {
            has_error: false,
            token: TokenKind::Blank,
            lexeme: "".to_string(),
            literal: "".to_string(),
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

    // Function that returns a string

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

impl Token {
    fn tokenize(&self) -> Vec<Self> {
        let at_end = false;
        let mut tokens = vec![];
        while !at_end {
            self.tokenize();
        }

        tokens.push(Token::new(TokenKind::EOF, "".to_string(), "".to_string(), 0));
        tokens
    }
}

fn run(token: Token) {
    token.tokenize();
    if token.has_error {
        std::process::exit(1);
    }
}

fn run_file(path: &str) {
    let path = Path::new(&path);
}

fn run_prompt() {
    let mut string = String::new();

    loop {
        print!("-> ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut string).unwrap();

        if string.trim() == "" {
            std::process::exit(1);
        }

        // tokenizing
        let token = Token::from(string.trim());
        token.tokenize();

        println!("{}", string);
        string.clear();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let length = args.len();
    if length > 2 || length == 1 {
        println!("Usage: rlox [script]");
        std::process::exit(0);
    } else if length == 2 {
        run_file(&args[1]);
    } else {
        // interactive session or REPL
    }
}

