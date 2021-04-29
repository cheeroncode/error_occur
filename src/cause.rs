/*!
# Error cause module
*/
use std::{error::Error, fmt::Display};

/**
## Define the description of the original message that caused the error
*/
#[derive(Debug)]
pub struct ErrorCause {
    /// Error original description
    description: String,
}

impl ErrorCause {
    /**
    Create a new `ErrorCause`
    */
    pub fn new(description: &str) -> ErrorCause {
        ErrorCause {
            description: description.to_string(),
        }
    }
}

impl Error for ErrorCause {}

impl Display for ErrorCause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description)
    }
}
