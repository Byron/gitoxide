use bstr::{BStr, ByteSlice};
use std::borrow::Cow;
use std::path::Path;

use crate::{MagicSignature, Pattern, Search};

/// Describes a matching pattern within a search for ignored paths.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
pub struct Match<'a> {
    /// The matching search specification, which contains the pathspec as well.
    pub pattern: &'a Pattern,
    /// The number of the sequence the matching pathspec was in, or the line of pathspec file it was read from if [Search::source] is not `None`.
    pub sequence_number: usize,
    /// How the pattern matched.
    pub kind: MatchKind,
}

/// Describe how a pathspec pattern matched.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash, Ord, PartialOrd)]
pub enum MatchKind {
    /// The match happened because there wasn't any pattern, which matches all, or because there was a nil pattern or one with an empty path.
    /// Thus this is not a match by merit.
    Always,
    /// The first part of a pathspec matches, like `dir/` that matches `dir/a`.
    Prefix,
    /// The whole pathspec matched and used a wildcard match, like `a/*` matching `a/file`.
    WildcardMatch,
    /// The entire pathspec matched, letter by letter, e.g. `a/file` matching `a/file`.
    Verbatim,
}

mod init;

impl Match<'_> {
    /// Return `true` if the pathspec that matched was negative, which excludes this item from the set.
    pub fn is_excluded(&self) -> bool {
        self.pattern.is_excluded()
    }
}

/// Access
impl Search {
    /// Return an iterator over the patterns that participate in the search.
    pub fn patterns(&self) -> impl ExactSizeIterator<Item = &Pattern> + '_ {
        self.patterns.iter().map(|m| &m.value.pattern)
    }

    /// Return the portion of the prefix among all of the pathspecs involved in this search, or an empty string if
    /// there is none. It doesn't have to end at a directory boundary though, nor does it denote a directory.
    ///
    /// Note that the common_prefix is always matched case-sensitively, and it is useful to skip large portions of input.
    /// Further, excluded pathspecs don't participate which makes this common prefix inclusive. To work correctly though,
    /// one will have to additionally match paths that have the common prefix with that pathspec itself to assure it is
    /// not excluded.
    pub fn common_prefix(&self) -> &BStr {
        self.patterns
            .iter()
            .find(|p| !p.value.pattern.is_excluded())
            .map_or("".into(), |m| m.value.pattern.path[..self.common_prefix_len].as_bstr())
    }

    /// Returns a guaranteed-to-be-directory that is shared across all pathspecs, in its repository-relative form.
    /// Thus to be valid, it must be joined with the worktree root.
    /// The prefix is the CWD within a worktree passed when [normalizing](crate::Pattern::normalize) the pathspecs.
    ///
    /// Note that it may well be that the directory isn't available even though there is a [`common_prefix()`](Self::common_prefix),
    /// as they are not quire the same.
    ///
    /// See also: [`maybe_prefix_directory()`](Self::longest_common_directory).
    pub fn prefix_directory(&self) -> Cow<'_, Path> {
        gix_path::from_bstr(
            self.patterns
                .iter()
                .find(|p| !p.value.pattern.is_excluded())
                .map_or("".into(), |m| m.value.pattern.prefix_directory()),
        )
    }

    /// Return the longest possible common directory that is shared across all non-exclusive pathspecs.
    /// It must be tested for existence by joining it with a suitable root before being able to use it.
    /// Note that if it is returned, it's guaranteed to be longer than the [prefix-directory](Self::prefix_directory).
    ///
    /// Returns `None` if the returned directory would be empty, or if all pathspecs are exclusive.
    pub fn longest_common_directory(&self) -> Option<Cow<'_, Path>> {
        let first_non_excluded = self.patterns.iter().find(|p| !p.value.pattern.is_excluded())?;
        let common_prefix = first_non_excluded.value.pattern.path[..self.common_prefix_len].as_bstr();
        let stripped_prefix = if first_non_excluded
            .value
            .pattern
            .signature
            .contains(MagicSignature::MUST_BE_DIR)
        {
            common_prefix
        } else {
            common_prefix[..common_prefix.rfind_byte(b'/')?].as_bstr()
        };
        Some(gix_path::from_bstr(stripped_prefix))
    }
}

#[derive(Default, Clone, Debug)]
pub(crate) struct Spec {
    pub pattern: Pattern,
    pub attrs_match: Option<gix_attributes::search::Outcome>,
}

mod matching;
