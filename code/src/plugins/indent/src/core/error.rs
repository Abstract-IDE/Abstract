use std::{
    fmt,
    io, //
};

#[derive(Debug)]
pub enum IndentError {
    /// An underlying I/O error occurred (e.g., file not found, permission denied, read error).
    Io(io::Error),
    /// An invalid configuration was provided (e.g., invalid sample percentage).
    InvalidConfig(String),
}

impl fmt::Display for IndentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IndentError::Io(err) => write!(f, "I/O error: {}", err),
            IndentError::InvalidConfig(msg) => write!(f, "Invalid configuration: {}", msg),
        }
    }
}

impl std::error::Error for IndentError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            IndentError::Io(err) => Some(err),
            IndentError::InvalidConfig(_) => None,
        }
    }
}
