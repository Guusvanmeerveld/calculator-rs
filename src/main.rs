use std::{fs::File, path::PathBuf};

use clap::{Parser, Subcommand};
use lexer::{Lexer, Token};

mod error;
mod lexer;

/// A simple Nix alternative
#[derive(Parser)]
#[command(name = "clap_app_example")]
#[command(version = "1.0")]
#[command(about = "An example of a simple clap app", long_about = None)]
struct Cli {
    file: PathBuf,
    // The command to run
    // #[command(subcommand)]
    // command: Commands,
}

#[derive(Subcommand)]
enum Commands {}

fn main() {
    let cli = Cli::parse();

    match File::open(cli.file) {
        Ok(file) => {
            let mut lexer = Lexer::new(file);

            while let Some(token) = lexer.next() {
                if token != Token::Unrecognized {
                    println!("{:?}", token);
                }
            }
        }
        Err(err) => {
            println!("Failed to open file: {}", err);
        }
    };
}
