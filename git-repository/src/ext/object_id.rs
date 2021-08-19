use git_hash::ObjectId;
#[cfg(feature = "git-traverse")]
use git_object::immutable;
#[cfg(feature = "git-traverse")]
use git_traverse::commit::ancestors::{Ancestors, State};

use crate::{easy, Oid};

pub trait Sealed {}

pub trait ObjectIdExt: Sealed {
    #[cfg(feature = "git-traverse")]
    fn ancestors_iter<Find>(self, find: Find) -> Ancestors<Find, fn(&git_hash::oid) -> bool, State>
    where
        Find: for<'a> FnMut(&git_hash::oid, &'a mut Vec<u8>) -> Option<immutable::CommitIter<'a>>;

    fn attach<A: easy::Access + Sized>(self, access: &A) -> Oid<'_, A>;
}

impl Sealed for ObjectId {}

impl ObjectIdExt for ObjectId {
    #[cfg(feature = "git-traverse")]
    fn ancestors_iter<Find>(self, find: Find) -> Ancestors<Find, fn(&git_hash::oid) -> bool, State>
    where
        Find: for<'a> FnMut(&git_hash::oid, &'a mut Vec<u8>) -> Option<immutable::CommitIter<'a>>,
    {
        Ancestors::new(Some(self), State::default(), find)
    }

    fn attach<A: easy::Access + Sized>(self, access: &A) -> Oid<'_, A> {
        Oid::from_id(self, access)
    }
}
