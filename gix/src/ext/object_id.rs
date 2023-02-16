use gix_hash::ObjectId;
use gix_traverse::commit::{ancestors, Ancestors};

pub trait Sealed {}

pub type AncestorsIter<Find> = Ancestors<Find, fn(&gix_hash::oid) -> bool, ancestors::State>;

/// An extension trait to add functionality to [`ObjectId`]s.
pub trait ObjectIdExt: Sealed {
    /// Create an iterator over the ancestry of the commits reachable from this id, which must be a commit.
    fn ancestors<Find, E>(self, find: Find) -> AncestorsIter<Find>
    where
        Find: for<'a> FnMut(&gix_hash::oid, &'a mut Vec<u8>) -> Result<gix_object::CommitRefIter<'a>, E>,
        E: std::error::Error + Send + Sync + 'static;

    /// Infuse this object id `repo` access.
    fn attach(self, repo: &crate::Repository) -> crate::Id<'_>;
}

impl Sealed for ObjectId {}

impl ObjectIdExt for ObjectId {
    fn ancestors<Find, E>(self, find: Find) -> AncestorsIter<Find>
    where
        Find: for<'a> FnMut(&gix_hash::oid, &'a mut Vec<u8>) -> Result<gix_object::CommitRefIter<'a>, E>,
        E: std::error::Error + Send + Sync + 'static,
    {
        Ancestors::new(Some(self), ancestors::State::default(), find)
    }

    fn attach(self, repo: &crate::Repository) -> crate::Id<'_> {
        crate::Id::from_id(self, repo)
    }
}
