use std::{fmt, io};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    IoError(#[from] io::Error),
    FmtError(#[from] fmt::Error),
    ValidationError(String),
    CommandError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::IoError(e) => write!(f, "I/O Error: {}", e),
            Error::FmtError(e) => write!(f, "Formatting Error: {}", e),
            Error::ValidationError(msg) => write!(f, "Validation Error: {}", msg),
            Error::CommandError(msg) => write!(f, "Command Error: {}", msg),
        }
    }
}
