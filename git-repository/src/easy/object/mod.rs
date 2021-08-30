#![allow(missing_docs)]
use std::{cell::Ref, convert::TryInto};

use git_hash::ObjectId;
pub use git_object::Kind;
use git_object::{CommitRefIter, TagRefIter};
use git_odb as odb;

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
    use git_odb as odb;

    use crate::easy;

    pub(crate) type OdbError = odb::compound::find::Error;

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
        use git_odb as odb;
        use quick_error::quick_error;

        use crate::easy;

        quick_error! {
            #[derive(Debug)]
            pub enum Error {
                FindExisting(err: OdbError) {
                    display("Could not find a supposedly existing object")
                    from()
                    source(err)
                }
                BorrowState(err: easy::borrow::state::Error) {
                    display("BUG: Part of interior state could not be borrowed.")
                    from()
                    source(err)
                }
                BorrowRepo(err: easy::borrow::repo::Error) {
                    display("BUG: The repository could not be borrowed")
                    from()
                }
            }
        }

        pub(crate) type OdbError = odb::pack::find::existing::Error<odb::compound::find::Error>;
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
    pub fn to_commit_iter(&self) -> Option<CommitRefIter<'_>> {
        odb::data::Object::new(self.kind, &self.data).into_commit_iter()
    }

    pub fn to_tag_iter(&self) -> Option<TagRefIter<'_>> {
        odb::data::Object::new(self.kind, &self.data).into_tag_iter()
    }
}

pub mod peel_to_kind {
    pub use error::Error;

    use crate::{
        easy,
        easy::{
            object::{peel_to_kind, Kind},
            ObjectRef,
        },
    };

    impl<'repo, A> ObjectRef<'repo, A>
    where
        A: easy::Access + Sized,
    {
        // TODO: tests
        pub fn peel_to_kind(mut self, kind: Kind) -> Result<Self, peel_to_kind::Error> {
            loop {
                match self.kind {
                    any_kind if kind == any_kind => {
                        return Ok(self);
                    }
                    Kind::Commit => {
                        let tree_id = self.to_commit_iter().expect("commit").tree_id().expect("valid commit");
                        let access = self.access;
                        drop(self);
                        self = crate::easy::ext::object::find_object(access, tree_id)?;
                    }
                    Kind::Tag => {
                        let target_id = self.to_tag_iter().expect("tag").target_id().expect("valid tag");
                        let access = self.access;
                        drop(self);
                        self = crate::easy::ext::object::find_object(access, target_id)?;
                    }
                    Kind::Tree | Kind::Blob => {
                        return Err(peel_to_kind::Error::NotFound {
                            actual: self.kind,
                            expected: kind,
                        })
                    }
                }
            }
        }
    }

    mod error {
        use quick_error::quick_error;

        use crate::easy::{object, object::find};

        quick_error! {
            #[derive(Debug)]
            pub enum Error {
                FindExisting(err: find::existing::Error) {
                    display("A non existing object was encountered during object peeling")
                    from()
                    source(err)
                }
                NotFound{actual: object::Kind, expected: object::Kind} {
                    display("Last encountered object kind was {} while trying to peel to {}", actual, expected)
                }
            }
        }
    }
}
