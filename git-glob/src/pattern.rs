use crate::{pattern, Pattern};
use bitflags::bitflags;
use bstr::{BStr, BString, ByteSlice};

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
            base_path: None,
        })
    }

    /// Return true if a match is negated.
    pub fn is_negative(&self) -> bool {
        self.mode.contains(Mode::NEGATIVE)
    }

    /// Set the base path of the pattern.
    /// Must be a slash-separated relative path with a trailing slash.
    ///
    /// Use this upon creation of the pattern when the source file is known.
    pub fn with_base(mut self, path: impl Into<BString>) -> Self {
        let path = path.into();
        debug_assert!(path.ends_with(b"/"), "base must end with a trailing slash");
        debug_assert!(!path.starts_with(b"/"), "base must be relative");
        self.base_path = Some(path);
        self
    }

    /// Match the given `path` which takes slashes (and only slashes) literally, and is relative to the repository root.
    /// Note that `path` is assumed to be relative to the repository, and that our [`base_path`][Self::base_path]
    /// is assumed to contain `path`.
    ///
    /// We may take various shortcuts which is when `basename_start_pos` and `is_dir` come into play.
    /// `basename_start_pos` is the index at which the `path`'s basename starts.
    ///
    /// Lastly, `case` folding can be configured as well.
    pub fn matches_repo_relative_path<'a>(
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
        debug_assert_eq!(
            basename_start_pos,
            path.rfind_byte(b'/').map(|p| p + 1),
            "BUG: invalid cached basename_start_pos provided"
        );
        debug_assert!(
            self.base_path
                .as_ref()
                .map(|base| path.starts_with(base))
                .unwrap_or(true),
            "repo-relative paths must be pre-filtered to match our base."
        );

        if self.mode.contains(pattern::Mode::NO_SUB_DIR) {
            let basename = if self.mode.contains(pattern::Mode::ABSOLUTE) {
                self.base_path
                    .as_ref()
                    .and_then(|base| path.strip_prefix(base.as_slice()).map(|b| b.as_bstr()))
                    .unwrap_or(path)
            } else {
                &path[basename_start_pos.unwrap_or_default()..]
            };
            self.matches(basename, flags)
        } else {
            let path = match self.base_path.as_ref() {
                Some(base) => match path.strip_prefix(base.as_slice()) {
                    Some(path) => path.as_bstr(),
                    None => return false,
                },
                None => path,
            };
            self.matches(path, flags)
        }
    }

    fn matches(&self, value: &BStr, options: MatchOptions) -> bool {
        match self.first_wildcard_pos {
            // "*literal" case, overrides starts-with
            Some(pos) if self.mode.contains(pattern::Mode::ENDS_WITH) && !value.contains(&b'/') => {
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
            Some(pos) => {
                if options.contains(MatchOptions::IGNORE_CASE) {
                    if !value
                        .get(..pos)
                        .map_or(false, |value| value.eq_ignore_ascii_case(&self.text[..pos]))
                    {
                        return false;
                    }
                } else if !value.starts_with(&self.text[..pos]) {
                    return false;
                }
                todo!("actual wildcard match for '{}' ~= '{}'", self.text, value)
            }
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
