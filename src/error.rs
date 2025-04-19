use std::{
    io,
    num::{ParseFloatError, ParseIntError},
    result,
};

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

    #[error("Failed to parse float: {0}")]
    ParseFloat(String, ParseFloatError),

    #[error("Failed to parse integer: {0}")]
    ParseInteger(String, ParseIntError),
}

pub type Result<T> = result::Result<T, Error>;
