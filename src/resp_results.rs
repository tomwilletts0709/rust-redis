use std::fmt;
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum RESPError {
    FromUtf8,
    IncorrectLength(usize),
    OutOfBounds(usize),
    Unknown,
    WrongType,
    ParseInt,
}

impl fmt::Display for RESPError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RESPError::OutOfBounds(index) => write!(f, "Out of bounds: {}", index),
            RESPError::IncorrectLength(length) => write!(f, "Incorrect length: {}", length),
            RESPError::FromUtf8 => write!(f, "Error converting bytes to string"),
            RESPError::Unknown => write!(f, "Unknown error"),
            RESPError::WrongType => write!(f, "Wrong type"),
            RESPError::ParseInt => write!(f, "Error parsing integer"),
        }
    }
}

impl From<FromUtf8Error> for RESPError {
    fn from(_err: FromUtf8Error) -> Self {
        Self::FromUtf8
    }
}

impl From<std::num::ParseIntError> for RESPError {
    fn from(_err: std::num::ParseIntError) -> Self {
        Self::ParseInt
    }
}

pub type RESPResult<T> = Result<T, RESPError>;