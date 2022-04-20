#![forbid(unsafe_code)]
#![deny(rust_2018_idioms)]

use bstr::{BStr, BString};

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
#[derive(Debug, Clone)]
pub struct MatchGroup<T: description::Tag> {
    /// A list of pattern lists, each representing a patterns from a file or specified by hand, in the order they were
    /// specified in.
    ///
    /// During matching, this order is reversed.
    pub patterns: Vec<PatternList<T>>,
}

/// A list of patterns with an optional names, for matching against it.
#[derive(Debug, Clone)]
pub struct PatternList<T: description::Tag> {
    /// Patterns and their associated data in the order they were loaded in or specified.
    ///
    /// During matching, this order is reversed.
    pub patterns: Vec<(git_glob::Pattern, T::Value)>,

    /// The path at which the patterns are located in a format suitable for matches, or `None` if the patterns
    /// are relative to the worktree root.
    _base: Option<BString>,
}

mod description {
    /// A marker trait to identify the type of a description.
    pub trait Tag {
        /// The value associated with a pattern.
        type Value: std::fmt::Debug + Clone;
    }

    /// Identify ignore patterns.
    #[derive(Debug)]
    pub struct Ignore;
    impl Tag for Ignore {
        type Value = ();
    }

    /// Identify patterns with attributes.
    #[derive(Debug)]
    pub struct Attributes;
    impl Tag for Attributes {
        /// TODO: identify the actual value, should be name/State pairs, but there is the question of storage.
        type Value = ();
    }
}
pub use description::{Attributes, Ignore, Tag};

pub type Files = MatchGroup<Attributes>;
pub type IgnoreFiles = MatchGroup<Ignore>;

pub mod parse;

pub fn parse(buf: &[u8]) -> parse::Lines<'_> {
    parse::Lines::new(buf)
}
