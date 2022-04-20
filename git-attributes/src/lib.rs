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

/// A way to describe the combination of files of a given kind one wants to instantiate, with support for additional
/// overrides as passed by a command-line.
pub struct Description<T: description::Kind> {
    /// Paths to files whose patterns don't have a root directory, thus apply as if the file was at the root of the repository.
    pub global_files: Vec<PathBuf>,
    /// Paths to files whose patterns apply only to the directory they themselves are in.
    /// They have higher priority than `global_files`.
    pub per_directory_files: Vec<PathBuf>,
    /// Additional pattern/data combinations that should have the highest priority and apply from the root of the repository.
    /// These are unparsed as of yet and will be parsed later when preparing the files.
    pub global_overrides: Vec<(BString, T::Value)>,
}

///
pub mod description {
    use crate::Description;

    /// A marker trait to identify the type of a description.
    pub trait Kind {
        /// The value associated with a pattern.
        type Value;
    }

    /// Identify ignore patterns.
    pub struct Ignore;
    impl Kind for Ignore {
        type Value = ();
    }

    /// Identify patterns with attributes.
    pub struct Attributes;
    impl Kind for Attributes {
        /// TODO: identify the actual value, should be name/State pairs, but there is the question of storage.
        type Value = ();
    }

    impl Description<Ignore> {}
}

pub type Files = Description<description::Attributes>;
pub type IgnoreFiles = Description<description::Ignore>;

pub mod parse;

pub fn parse(buf: &[u8]) -> parse::Lines<'_> {
    parse::Lines::new(buf)
}
