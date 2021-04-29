/*!
# Error remind module
*/
use std::{error::Error, fmt::Display};

use super::ErrorCause;

/**
## Define a basic `error message` and an optional `error cause`.
*/
#[derive(Debug)]
pub struct ErrorRemind {
    /// Error message
    message: String,
    /// Error cause
    cause: Option<ErrorCause>,
}

impl ErrorRemind {
    /// Create a new `ErrorRemind`
    pub fn new(message: &str) -> ErrorRemind {
        ErrorRemind {
            cause: None,
            message: message.to_string(),
        }
    }
    /// Add the original cause of the error
    pub fn cause(self, cause: &dyn Error) -> ErrorRemind {
        ErrorRemind {
            cause: Some(ErrorCause::new(&cause.to_string())),
            ..self
        }
    }
}

impl Error for ErrorRemind {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self.cause {
            Some(cause) => Some(cause),
            None => None,
        }
    }
}

impl Display for ErrorRemind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.cause {
            Some(cause) => {
                writeln!(f, "{}, {}", self.message, cause)
            }
            None => {
                writeln!(f, "{}", self.message)
            }
        }
    }
}

impl From<&str> for ErrorRemind {
    fn from(message: &str) -> Self {
        ErrorRemind::new(message)
    }
}

impl From<&dyn Error> for ErrorRemind {
    fn from(e: &dyn Error) -> Self {
        ErrorRemind::new("意外错误").cause(e)
    }
}

use std::io::Error as IOERR;
use std::io::ErrorKind as Kind;
impl From<ErrorRemind> for IOERR {
    fn from(e: ErrorRemind) -> Self {
        IOERR::new(Kind::Other, e.to_string())
    }
}
