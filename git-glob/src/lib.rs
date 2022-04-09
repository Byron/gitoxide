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
            /// The pattern matches, but should be negated. Note that this mode has to be checked and applied by the caller.
            const NEGATIVE = 1 << 3;
        }
    }
    bitflags! {
        #[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
        pub struct MatchOptions: u8 {
            /// Let globs not match the slash `/` literal.
            const SLASH_IS_LITERAL = 1 << 0;
            /// Match case insensitively for ascii characters only.
            const IGNORE_CASE = 1 << 1;
        }
    }

    pub enum Case {
        /// The case affects the match
        Sensitive,
        /// Ignore the case of ascii characters.
        Fold,
    }

    impl Pattern {
        pub fn from_bytes(text: &[u8]) -> Option<Self> {
            crate::parse::pattern(text).map(|(text, mode, first_wildcard_pos)| Pattern {
                text,
                mode,
                first_wildcard_pos,
            })
        }

        /// Return true if a match is negated.
        pub fn is_negative(&self) -> bool {
            self.mode.contains(Mode::NEGATIVE)
        }

        /// Match the given `path` which takes slashes (and only slashes) literally.
        /// We may take various shortcuts which is when `basename_start_pos` and `is_dir` come into play.
        /// Lastly, case insensitive matches are supported as well.
        pub fn matches_path(
            &self,
            _path: &BStr,
            _basename_start_pos: Option<usize>,
            _is_dir: bool,
            case: Case,
        ) -> bool {
            let _flags = MatchOptions::SLASH_IS_LITERAL
                | match case {
                    Case::Fold => MatchOptions::IGNORE_CASE,
                    Case::Sensitive => MatchOptions::empty(),
                };
            todo!()
        }

        pub fn matches(&self, _value: &BStr, _options: MatchOptions) -> bool {
            todo!()
        }
    }
}

mod parse;
pub use parse::pattern as parse;
