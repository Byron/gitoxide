#![forbid(unsafe_code)]
#![deny(rust_2018_idioms)]

use bstr::{BStr, BString};
use std::path::PathBuf;

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum State<'a> {
    /// The attribute is listed, or has the special value 'true'
    Set,
    /// The attribute has the special value 'false', or was prefixed with a `-` sign.
    Unset,
    /// The attribute is set to the given value, which followed the `=` sign.
    /// Note that values can be empty.
    #[cfg_attr(feature = "serde1", serde(borrow))]
    Value(&'a BStr),
    /// The attribute isn't mentioned with a given path or is explicitly set to `Unspecified` using the `!` sign.
    Unspecified,
}

/// A grouping of lists of patterns while possibly keeping associated to their base path.
///
/// Patterns with base path are queryable relative to that base, otherwise they are relative to the repository root.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Default)]
pub struct MatchGroup<T: match_group::Tag = Attributes> {
    /// A list of pattern lists, each representing a patterns from a file or specified by hand, in the order they were
    /// specified in.
    ///
    /// During matching, this order is reversed.
    pub patterns: Vec<PatternList<T>>,
}

/// A list of patterns with an optional names, for matching against it.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
pub struct PatternList<T: match_group::Tag> {
    /// Patterns and their associated data in the order they were loaded in or specified,
    /// the line number in its source file or its sequence number (_`(pattern, value, line_number)`_).
    ///
    /// During matching, this order is reversed.
    pub patterns: Vec<(git_glob::Pattern, T::Value, usize)>,

    /// The path from which the patterns were read, or `None` if the patterns
    /// don't originate in a file on disk.
    source: Option<PathBuf>,

    /// The parent directory of source, or `None` if the patterns are _global_ to match against the repository root.
    /// It's processed to contain slashes only and to end with a trailing slash, and is relative to the repository root.
    base: Option<BString>,
}

mod match_group;
pub use match_group::{Attributes, Ignore, Match, Tag};

pub mod parse;

pub fn parse(buf: &[u8]) -> parse::Lines<'_> {
    parse::Lines::new(buf)
}
