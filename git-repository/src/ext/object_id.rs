use git_hash::ObjectId;
use git_traverse::commit::{ancestors, Ancestors};

use crate::easy;

pub trait Sealed {}

/// An extension trait to add functionality to [`ObjectId`]s.
pub trait ObjectIdExt: Sealed {
    /// Create an iterator over the ancestry of the commits reachable from this id, which must be a commit.
    fn ancestors<Find>(self, find: Find) -> Ancestors<Find, fn(&git_hash::oid) -> bool, ancestors::State>
    where
        Find: for<'a> FnMut(&git_hash::oid, &'a mut Vec<u8>) -> Option<git_object::CommitRefIter<'a>>;

    /// Infuse this object id with an [`easy::Handle`].
    fn attach(self, handle: &easy::Handle) -> easy::Oid<'_>;
}

impl Sealed for ObjectId {}

impl ObjectIdExt for ObjectId {
    fn ancestors<Find>(self, find: Find) -> Ancestors<Find, fn(&git_hash::oid) -> bool, ancestors::State>
    where
        Find: for<'a> FnMut(&git_hash::oid, &'a mut Vec<u8>) -> Option<git_object::CommitRefIter<'a>>,
    {
        Ancestors::new(Some(self), ancestors::State::default(), find)
    }

    fn attach(self, handle: &easy::Handle) -> easy::Oid<'_> {
        easy::Oid::from_id(self, handle)
    }
}
