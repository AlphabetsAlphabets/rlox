// reference: https://gitlab.com/OptimalStrategy/rlox/-/blob/dev/src/lib/frontend/scanner.rs
use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;

mod scanner;
use scanner::Scanner;

mod token_kind;
use token_kind::TokenKind;

/// Passes the lox file to be processed by the scanenr
pub fn run_file(path: &str) {
    let path = Path::new(&path);
    let content = fs::read_to_string(path).expect("The file does not exist.");

    let mut scanner = Scanner::new(content);
    scanner.tokenize();
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

    #[test]
    fn string_error() {
        let lox = "h_i&".to_string();
        let mut scanner = Scanner::new(lox);

        scanner.tokenize();
        scanner.print();
    }

    #[test]
    fn string_ok() {
        let lox = "h_ii".to_string();
        let mut scanner = Scanner::new(lox);

        scanner.tokenize();
        scanner.print();
    }

    #[test]
    fn tokenization() {
        // designed to get a mix of errors and successes output
        let valid_lox = "&%$....()=>* Hi_there_how_are_you\n&\n&\n//comment\n".to_string();

        let mut scanner = Scanner::new(valid_lox);
        scanner.tokenize();
        scanner.print();
    }

    #[test]
    fn operators() {
        let operators = "src/tests/operators.lox";
        run_file(operators);
    }

    #[test]
    fn strings() {
        let string_file = "src/tests/string.lox";
        run_file(string_file);
    }

    #[test]
    fn lox() {
        println!("Output: ");
        let lox = "src/tests/main.lox";
        run_file(lox);
    }
}
