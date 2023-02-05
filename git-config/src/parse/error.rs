use std::fmt::Display;

use crate::parse::Error;

/// A list of parsers that parsing can fail on. This is used for pretty-printing errors
#[derive(PartialEq, Debug, Clone, Copy)]
pub(crate) enum ParseNode {
    SectionHeader,
    Name,
    Value,
}

impl Display for ParseNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SectionHeader => write!(f, "section header"),
            Self::Name => write!(f, "name"),
            Self::Value => write!(f, "value"),
        }
    }
}

impl Error {
    /// The one-indexed line number where the error occurred. This is determined
    /// by the number of newlines that were successfully parsed.
    #[must_use]
    pub const fn line_number(&self) -> usize {
        self.line_number + 1
    }

    /// The data that was left unparsed, which contains the cause of the parse error.
    #[must_use]
    pub fn remaining_data(&self) -> &[u8] {
        &self.parsed_until
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Got an unexpected token on line {} while trying to parse a {}: ",
            self.line_number + 1,
            self.last_attempted_parser,
        )?;

        let data_size = self.parsed_until.len();
        let data = std::str::from_utf8(&self.parsed_until);
        match (data, data_size) {
            (Ok(data), _) if data_size > 10 => {
                write!(
                    f,
                    "'{}' ... ({} characters omitted)",
                    &data.chars().take(10).collect::<String>(),
                    data_size - 10
                )
            }
            (Ok(data), _) => write!(f, "'{data}'"),
            (Err(_), _) => self.parsed_until.fmt(f),
        }
    }
}

impl std::error::Error for Error {}
