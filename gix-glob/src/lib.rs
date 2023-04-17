//! Provide glob [`Patterns`][Pattern] for matching against paths or anything else.
//! ## Feature Flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![deny(missing_docs, rust_2018_idioms)]
#![forbid(unsafe_code)]

use bstr::BString;

/// A glob pattern optimized for matching paths relative to a root directory.
///
/// For normal globbing, use [`wildmatch()`] instead.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

pub mod search;

///
pub mod wildmatch;
pub use wildmatch::function::wildmatch;

mod parse;

/// Create a [`Pattern`] by parsing `text` or return `None` if `text` is empty.
///
/// Note that
pub fn parse(text: impl AsRef<[u8]>) -> Option<Pattern> {
    Pattern::from_bytes(text.as_ref())
}
