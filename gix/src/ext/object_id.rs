use gix_hash::ObjectId;
use gix_traverse::commit::{ancestors, Ancestors};

pub trait Sealed {}

pub type AncestorsIter<Find> = Ancestors<Find, fn(&gix_hash::oid) -> bool, ancestors::State>;

/// An extension trait to add functionality to [`ObjectId`]s.
pub trait ObjectIdExt: Sealed {
    /// Create an iterator over the ancestry of the commits reachable from this id, which must be a commit.
    fn ancestors<Find>(self, find: Find) -> AncestorsIter<Find>
    where
        Find: gix_object::Find;

    /// Infuse this object id `repo` access.
    fn attach(self, repo: &crate::Repository) -> crate::Id<'_>;
}

impl Sealed for ObjectId {}

impl ObjectIdExt for ObjectId {
    fn ancestors<Find>(self, find: Find) -> AncestorsIter<Find>
    where
        Find: gix_object::Find,
    {
        Ancestors::new(Some(self), ancestors::State::default(), find)
    }

    fn attach(self, repo: &crate::Repository) -> crate::Id<'_> {
        crate::Id::from_id(self, repo)
    }
}
