use crate::{pattern, Pattern};
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
        /// The pattern starts with a slash and thus matches only from the beginning.
        const ABSOLUTE = 1 << 4;
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

    /// Match the given `path` which takes slashes (and only slashes) literally, and is relative to the repository root.
    /// Note that `path` is assumed to be sharing the base with the pattern already so they can be reasonably compared.
    ///
    /// We may take various shortcuts which is when `basename_start_pos` and `is_dir` come into play.
    /// `basename_start_pos` is the index at which the `path`'s basename starts.
    ///
    /// Lastly, `case` folding can be configured as well.
    pub fn matches_path<'a>(
        &self,
        path: impl Into<&'a BStr>,
        basename_start_pos: Option<usize>,
        is_dir: bool,
        case: Case,
    ) -> bool {
        if !is_dir && self.mode.contains(pattern::Mode::MUST_BE_DIR) {
            return false;
        }

        let flags = MatchOptions::SLASH_IS_LITERAL
            | match case {
                Case::Fold => MatchOptions::IGNORE_CASE,
                Case::Sensitive => MatchOptions::empty(),
            };
        let path = path.into();

        if self.mode.contains(pattern::Mode::NO_SUB_DIR) {
            let basename = &path[basename_start_pos.unwrap_or_default()..];
            self.matches(basename, flags)
        } else {
            // TODO
            false
        }
    }

    pub fn matches(&self, value: &BStr, options: MatchOptions) -> bool {
        match self.first_wildcard_pos {
            // "*literal" case
            Some(pos) if self.mode.contains(pattern::Mode::ENDS_WITH) => {
                let text = &self.text[pos + 1..];
                if options.contains(MatchOptions::IGNORE_CASE) {
                    value
                        .len()
                        .checked_sub(text.len())
                        .map(|start| text.eq_ignore_ascii_case(&value[start..]))
                        .unwrap_or(false)
                } else {
                    value.ends_with(text.as_ref())
                }
            }
            Some(_pos) => todo!("actual wildcards for basename: {}", _pos),
            None => {
                if options.contains(MatchOptions::IGNORE_CASE) {
                    self.text.eq_ignore_ascii_case(value)
                } else {
                    self.text == value
                }
            }
        }
    }
}
