use std::error::Error;

mod cause;
mod remind;

#[doc(inline)]
use cause::ErrorCause;
#[doc(inline)]
pub use remind::ErrorRemind;
/**
## Defines a method to create [`ErrorRemind`] directly.
*/
pub trait ErrorOccur<T> {
    /// Define the error `ErrorRemind` directly.
    fn error(self, message: &str) -> std::result::Result<T, ErrorRemind>;
}

impl<T> ErrorOccur<T> for Option<T> {
    /// Define the error `ErrorRemind` directly with `Option`
    fn error(self, message: &str) -> std::result::Result<T, ErrorRemind> {
        match self {
            Some(v) => Ok(v),
            None => Err(message.into()),
        }
    }
}

impl<T, E: Error> ErrorOccur<T> for std::result::Result<T, E> {
    /// Define the error `ErrorRemind` directly with `Result`
    fn error(self, message: &str) -> Result<T, ErrorRemind> {
        match self {
            Ok(v) => Ok(v),
            Err(ref e) => Err(ErrorRemind::new(message).cause(e)),
        }
    }
}

impl ErrorOccur<bool> for bool {
    /// Define the error `ErrorRemind` directly with `bool`
    fn error(self, message: &str) -> std::result::Result<bool, ErrorRemind> {
        match self {
            true => Ok(true),
            false => Err(message.into()),
        }
    }
}

impl<T> ErrorOccur<T> for &'static str {
    /**
    Define the error `ErrorRemind` directly with `&'static str`.

    It always returns the value `Err`.

    **NOTE** : So, do not use `match` for the return value, you will see the following error, and this is intentional.
    ```
    /*
    type annotations needed
    cannot infer type for type parameter `T` declared on the trait `ErrorOccur`rustc(E0282)
    */
    ```
    */
    fn error(self, message: &str) -> Result<T, ErrorRemind> {
        if self.trim().is_empty() {
            Err(ErrorRemind::new(&format!("{}", message)))
        } else {
            Err(ErrorRemind::new(&format!("{}, {}", self, message)))
        }
    }
}

impl<T> ErrorOccur<T> for String {
    /**
    Define the error `ErrorRemind` directly with `String`.

    It always returns the value `Err`.

    **NOTE** : So, do not use `match` for the return value, you will see the following error, and this is intentional.
    ```
    /*
    type annotations needed
    cannot infer type for type parameter `T` declared on the trait `ErrorOccur`rustc(E0282)
    */
    ```
    */
    fn error(self, message: &str) -> Result<T, ErrorRemind> {
        if self.trim().is_empty() {
            Err(ErrorRemind::new(&format!("{}", message)))
        } else {
            Err(ErrorRemind::new(&format!("{}, {}", self, message)))
        }
    }
}
