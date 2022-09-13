use crate::Matcher;
use bstr::BStr;
use git_hash::{oid, ObjectId};
use std::borrow::BorrowMut;

#[allow(dead_code)]
pub(crate) enum Needle<'a> {
    FullName(&'a BStr),
    PartialName(&'a BStr),
    Glob { glob: &'a BStr, asterisk_pos: usize },
    Object(ObjectId),
}

impl<'a> From<&'a BStr> for Needle<'a> {
    fn from(v: &'a BStr) -> Self {
        if v.starts_with(b"refs/") {
            Needle::FullName(v)
        } else {
            todo!()
        }
    }
}

impl<'a> Matcher<'a> {
    /// For each item in `names`, fill the accompanying `Match` structure to represent whether or not a match was found based
    /// on all included information to match against.
    /// Note that each _name_ in `names` is expected to be the full name of a reference.
    pub fn match_remotes<'b>(&self, _names: impl Iterator<Item = (Item<'b>, impl BorrowMut<Match<'a>>)>) {
        todo!()
    }
}

/// An item to match, input to various matching operations.
#[derive(Debug, Copy, Clone)]
pub struct Item<'a> {
    /// The full name of the references, like `refs/heads/main`
    pub full_ref_name: &'a BStr,
    /// The peeled id it points to that we should match against.
    pub target: &'a oid,
    /// The tag object's id if this is a tag
    pub tag: Option<&'a oid>,
}

impl Match<'_> {
    /// Return true if we are representing an actual match
    pub fn matched(&self) -> bool {
        self.lhs.is_some()
    }

    /// Return the remote side (i.e. left side) of the fetch ref-spec that matched, or `None` if it didn't match.
    pub fn remote(&self) -> Option<&BStr> {
        self.lhs
    }

    /// Return the local side (i.e. right side) of the fetch ref-spec that matched, or `None` if it didn't match.
    ///
    /// This is also called a tracking ref.
    pub fn local(&self) -> Option<&BStr> {
        self.rhs
    }
}

/// The result of a match operation.
#[derive(Default, Clone)]
#[allow(dead_code)]
pub struct Match<'a> {
    pub(crate) lhs: Option<&'a bstr::BStr>,
    pub(crate) rhs: Option<&'a bstr::BStr>,
}
