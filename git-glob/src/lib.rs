#![forbid(unsafe_code)]
#![deny(rust_2018_idioms, missing_docs)]
//! Provide glob [`Patterns`][Pattern] for matching against paths or anything else.

use bstr::BString;

/// A glob pattern optimized for matching paths relative to a root directory.
///
/// For normal globbing, use [`wildmatch()`] instead.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Pattern {
    /// the actual pattern bytes
    pub text: BString,
    /// Additional information to help accelerate pattern matching.
    pub mode: pattern::Mode,
    /// The position in `text` with the first wildcard character, or `None` if there is no wildcard at all.
    pub first_wildcard_pos: Option<usize>,
}

///
pub mod pattern;

///
pub mod wildmatch;
pub use wildmatch::function::wildmatch;

mod parse;

/// Create a [`Pattern`] by parsing `text` or return `None` if `text` is empty.
///
/// Note that
pub fn parse(text: &[u8]) -> Option<Pattern> {
    Pattern::from_bytes(text)
}
