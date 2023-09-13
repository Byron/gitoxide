use bstr::{BStr, ByteSlice};

use crate::{Pattern, Search};

/// Describes a matching pattern within a search for ignored paths.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
pub struct Match<'a> {
    /// The matching search specification, which contains the pathspec as well.
    pub pattern: &'a Pattern,
    /// The number of the sequence the matching pathspec was in, or the line of pathspec file it was read from if [Search::source] is not `None`.
    pub sequence_number: usize,
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
    pub fn patterns(&self) -> impl Iterator<Item = &Pattern> + '_ {
        self.patterns.iter().map(|m| &m.value.pattern)
    }

    /// Return the portion of the prefix among all of the pathspecs involved in this search, or an empty string if
    /// there is none. It doesn't have to end at a directory boundary though, nor does it denote a directory.
    ///
    /// Note that the common_prefix is always matched case-sensitively, and it is useful to skip large portions of input.
    /// Further, excluded pathspecs don't participate which makes this common prefix inclusive. To work correclty though,
    /// one will have to additionally match paths that have the common prefix with that pathspec itself to assure it is
    /// not excluded.
    pub fn common_prefix(&self) -> &BStr {
        self.patterns
            .iter()
            .find(|p| !p.value.pattern.is_excluded())
            .map_or("".into(), |m| m.value.pattern.path[..self.common_prefix_len].as_bstr())
    }
}

#[derive(Default, Clone, Debug)]
pub(crate) struct Spec {
    pub pattern: Pattern,
    pub attrs_match: Option<gix_attributes::search::Outcome>,
}

mod matching;
