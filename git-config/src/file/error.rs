use std::{error::Error, fmt::Display};

use crate::parser::SectionHeaderName;

/// All possible error types that may occur from interacting with
/// [`GitConfig`](super::GitConfig).
#[allow(clippy::module_name_repetitions)]
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

impl Error for GitConfigError<'_> {}

/// Represents the errors that may occur when calling [`GitConfig::from_env`].
///
/// [`GitConfig::from_env`]: crate::file::GitConfig::from_env
#[allow(clippy::module_name_repetitions)]
#[derive(PartialEq, Eq, Hash, Clone, PartialOrd, Ord, Debug)]
pub enum GitConfigFromEnvError {
    /// `GIT_CONFIG_COUNT` was not a positive integer
    ParseError(String),
    /// `GIT_CONFIG_KEY_<n>` was not set.
    InvalidKeyId(usize),
    /// An key at `GIT_CONFIG_KEY_<n>` was found, but it wasn't a valid string.
    InvalidKeyValue(usize, String),
    /// `GIT_CONFIG_VALUE_<n>` was not set.
    InvalidValueId(usize),
}

impl Display for GitConfigFromEnvError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GitConfigFromEnvError::ParseError(err) => write!(f, "GIT_CONFIG_COUNT was not a positive integer: {}", err),
            GitConfigFromEnvError::InvalidKeyId(key_id) => write!(f, "GIT_CONFIG_KEY_{} was not set.", key_id),
            GitConfigFromEnvError::InvalidKeyValue(key_id, key_val) => {
                write!(f, "GIT_CONFIG_KEY_{} was set to an invalid value: {}", key_id, key_val)
            }
            GitConfigFromEnvError::InvalidValueId(value_id) => write!(f, "GIT_CONFIG_VALUE_{} was not set.", value_id),
        }
    }
}

impl Error for GitConfigFromEnvError {}
