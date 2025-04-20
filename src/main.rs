use std::{
    fs::File,
    io::{self, Write, stdout},
    path::PathBuf,
};

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
#[command(name = "calculator")]
#[command(version = "1.0")]
#[command(about = "A simple calculator", long_about = None)]
struct Cli {
    file: Option<PathBuf>,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Open a REPL
    Repl,
}

fn main() {
    let cli = Cli::parse();

    if let Some(file) = cli.file {
        match File::open(file) {
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

        return;
    }

    if let Some(command) = cli.command {
        match command {
            Commands::Repl => loop {
                write!(std::io::stdout(), "$ ").expect("Failed to write to stdout");
                stdout().flush().expect("Failed to flush stdout");

                let mut buffer = String::new();

                std::io::stdin()
                    .read_line(&mut buffer)
                    .expect("Failed to read data");

                let mut lexer = Lexer::new(buffer.as_bytes());

                let expression = Parser::parse(&mut lexer).unwrap();

                let output = SimpleInterpreter::eval(expression).unwrap();

                println!("{}", output);
            },
        }
    }

    println!("You must specify either a file or a command!")
}
