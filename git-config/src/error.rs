use std::fmt::{self, Display};

#[cfg(feature = "serde")]
use serde::{de, ser};

pub type Result<T> = std::result::Result<T, Error>;

// This is a bare-bones implementation. A real library would provide additional
// information in its error type, for example the line and column at which the
// error occurred, the byte offset into the input, or the current key being
// processed.
#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    Message(String),
    Eof,
    InvalidInteger,
    InvalidBoolean(String),
}

#[cfg(feature = "serde")]
impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

#[cfg(feature = "serde")]
impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Message(msg) => formatter.write_str(msg),
            Error::Eof => formatter.write_str("unexpected end of input"),
            Error::InvalidInteger => formatter.write_str("invalid integer given"),
            Error::InvalidBoolean(_) => formatter.write_str("invalid boolean given"),
        }
    }
}

impl std::error::Error for Error {}
