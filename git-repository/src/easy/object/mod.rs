#![allow(missing_docs)]
use std::{cell::Ref, convert::TryInto};

use git_hash::ObjectId;
pub use git_object::Kind;
use git_object::{CommitRefIter, TagRefIter};

use crate::{
    easy,
    easy::{Object, ObjectRef, TreeRef},
};

mod impls;
mod tree;

impl Object {
    pub fn attach<A>(self, access: &A) -> easy::borrow::state::Result<ObjectRef<'_, A>>
    where
        A: easy::Access + Sized,
    {
        *access.state().try_borrow_mut_buf()? = self.data;
        Ok(ObjectRef {
            id: self.id,
            kind: self.kind,
            data: Ref::map(access.state().try_borrow_buf()?, |v| v.as_slice()),
            access,
        })
    }
}

impl<'repo, A> ObjectRef<'repo, A>
where
    A: easy::Access + Sized,
{
    pub(crate) fn from_current_buf(
        id: impl Into<ObjectId>,
        kind: Kind,
        access: &'repo A,
    ) -> easy::borrow::state::Result<Self> {
        Ok(ObjectRef {
            id: id.into(),
            kind,
            data: Ref::map(access.state().try_borrow_buf()?, |v| v.as_slice()),
            access,
        })
    }

    pub fn into_tree(self) -> TreeRef<'repo, A> {
        match self.try_into() {
            Ok(tree) => tree,
            Err(this) => panic!("Tried to use {} as tree, but was {}", this.id, this.kind),
        }
    }

    pub fn try_into_tree(self) -> Result<TreeRef<'repo, A>, Self> {
        self.try_into()
    }
}

pub mod find {

    use crate::easy;

    pub(crate) type OdbError = git_odb::compound::find::Error;

    #[derive(Debug, thiserror::Error)]
    pub enum Error {
        #[error(transparent)]
        Find(#[from] OdbError),
        #[error("BUG: Part of interior state could not be borrowed.")]
        BorrowState(#[from] easy::borrow::state::Error),
        #[error("BUG: The repository could not be borrowed")]
        BorrowRepo(#[from] easy::borrow::repo::Error),
    }

    pub mod existing {
        use crate::easy;

        pub(crate) type OdbError = git_odb::pack::find::existing::Error<git_odb::compound::find::Error>;

        #[derive(Debug, thiserror::Error)]
        pub enum Error {
            #[error(transparent)]
            FindExisting(#[from] OdbError),
            #[error("BUG: Part of interior state could not be borrowed.")]
            BorrowState(#[from] easy::borrow::state::Error),
            #[error("BUG: The repository could not be borrowed")]
            BorrowRepo(#[from] easy::borrow::repo::Error),
        }
    }
}

pub mod write {
    use crate::easy;

    #[derive(Debug, thiserror::Error)]
    pub enum Error {
        #[error(transparent)]
        OdbWrite(#[from] git_odb::loose::write::Error),
        #[error("BUG: The repository could not be borrowed")]
        BorrowRepo(#[from] easy::borrow::repo::Error),
    }
}

impl<'repo, A> ObjectRef<'repo, A> {
    pub fn to_owned(&self) -> Object {
        Object {
            id: self.id,
            kind: self.kind,
            data: self.data.to_owned(),
        }
    }

    pub fn into_owned(self) -> Object {
        Object {
            id: self.id,
            kind: self.kind,
            data: self.data.to_owned(),
        }
    }

    pub fn detach(self) -> Object {
        self.into()
    }
}

impl<'repo, A> ObjectRef<'repo, A>
where
    A: easy::Access + Sized,
{
    /// As [`to_commit_iter()`][ObjectRef::to_commit_iter()] but panics if this is not a commit
    pub fn commit_iter(&self) -> CommitRefIter<'_> {
        git_odb::data::Object::new(self.kind, &self.data)
            .try_into_commit_iter()
            .expect("BUG: This object must be a commit")
    }

    pub fn to_commit_iter(&self) -> Option<CommitRefIter<'_>> {
        git_odb::data::Object::new(self.kind, &self.data).try_into_commit_iter()
    }

    pub fn to_tag_iter(&self) -> Option<TagRefIter<'_>> {
        git_odb::data::Object::new(self.kind, &self.data).try_into_tag_iter()
    }
}

pub mod peel {
    use crate::{
        easy,
        easy::{
            ext::ObjectAccessExt,
            object,
            object::{peel, Kind},
            ObjectRef,
        },
    };

    pub mod to_kind {
        mod error {

            use crate::easy::object;

            #[derive(Debug, thiserror::Error)]
            pub enum Error {
                #[error(transparent)]
                FindExistingObject(#[from] object::find::existing::Error),
                #[error("Last encountered object kind was {} while trying to peel to {}", .actual, .expected)]
                NotFound {
                    actual: object::Kind,
                    expected: object::Kind,
                },
            }
        }
        pub use error::Error;
    }

    impl<'repo, A> ObjectRef<'repo, A>
    where
        A: easy::Access + Sized,
    {
        // TODO: tests
        pub fn peel_to_kind(mut self, kind: Kind) -> Result<Self, peel::to_kind::Error> {
            loop {
                match self.kind {
                    any_kind if kind == any_kind => {
                        return Ok(self);
                    }
                    Kind::Commit => {
                        let tree_id = self.to_commit_iter().expect("commit").tree_id().expect("valid commit");
                        let access = self.access;
                        drop(self);
                        self = access.find_object(tree_id)?;
                    }
                    Kind::Tag => {
                        let target_id = self.to_tag_iter().expect("tag").target_id().expect("valid tag");
                        let access = self.access;
                        drop(self);
                        self = access.find_object(target_id)?;
                    }
                    Kind::Tree | Kind::Blob => {
                        return Err(peel::to_kind::Error::NotFound {
                            actual: self.kind,
                            expected: kind,
                        })
                    }
                }
            }
        }

        // TODO: tests
        pub fn peel_to_end(mut self) -> Result<Self, object::find::existing::Error> {
            loop {
                match self.kind {
                    Kind::Commit | Kind::Tree | Kind::Blob => break Ok(self),
                    Kind::Tag => {
                        let target_id = self.to_tag_iter().expect("tag").target_id().expect("valid tag");
                        let access = self.access;
                        drop(self);
                        self = access.find_object(target_id)?;
                    }
                }
            }
        }
    }
}
