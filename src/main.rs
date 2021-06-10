// reference: https://gitlab.com/OptimalStrategy/rlox/-/blob/dev/src/lib/frontend/scanner.rs
use std::env;
use std::io;
use std::io::Write;
use std::path::Path;

mod scanner;
use scanner::Scanner;
use scanner::Token;

mod token_kind;
use token_kind::TokenKind;

fn run_file(path: &str) {
    let path = Path::new(&path);
    println!("{:?}", path);

    // load the file, then the contents of the file into tokenize()

    // token.tokenize();
    // if token.has_error {
    //     std::process::exit(1);
    // }
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
    spawn();
}

#[cfg(test)]
mod tests {
    use super::Scanner;
    use super::Token;

    #[test]
    fn to_string() {
        let token = Token::default();
        let string_rep = token.to_string();
        println!("{}", &string_rep);
        assert_eq!(string_rep, "eof + lexeme + literal");
    }

    #[test]
    fn tokenization() {
        let valid_lox = r"('s...d,2\\.*aslkdj');
        (';.,2,4,5.;
         "
        .to_string();

        let mut scanner = Scanner::new();
        scanner.tokenize(valid_lox);
        for token in scanner.tokens {
            println!("{}", &token.token_kind);
        }

        assert_eq!(true, true);
    }
}
