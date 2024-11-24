mod scanner;
mod token;

use crate::scanner::Scanner;

use std::env;
use std::fs;
use std::io::{stdin, stdout, Write};
use std::process::exit;

fn run_file(path: &str) -> Result<(), String> {
    match fs::read_to_string(path) {
        Err(message) => return Err(message.to_string()),
        Ok(contents) => return run(&contents),
    }
}

fn run_prompt() -> Result<(), String> {
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

        run(&input)?
    }
}

fn run(source: &str) -> Result<(), String> {
    let scanner = Scanner {
        source: source,
        tokens: Vec::new(),
    };
    let tokens = scanner.scan_tokens()?;

    for token in &tokens {
        println!("{:?}", token);
    }

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
