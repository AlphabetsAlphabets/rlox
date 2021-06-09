// reference: https://gitlab.com/OptimalStrategy/rlox/-/blob/dev/src/lib/frontend/scanner.rs
use std::env;
use std::io;
use std::io::Write;
use std::path::Path;

mod scanner;
use scanner::Token;

mod token_kind;
use token_kind::TokenKind;

fn run(token: Token) {
    token.tokenize();
    if token.has_error {
        std::process::exit(1);
    }
}

fn run_file(path: &str) {
    let path = Path::new(&path);
    println!("{:?}", path);
}

fn run_prompt() {
    println!("Welcome to the rlox interactive REPL");
    let mut string = String::new();

    loop {
        print!("-> ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut string).unwrap();

        if string.trim() == "" {
            std::process::exit(1);
        }

        // tokenizing (has error)
        // let token = Token::from(string.trim());
        // token.tokenize();

        println!("{}", string);
        string.clear();
    }
}

fn spawn() {
    let args: Vec<String> = env::args().collect();
    let length = args.len();
    if length >= 2 {
        run_file(&args[1]);
    } else {
        run_prompt();
    }
}

fn main() {
}

#[cfg(test)]
mod tests {
    use super::Token;

    #[test]
    fn to_string() {
        let token = Token::default();
        let string_rep = token.to_string();
        assert_eq!(string_rep, "eof + lexeme + literal");
    }
}
