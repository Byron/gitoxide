#![forbid(unsafe_code)]
#![deny(rust_2018_idioms)]

use bstr::BString;

/// A glob pattern at a particular base path.
///
/// This closely models how patterns appear in a directory hierarchy of include or attribute files.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Pattern {
    /// the actual pattern bytes
    pub text: BString,
    /// Additional information to help accelerate pattern matching.
    pub mode: pattern::Mode,
    /// The position in `text` with the first wildcard character, or `None` if there is no wildcard at all.
    pub first_wildcard_pos: Option<usize>,
    /// The relative base at which this pattern resides, with trailing slash, using slashes as path separator.
    /// If `None`, the pattern is considered to be at the root of the repository.
    pub base_path: Option<BString>,
}

pub mod pattern;

pub mod wildmatch;
pub use wildmatch::function::wildmatch;

mod parse;
pub use parse::pattern as parse;
