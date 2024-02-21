use std::fmt::{Debug, Display, Formatter};

/// A type indicating a failure to convert to `Refined`
#[derive(Debug)]
pub struct Error<T: Sized> {
    message: String,
    target: T,
}

impl<T> std::error::Error for Error<T> where T: Debug {}

impl<T> Error<T> {
    pub fn new(message: impl Into<String>, target: T) -> Self {
        Self {
            message: message.into(),
            target,
        }
    }

    pub fn into_target(self) -> T {
        self.target
    }
}

impl<T> Display for Error<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
