use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum Error {
    SerializationFailed(String),
    DeserializationFailed(String),
    Custom(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Error::SerializationFailed(err) => write!(f, "Failed to serialize the input: {}", err),
            Error::DeserializationFailed(err) => {
                write!(f, "Failed to deserialize the input: {}", err)
            }
            Error::Custom(ref err) => write!(f, "{}", err),
        }
    }
}

impl std::error::Error for Error {}
