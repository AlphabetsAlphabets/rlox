use std::env;
use std::fs;
use std::io::{stdin, stdout, Write};
use std::path::Path;

mod token_type;
use token_type::*;

mod scanner;

struct Lox {
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

    fn run_prompt(&mut self) {
        let mut input = String::new();

        loop {
            print!("-> ");
            stdout().flush().unwrap();

            stdin().read_line(&mut input).unwrap();

            println!("{}", &input);
            self.run(input.clone());
            self.had_error = false;
            input.clear();
        }
    }

    fn run(&self, source: String) {
        // Scanner:new();
    }

    fn run_file(&self, path: String) {
        let source = fs::read_to_string(path).unwrap();
        self.run(source);
    }

    fn error(&mut self, line: usize, message: String) {
        self.report(line, "".to_string(), message);
    }

    fn report(&mut self, line: usize, col: String, message: String) {
        let msg = format!("[line {}] Error {}: {}", line, col, message);
        self.had_error = true;
        eprintln!("{}", msg);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let lox = Lox::new();
    lox.main(args);
}
