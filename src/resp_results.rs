use std::fmt;

#[derive(Debug)]
pub enum RESPError {
    FromUtf8,
    OutOfBounds(usize),
    Unknown,
    WrongType
}

impl fmt::Display for RESPError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RESPError::OutOfBounds(index) => write!(f, "Out of bounds: {}", index),
            RESPError::FromUtf8 => write!(f, "Error converting bytes to string"),
            RESPError::Unknown => write!(f, "Unknown error"),
            RESPError::WrongType => write!(f, "Wrong type"),
        }
    }
}

pub type RESPResult<T> = Result<T, RESPError>;