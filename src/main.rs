use std::env;
use std::fs;
use std::io::{stdin, stdout, Write};

mod token_type;
use token_type::*;

mod scanner;
use scanner::Scanner;

pub struct Lox {
    had_error: bool,
}

impl Lox {
    fn new() -> Self {
        Lox { had_error: false }
    }

    fn main(&mut self, args: Vec<String>) {
        if args.len() > 2 {
            println!("Usage: rlox [script]");
        } else if args.len() == 2 {
            self.run_file(args[1].clone());
            if self.had_error {
                std::process::exit(65);
            }
        } else {
            self.run_prompt();
        }
    }

    // Starts the interactive repl
    fn run_prompt(&mut self) {
        let mut input = String::new();

        loop {
            print!("-> ");
            stdout().flush().unwrap();

            stdin().read_line(&mut input).unwrap();
            println!("{}", &input);

            let mut scanner = Scanner::new(input.clone());
            scanner.scan_tokens();

            input.clear();

            //  self.check_tokens(scanner);
            self.had_error = false;

        }
    }

    fn check_tokens(&self, scanner: Scanner) {
        for token in scanner.tokens {
            println!("{:?}", token);
            println!("=========");
        }
    }

    fn run(&self, source: String) {
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens();
    }

    fn run_file(&self, path: String) {
        let source = fs::read_to_string(path).unwrap();
        self.run(source);
    }

    pub fn error(&mut self, line: usize, column: usize, message: String) {
        self.report(line, column, message);
    }

    pub fn report(&mut self, line: usize, col: usize, message: String) {
        let msg = format!("[line {}, column {}] {}", line, col, message);
        self.had_error = true;
        eprintln!("{}", msg);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut lox = Lox::new();
    lox.main(args);
}
