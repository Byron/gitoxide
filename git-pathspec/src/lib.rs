#![forbid(unsafe_code, rust_2018_idioms)]

use bitflags::bitflags;
use bstr::BString;

pub mod parse;
pub mod parse_functional;

/// The output of a pathspec parsing operaion. It can be used to matche against a path / multiple paths.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
pub struct Pattern {
    /// The path part of a pathspec.
    pub path: BString,
    /// All magig sigantures that were included in the pathspec.
    pub signature: Option<MagicSignature>,
    /// All attributes that were included in the `ATTR` part of the pathspec, if present.
    pub attributes: Vec<(BString, git_attributes::State)>,
}

bitflags! {
    pub struct MagicSignature: u32 {
        /// Matches patterns from the root of the repository
        const TOP = 1 << 0;
        /// Special characters in the pattern, like '*' or '?', are treated literally
        const LITERAL = 1 << 1;
        /// Matches patterns in case insensitive mode
        const ICASE = 1 << 2;
        /// A single '*' will not match a '/' in the pattern, but a '**' will
        const GLOB = 1 << 3;
        /// Specifies a list of attribute requirements that the matches should meet
        const ATTR = 1 << 4;
        /// Excludes the matching patterns from the previous results
        const EXCLUDE = 1 << 5;
    }
}

/// Parse a git-style pathspec into a `Pattern`
pub fn parse(input: &[u8]) -> Result<Pattern, parse::Error> {
    Pattern::from_bytes_functional(input)
}
