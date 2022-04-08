#![forbid(unsafe_code)]
#![deny(rust_2018_idioms)]

use bstr::BString;

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

pub mod pattern {
    use crate::Pattern;
    use bitflags::bitflags;
    use bstr::BStr;

    bitflags! {
        #[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
        pub struct Mode: u32 {
            /// The pattern does not contain a sub-directory and - it doesn't contain slashes after removing the trailing one.
            const NO_SUB_DIR = 1 << 0;
            /// A pattern that is '*literal', meaning that it ends with what's given here
            const ENDS_WITH = 1 << 1;
            /// The pattern must match a directory, and not a file.
            const MUST_BE_DIR = 1 << 2;
            const NEGATIVE = 1 << 3;
        }
    }

    impl Pattern {
        pub fn from_bytes(text: &[u8]) -> Option<Self> {
            crate::parse::pattern(text).map(|(text, mode, first_wildcard_pos)| Pattern {
                text,
                mode,
                first_wildcard_pos,
            })
        }

        pub fn matches(&self, _value: &BStr) -> bool {
            todo!()
        }
    }
}

mod parse;
pub use parse::pattern as parse;
