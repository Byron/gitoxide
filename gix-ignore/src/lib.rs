//! Parse `.gitignore` files and provide utilities to match against them.
//!
//! ## Feature Flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![deny(missing_docs, rust_2018_idioms)]
#![forbid(unsafe_code)]

pub use gix_glob as glob;

///
pub mod search;
/// A grouping of lists of patterns while possibly keeping associated to their base path in order to find matches.
///
/// Pattern lists with base path are queryable relative to that base, otherwise they are relative to the repository root.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Default)]
pub struct Search {
    /// A list of pattern lists, each representing a patterns from a file or specified by hand, in the order they were
    /// specified in.
    ///
    /// When matching, this order is reversed.
    pub patterns: Vec<gix_glob::search::pattern::List<search::Ignore>>,
}

///
pub mod parse;

/// Parse git ignore patterns, line by line, from `bytes`.
pub fn parse(bytes: &[u8]) -> parse::Lines<'_> {
    parse::Lines::new(bytes)
}
