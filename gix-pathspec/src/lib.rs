//! Parse [path specifications](https://git-scm.com/docs/gitglossary#Documentation/gitglossary.txt-aiddefpathspecapathspec) and
//! see if a path matches.
#![deny(missing_docs, rust_2018_idioms)]
#![forbid(unsafe_code)]

use bitflags::bitflags;
use bstr::BString;

///
pub mod parse;

/// The output of a pathspec [parsing][parse()] operation. It can be used to match against a one or more paths.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
pub struct Pattern {
    /// The path part of a pathspec.
    ///
    /// `:(top,literal,icase,attr,exclude)some/path` would yield `some/path`.
    pub path: BString,
    /// All magic signatures that were included in the pathspec.
    pub signature: MagicSignature,
    /// The search mode of the pathspec.
    pub search_mode: MatchMode,
    /// All attributes that were included in the `ATTR` part of the pathspec, if present.
    ///
    /// `:(attr:a=one b=):path` would yield attribute `a` and `b`.
    pub attributes: Vec<gix_attributes::Assignment>,
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
    /// See [`PathAwareGlob`][MatchMode::PathAwareGlob] for the alternative.
    ShellGlob,
    /// Special characters in the pattern, like `*` or `?`, are treated literally, effectively turning off globbing.
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
