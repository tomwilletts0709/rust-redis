use std::fmt;

#[derive(Debug)]
pub enum StorageError {
    IncorrectRequest, 
    CommandNotAvailable(String),
    CommandSyntaxError(String),
    CommandInternalError(String),
}

impl fmt::Display for StorageError {
    fn fmt (&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StorageError::IncorrectRequest => {
                write!(f, "the client sent an incorrect request")
            }
            StorageError::CommandNotAvailable(c) => {
                write!(f, "The requested command {} is not available!", c)
            }
            StorageError::CommandSyntaxError(c) => {
                write!(f, "The requested command {} has a syntax error!", c)
            }
            StorageError::CommandInternalError(c) => {
                write!(f, "The requested command {} has an internal error!", c)
            }
        }
    }
}

pub type StorageResult<T> = Result <T, StorageError>'

