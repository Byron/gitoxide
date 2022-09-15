use crate::RefSpecRef;
use bstr::BStr;
use git_hash::oid;
use std::borrow::Cow;

/// A match group is able to match a list of ref specs in order while handling negation, conflicts and one to many mappings.
#[derive(Debug, Clone)]
pub struct MatchGroup<'a> {
    pub(crate) specs: Vec<RefSpecRef<'a>>,
}

/// The outcome of any matching operation of a [`MatchGroup`].
///
/// It's used to validate and process the contained [mappings][Mapping].
#[derive(Debug, Clone)]
pub struct Outcome<'spec, 'item> {
    /// The match group that produced this outcome.
    pub group: MatchGroup<'spec>,
    /// The mappings derived from matching [items][Item].
    pub mappings: Vec<Mapping<'item, 'spec>>,
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
/// The source (or left-hand) side of a mapping.
pub enum Source<'a> {
    /// A full reference name, which is expected to be valid.
    ///
    /// Validity, however, is not enforced here.
    FullName(&'a BStr),
    /// The name of an object that is expected to exist on the remote side.
    /// Note that it might not be advertised by the remote but part of the object graph,
    /// and thus gets sent in the pack. The server is expected to fail unless the desired
    /// object is present but at some time it is merely a request by the user.
    ObjectId(git_hash::ObjectId),
}

/// A mapping from a remote to a local refs for fetches or local to remote refs for pushes.
///
/// Mappings are like edges in a graph, initially without any constraints.
#[derive(Debug, Clone)]
pub struct Mapping<'a, 'b> {
    /// The index into the initial `items` list that matched against a spec.
    pub item_index: Option<usize>,
    /// The name of the remote side for fetches or the local one for pushes that matched.
    pub lhs: Source<'a>,
    /// The name of the local side for fetches or the remote one for pushes that corresponds to `lhs`, if available.
    pub rhs: Option<Cow<'b, BStr>>,
    /// The index of the matched ref-spec as seen from the match group.
    pub(crate) spec_index: usize,
}
