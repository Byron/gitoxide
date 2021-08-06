use crate::parser::SectionHeaderName;
use std::fmt::Display;

/// All possible error types that may occur from interacting with
/// [`GitConfig`](super::GitConfig).
#[derive(PartialEq, Eq, Hash, Clone, PartialOrd, Ord, Debug)]
pub enum GitConfigError<'a> {
    /// The requested section does not exist.
    SectionDoesNotExist(SectionHeaderName<'a>),
    /// The requested subsection does not exist.
    SubSectionDoesNotExist(Option<&'a str>),
    /// The key does not exist in the requested section.
    KeyDoesNotExist,
    /// The conversion into the provided type for methods such as
    /// [`GitConfig::value`](super::GitConfig::value) failed.
    FailedConversion,
}

impl Display for GitConfigError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SectionDoesNotExist(s) => write!(f, "Section '{}' does not exist.", s),
            Self::SubSectionDoesNotExist(s) => match s {
                Some(s) => write!(f, "Subsection '{}' does not exist.", s),
                None => write!(f, "Top level section does not exist."),
            },
            Self::KeyDoesNotExist => write!(f, "The name for a value provided does not exist."),
            Self::FailedConversion => write!(f, "Failed to convert to specified type."),
        }
    }
}

impl std::error::Error for GitConfigError<'_> {}

#[derive(PartialEq, Eq, Hash, Clone, PartialOrd, Ord, Debug)]
pub enum GitConfigFromEnvError {
    ParseError(String),
    InvalidKeyId(usize),
    InvalidKeyValue(String),
    InvalidValueId(usize),
}
