// reference: https://gitlab.com/OptimalStrategy/rlox/-/blob/dev/src/lib/frontend/scanner.rs
use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;

mod scanner;
use scanner::Scanner;
use scanner::Token;

mod token_kind;
use token_kind::TokenKind;

pub fn run_file(path: &str) {
    let path = Path::new(&path);
    let content = fs::read_to_string(path).expect("The file does not exist.");

    let mut scanner = Scanner::new(content);
    scanner.print();
}

fn repl() {
    println!("Welcome to the rlox interactive REPL");

    let mut string = String::new();
    let mut scanner = Scanner::new("".to_string());
    loop {
        print!("-> ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut string).unwrap();
        scanner.source = string.clone();
        string.clear();

        scanner.tokenize();
        scanner.print();

        // eval here

        scanner.tokens.clear();
    }
}

fn spawn() {
    let args: Vec<String> = env::args().collect();
    let length = args.len();
    if length >= 2 {
        run_file(&args[1]);
    } else {
        repl();
    }
}

fn main() {
    spawn();
}

#[cfg(test)]
mod tests {
    use super::run_file;
    use super::Scanner;
    use super::Token;

    #[test]
    fn tokenization() {
        let valid_lox = r"('s...d,2\\.*aslkdj');
        (';.,2,4,5.;
         " .to_string();

        let mut scanner = Scanner::new(valid_lox);
        scanner.tokenize();
        &scanner.print();

        assert_eq!(true, true);
    }

    #[test]
    fn operators() {
        let operators = "src/tests/operators.lox";
        run_file(operators);
    }

    #[test]
    fn error() {
        let error_file = "src/tests/errors.lox";
        run_file(error_file);
    }
}
