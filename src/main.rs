use std::{fs::File, path::PathBuf};

use clap::{Parser as ClapParser, Subcommand};
use interpreter::{Interpreter, SimpleInterpreter};
use lexer::Lexer;
use parser::Parser;

mod ast;
mod error;
mod interpreter;
mod lexer;
mod parser;

/// A simple Nix alternative
#[derive(ClapParser)]
#[command(name = "clap_app_example")]
#[command(version = "1.0")]
#[command(about = "An example of a simple clap app", long_about = None)]
struct Cli {
    file: PathBuf,
}

#[derive(Subcommand)]
enum Commands {}

fn main() {
    let cli = Cli::parse();

    match File::open(cli.file) {
        Ok(file) => {
            let mut lexer = Lexer::new(file);

            let expression = Parser::parse(&mut lexer).unwrap();
            // println!("Lexer errors: {:?}", lexer.errors());

            println!("{}", expression);

            let output = SimpleInterpreter::eval(expression).unwrap();

            println!("Output of expression: {}", output)
        }
        Err(err) => {
            println!("Failed to open file: {}", err);
        }
    };
}
