use crate::parse::{Error, ParserOrIoError};
use std::fmt::Display;

/// A list of parsers that parsing can fail on. This is used for pretty-printing
/// errors
#[derive(PartialEq, Debug, Clone, Copy)]
pub(crate) enum ParserNode {
    SectionHeader,
    ConfigName,
}

impl Display for ParserNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SectionHeader => write!(f, "section header"),
            Self::ConfigName => write!(f, "config name"),
        }
    }
}

impl Error<'_> {
    /// The one-indexed line number where the error occurred. This is determined
    /// by the number of newlines that were successfully parsed.
    #[must_use]
    pub const fn line_number(&self) -> usize {
        self.line_number + 1
    }

    /// The remaining data that was left unparsed.
    #[must_use]
    pub fn remaining_data(&self) -> &[u8] {
        &self.parsed_until
    }

    /// Coerces into an owned instance. This differs from the standard [`clone`]
    /// implementation as calling clone will _not_ copy the borrowed variant,
    /// while this method will. In other words:
    ///
    /// | Borrow type | `.clone()` | `to_owned()` |
    /// | ----------- | ---------- | ------------ |
    /// | Borrowed    | Borrowed   | Owned        |
    /// | Owned       | Owned      | Owned        |
    ///
    /// This can be most effectively seen by the differing lifetimes between the
    /// two. This method guarantees a `'static` lifetime, while `clone` does
    /// not.
    ///
    /// [`clone`]: std::clone::Clone::clone
    #[must_use]
    pub fn to_owned(&self) -> Error<'static> {
        Error {
            line_number: self.line_number,
            last_attempted_parser: self.last_attempted_parser,
            parsed_until: self.parsed_until.clone().into_owned().into(),
        }
    }
}

impl Display for Error<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data_size = self.parsed_until.len();
        let data = std::str::from_utf8(&self.parsed_until);
        write!(
            f,
            "Got an unexpected token on line {} while trying to parse a {}: ",
            self.line_number + 1,
            self.last_attempted_parser,
        )?;

        match (data, data_size) {
            (Ok(data), _) if data_size > 10 => {
                write!(f, "'{}' ... ({} characters omitted)", &data[..10], data_size - 10)
            }
            (Ok(data), _) => write!(f, "'{}'", data),
            (Err(_), _) if data_size > 10 => write!(
                f,
                "'{:02x?}' ... ({} characters omitted)",
                &self.parsed_until[..10],
                data_size - 10
            ),
            (Err(_), _) => write!(f, "'{:02x?}'", self.parsed_until),
        }
    }
}

impl std::error::Error for Error<'_> {}

impl ParserOrIoError<'_> {
    /// Coerces into an owned instance. This differs from the standard [`clone`]
    /// implementation as calling clone will _not_ copy the borrowed variant,
    /// while this method will. In other words:
    ///
    /// | Borrow type | `.clone()` | `to_owned()` |
    /// | ----------- | ---------- | ------------ |
    /// | Borrowed    | Borrowed   | Owned        |
    /// | Owned       | Owned      | Owned        |
    ///
    /// This can be most effectively seen by the differing lifetimes between the
    /// two. This method guarantees a `'static` lifetime, while `clone` does
    /// not.
    ///
    /// [`clone`]: std::clone::Clone::clone
    #[must_use]
    pub fn into_owned(self) -> ParserOrIoError<'static> {
        match self {
            ParserOrIoError::Parser(error) => ParserOrIoError::Parser(error.to_owned()),
            ParserOrIoError::Io(error) => ParserOrIoError::Io(error),
        }
    }
}

impl Display for ParserOrIoError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserOrIoError::Parser(e) => e.fmt(f),
            ParserOrIoError::Io(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for ParserOrIoError<'_> {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl std::error::Error for ParserOrIoError<'_> {}
