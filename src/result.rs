use std::fmt::{Debug, Display, Formatter};

/// A `Result` type alias with a custom-defined error in the `refined-type` crate
pub type Result<T> = std::result::Result<T, Error>;

/// A type indicating a failure to convert to `Refined`
#[derive(Debug)]
pub struct Error {
    message: String,
}

impl Error {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for Error {}
