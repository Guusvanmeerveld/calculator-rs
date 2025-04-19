use std::{io, result};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("IO Error: {0}")]
    Io(#[from] io::Error),

    #[error("Syntax error: {0}")]
    SyntaxError(SyntaxError),
}

#[derive(thiserror::Error, Debug)]
pub enum SyntaxError {
    #[error("Failed to recognize token: {0}")]
    UnrecognizedToken(char),
}

pub type Result<T> = result::Result<T, Error>;
