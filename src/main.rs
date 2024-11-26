mod expr;
mod scanner;
mod token;

use crate::expr::Expression;
use crate::expr::Expression::*;
use crate::expr::LiteralValue::*;
use crate::scanner::Scanner;
use crate::token::Token;
use crate::token::TokenType::*;

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
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens()?;

    for token in &tokens {
        println!("{:?}", token);
    }

    Ok(())
}

fn main() {
    // region testing AST

    let expression: Expression = Binary {
        left: Box::new(Literal {
            value: IntValue(12),
        }),
        operator: Token::new(STAR, String::from("*"), None, 1),
        right: Box::new(Expression::Literal {
            value: IntValue(12),
        }),
    };

    expression.print();

    // endregion

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
