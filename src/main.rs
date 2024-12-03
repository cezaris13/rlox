mod expr;
mod interpreter;
mod parser;
mod scanner;
mod stmt;
mod token;

use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::scanner::Scanner;

use std::env;
use std::fs;
use std::io::{stdin, stdout, Write};
use std::process::exit;

fn run_file(path: &str) -> Result<(), String> {
    let mut interpreter: Interpreter = Interpreter::new();
    match fs::read_to_string(path) {
        Err(message) => return Err(message.to_string()),
        Ok(contents) => return run(&mut interpreter, &contents),
    }
}

fn run_prompt() -> Result<(), String> {
    let mut interpreter: Interpreter = Interpreter::new();
    let mut input: String;

    loop {
        input = String::new();
        print!("> ");
        let _ = stdout().flush();

        stdin()
            .read_line(&mut input)
            .expect("Did not enter a correct string");

        if let Some('\n') = input.chars().next_back() {
            input.pop();
        }
        if let Some('\r') = input.chars().next_back() {
            input.pop();
        }

        if input == "" {
            break Ok(());
        }

        match run(&mut interpreter, &input) {
            Ok(_) => (),
            Err(message) => println!("{}", message),
        }
    }
}

fn run(interpreter: &mut Interpreter, source: &str) -> Result<(), String> {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens()?;

    let mut parser = Parser::new(tokens);
    let statements = parser.parse()?;

    interpreter.interpret_statements(statements)?;

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage OListp [script]");
        exit(64);
    } else if args.len() == 2 {
        match run_file(&args[1]) {
            Err(message) => {
                println!("Error: {}", message);
                exit(1);
            }
            _ => exit(0),
        }
    } else {
        match run_prompt() {
            Err(message) => {
                println!("Error: {}", message);
                exit(1);
            }
            _ => exit(0),
        }
    }
}
