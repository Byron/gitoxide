//! Parse [path specifications](https://git-scm.com/docs/gitglossary#Documentation/gitglossary.txt-aiddefpathspecapathspec) and
//! see if a path matches.
#![forbid(unsafe_code, rust_2018_idioms)]
#![deny(missing_docs)]

use bitflags::bitflags;
use bstr::BString;
use git_attributes::Assignment;

///
pub mod parse;

/// The output of a pathspec parsing operation. It can be used to match against a path / multiple paths.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
pub struct Pattern {
    /// The path part of a pathspec.
    pub path: BString,
    /// All magic signatures that were included in the pathspec.
    pub signature: MagicSignature,
    /// The search mode of the pathspec.
    pub search_mode: MatchMode,
    /// All attributes that were included in the `ATTR` part of the pathspec, if present.
    pub attributes: Vec<Assignment>,
}

bitflags! {
    /// Flags to represent 'magic signatures' which are parsed behind colons, like `:top:`.
    pub struct MagicSignature: u32 {
        /// Matches patterns from the root of the repository
        const TOP = 1 << 0;
        /// Matches patterns in case insensitive mode
        const ICASE = 1 << 1;
        /// Excludes the matching patterns from the previous results
        const EXCLUDE = 1 << 2;
    }
}

/// Parts of [magic signatures][MagicSignature] which don't stack as they all configure
/// the way path specs are matched.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
pub enum MatchMode {
    /// Expand special characters like `*` similar to how the shell would do it.
    ///
    /// See [`PathAwareGlob`][SearchMode::PathAwareGlob] for the alternative.
    ShellGlob,
    /// Special characters in the pattern, like `*` or `?`, are treated literally
    Literal,
    /// A single `*` will not match a `/` in the pattern, but a `**` will
    PathAwareGlob,
}

impl Default for MatchMode {
    fn default() -> Self {
        MatchMode::ShellGlob
    }
}

/// Parse a git-style pathspec into a [`Pattern`][Pattern].
pub fn parse(input: &[u8]) -> Result<Pattern, parse::Error> {
    Pattern::from_bytes(input)
}
