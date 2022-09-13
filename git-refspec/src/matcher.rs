use crate::Match;
use bstr::BStr;
use git_hash::{oid, ObjectId};

#[allow(dead_code)]
pub(crate) enum Needle<'a> {
    FullName(&'a BStr),
    PartialName(&'a BStr),
    Glob { glob: &'a BStr, asterisk_pos: usize },
    Object(ObjectId),
}

impl<'a> From<&'a BStr> for Needle<'a> {
    fn from(_v: &'a BStr) -> Self {
        todo!()
    }
}

/// An item to match
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
