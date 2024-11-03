use std::fmt::{Debug, Display, Formatter};

/// A type alias for a `Result` to use in the `Refined` module
pub type Result<T> = std::result::Result<T, Error<T>>;

/// A type indicating a failure to convert to `Refined`
#[derive(Debug)]
pub struct Error<T> {
    value: T,
    message: String,
}

impl<T> Error<T> {
    pub fn new(value: T, message: impl Into<String>) -> Self {
        Self {
            value,
            message: message.into(),
        }
    }

    pub fn into_value(self) -> T {
        self.value
    }
}

impl<T: Debug> std::error::Error for Error<T> {
    fn description(&self) -> &str {
        &self.message
    }
}

impl<T> Display for Error<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
