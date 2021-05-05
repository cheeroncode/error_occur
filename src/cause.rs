/*!
# Error cause module
*/
use std::{error::Error, fmt::Display};

/**
## 定义导致错误的原始消息的描述
Define the description of the original message that caused the error
*/
#[derive(Debug)]
pub struct ErrorCause {
    /// 错误的原始描述 
    /// Error original description
    description: String,
}

impl ErrorCause {
    /**
    ## 创建新的`ErrorCause`
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
