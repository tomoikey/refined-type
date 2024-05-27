use std::fmt::{Debug, Display, Formatter};

/// A type indicating a failure to convert to `Refined`
#[derive(Debug)]
pub struct Error {
    message: String,
}

impl std::error::Error for Error {}

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
