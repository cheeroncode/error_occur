use std::{error::Error, str};

mod cause;
mod remind;

#[doc(inline)]
use cause::ErrorCause;
#[doc(inline)]
pub use remind::ErrorRemind;
/**
## 定义发生错误时创建错误的方法。
Defines a method to create an error when it occurs.
*/
pub trait ErrorOccur<T, M = &'static str> {
    type Output;
    /**
    使用`message`创建错误。
    Use `message` to create the error.
    */
    fn error(self, message: M) -> Self::Output;
}

impl<T> ErrorOccur<T> for Option<T> {
    type Output = std::result::Result<T, ErrorRemind>;
    /**
    ## 如果是[`Option::None`]，使用`message`创建错误[`ErrorRemind`]。
    If it is [`Option::None`], use `message` to create the error [`ErrorRemind`].
    */
    fn error(self, message: &str) -> Self::Output {
        match self {
            Some(v) => Ok(v),
            None => Err(message.into()),
        }
    }
}

impl<T, E: Error> ErrorOccur<T> for std::result::Result<T, E> {
    type Output = std::result::Result<T, ErrorRemind>;
    /**
    ## 如果是[`Result::Err`]，使用`message`创建错误[`ErrorRemind`]。
    If it is [`Result::Err`], use `message` to create the error [`ErrorRemind`].
    */
    fn error(self, message: &str) -> Self::Output {
        match self {
            Ok(v) => Ok(v),
            Err(ref e) => Err(ErrorRemind::new(message).cause(e)),
        }
    }
}

impl ErrorOccur<bool> for bool {
    type Output = std::result::Result<bool, ErrorRemind>;
    /**
    ## 如果是[`false`]，使用`message`创建错误[`ErrorRemind`]。
    If it is [`false`], use `message` to create the error [`ErrorRemind`].
    */
    fn error(self, message: &str) -> std::result::Result<bool, ErrorRemind> {
        match self {
            true => Ok(true),
            false => Err(message.into()),
        }
    }
}

impl<T> ErrorOccur<T> for &'static str {
    type Output = std::result::Result<T, ErrorRemind>;
    /**
    ## 使用`＆'static str`直接创建错误[`ErrorRemind`]。

    Use `&'static str` to create the error [`ErrorRemind`] directly.

    It always returns the value `Err`.

    **NOTE** : So, do not use `match` for the return value, you will see the following error, and this is intentional.
    ```
    /*
    hint error:
    type annotations needed
    cannot infer type for type parameter `T` declared on the trait `ErrorOccur`rustc(E0282)
    */
    ```
    */
    fn error(self, message: &str) -> Self::Output {
        if self.trim().is_empty() {
            Err(ErrorRemind::new(&format!("{}", message)))
        } else {
            Err(ErrorRemind::new(&format!("{}, {}", self, message)))
        }
    }
}

impl<T> ErrorOccur<T> for String {
    type Output = std::result::Result<T, ErrorRemind>;
    /**
    ## 使用[`String`]直接创建错误[`ErrorRemind`]。

    Use [`String`] to create the error [`ErrorRemind`] directly.

    It always returns the value `Err`.

    **NOTE** : So, do not use `match` for the return value, you will see the following error, and this is intentional.
    ```
    /*
    type annotations needed
    cannot infer type for type parameter `T` declared on the trait `ErrorOccur`rustc(E0282)
    */
    ```
    */
    fn error(self, message: &str) -> Self::Output {
        if self.trim().is_empty() {
            Err(ErrorRemind::new(&format!("{}", message)))
        } else {
            Err(ErrorRemind::new(&format!("{}, {}", self, message)))
        }
    }
}