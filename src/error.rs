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

    #[error("Operator failed: {0}")]
    OperatorError(#[from] derive_more::BinaryError),
}

#[derive(thiserror::Error, Debug)]
pub enum SyntaxError {
    #[error("Failed to recognize token: {0}")]
    UnrecognizedToken(char),

    #[error("Failed to parse float: {0}")]
    ParseFloat(#[from] ParseFloatError),

    #[error("Failed to parse integer: {0}")]
    ParseInt(#[from] ParseIntError),
}

pub type Result<T> = result::Result<T, Error>;
