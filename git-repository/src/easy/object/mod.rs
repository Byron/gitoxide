use std::{cell::Ref, convert::TryInto};

pub use git_object::Kind;

use crate::{easy, hash::ObjectId, objs::immutable, odb, Object, ObjectRef, TreeRef};

mod impls;
mod tree;

impl Object {
    pub fn attach<A>(self, access: &A) -> easy::Result<ObjectRef<'_, A>>
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
    pub(crate) fn from_current_buf(id: impl Into<ObjectId>, kind: Kind, access: &'repo A) -> easy::Result<Self> {
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
    use quick_error::quick_error;

    use crate::{easy, odb};

    quick_error! {
        #[derive(Debug)]
        pub enum Error {
            Find(err: OdbError) {
                display("An error occurred while trying to find an object")
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
    pub(crate) type OdbError = odb::compound::find::Error;

    pub mod existing {
        use quick_error::quick_error;

        use crate::{easy, odb};

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
    pub fn to_commit_iter(&self) -> Option<immutable::CommitIter<'_>> {
        odb::data::Object::new(self.kind, &self.data).into_commit_iter()
    }

    pub fn to_tag_iter(&self) -> Option<immutable::TagIter<'_>> {
        odb::data::Object::new(self.kind, &self.data).into_tag_iter()
    }
}

pub mod peel_to_kind {
    pub use error::Error;

    use crate::easy::object::{peel_to_kind, Kind};
    use crate::{easy, ObjectRef};

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
                        self = crate::ext::access::object::find_object(access, tree_id)?;
                    }
                    Kind::Tag => {
                        let target_id = self.to_tag_iter().expect("tag").target_id().expect("valid tag");
                        let access = self.access;
                        drop(self);
                        self = crate::ext::access::object::find_object(access, target_id)?;
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

        use crate::easy::object;
        use crate::easy::object::find;

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
