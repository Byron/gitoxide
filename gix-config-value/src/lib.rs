//! Parsing for data types used in `git-config` files to allow their use from environment variables and other sources.
//!
//! ## Feature Flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![deny(missing_docs, rust_2018_idioms, unsafe_code)]

/// The error returned when any config value couldn't be instantiated due to malformed input.
#[derive(Debug, thiserror::Error, Eq, PartialEq)]
#[allow(missing_docs)]
#[error("Could not decode '{input}': {message}")]
pub struct Error {
    pub message: &'static str,
    pub input: bstr::BString,
    #[source]
    pub utf8_err: Option<std::str::Utf8Error>,
}

impl Error {
    /// Create a new value error from `message`, with `input` being what's causing the error.
    pub fn new(message: &'static str, input: impl Into<bstr::BString>) -> Self {
        Error {
            message,
            input: input.into(),
            utf8_err: None,
        }
    }

    pub(crate) fn with_err(mut self, err: std::str::Utf8Error) -> Self {
        self.utf8_err = Some(err);
        self
    }
}

mod boolean;
///
pub mod color;
///
pub mod integer;
///
pub mod path;

mod types;
pub use types::{Boolean, Color, Integer, Path};
