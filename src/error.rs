use std::{io, result};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("IO Error: {0}")]
    Io(#[from] io::Error),
}

pub type Result<T> = result::Result<T, Error>;
