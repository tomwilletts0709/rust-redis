use std::fmt;
use std::io;

#[derive(Debug)]
pub enum Error {
    // RESP parsing errors
    ProtocolError(String),
    WrongType(String),
    InvalidCommand(String),
    KeyNotFound(String),
    OutOfMemory(String),
    OutOfBounds(usize),
    ParseInt(String),
    Io(io::Error),
    Sync(String),
    Timeout(std::time::Duration),
    Other(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ProtocolError(e) => write!(f, "Protocol error: {}", e),
            Self::WrongType(e) => write!(f, "Wrong type: {}", e),
            Self::InvalidCommand(e) => write!(f, "Invalid command: {}", e),
            Self::KeyNotFound(e) => write!(f, "Key not found: {}", e),
            Self::OutOfMemory(e) => write!(f, "Out of memory: {}", e),
            Self::OutOfBounds(e) => write!(f, "Out of bounds: {}", e),
            Self::ParseInt(e) => write!(f, "Parse int: {}", e),
            Self::Io(e) => write!(f, "IO error: {}", e),
            Self::Sync(e) => write!(f, "Sync error: {}", e),
            Self::Timeout(e) => write!(f, "Timeout: {:?}", e),
            Self::Other(e) => write!(f, "Other error: {}", e),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}
