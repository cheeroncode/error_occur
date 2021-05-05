/*!
# Error remind module
*/
use std::{error::Error, fmt::Display};

use super::ErrorCause;

/**
## 定义基本的`错误消息`和可选的`错误原因`。
Define a basic `error message` and an optional `error cause`.
*/
#[derive(Debug)]
pub struct ErrorRemind {
    /// 错误消息
    /// Error message
    message: String,
    /// 错误原因
    /// Error cause
    cause: Option<ErrorCause>,
}

impl ErrorRemind {
    /// ## 创建新的`ErrorRemind`
    /// Create a new `ErrorRemind`
    pub fn new(message: &str) -> ErrorRemind {
        ErrorRemind {
            cause: None,
            message: message.to_string(),
        }
    }
    /// ## 添加错误的原始原因
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
