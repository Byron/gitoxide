use std::{cell::Ref, ops::DerefMut};

pub use git_object::Kind;

use crate::odb::Find;
use crate::{
    hash::{oid, ObjectId},
    object, odb,
    odb::FindExt,
    Access, Oid,
};
use std::borrow::Borrow;

mod impls {
    use super::Oid;
    use crate::hash::{oid, ObjectId};

    impl<'repo, A, B> PartialEq<Oid<'repo, A>> for Oid<'repo, B> {
        fn eq(&self, other: &Oid<'repo, A>) -> bool {
            self.id == other.id
        }
    }

    impl<'repo, A> PartialEq<ObjectId> for Oid<'repo, A> {
        fn eq(&self, other: &ObjectId) -> bool {
            &self.id == other
        }
    }

    impl<'repo, A> std::fmt::Debug for Oid<'repo, A> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.id.fmt(f)
        }
    }

    impl<'repo, A> AsRef<oid> for Oid<'repo, A> {
        fn as_ref(&self) -> &oid {
            &self.id
        }
    }

    impl<'repo, A> From<Oid<'repo, A>> for ObjectId {
        fn from(v: Oid<'repo, A>) -> Self {
            v.id
        }
    }
}

pub struct DetachedObject<'repo> {
    pub kind: Kind,
    pub data: Ref<'repo, [u8]>,
}

pub mod find {
    use crate::odb;

    pub type Error = odb::compound::find::Error;
    pub mod existing {
        use crate::odb;

        pub type Error = odb::pack::find::existing::Error<odb::compound::find::Error>;
    }
}

pub mod peel_to_kind {
    use quick_error::quick_error;

    use crate::{hash::ObjectId, object, object::find, odb};

    quick_error! {
        #[derive(Debug)]
        pub enum Error {
            FindExisting(err: find::existing::Error) {
                display("A non existing object was encountered during object peeling")
                from()
                source(err)
            }
            NotFound{id: ObjectId, kind: object::Kind} {
                display("Last encountered object was {} while trying to peel to {}", id, kind)
            }
        }
    }
}
impl<'repo, A> Oid<'repo, A>
where
    A: crate::prelude::ObjectAccessExt + Access + Sized,
{
    // NOTE: Can't access other object data that is attached to the same cache.
    pub fn existing_object(&self) -> Result<DetachedObject<'repo>, find::existing::Error> {
        self.access.find_existing_object(&self.id)
    }

    // NOTE: Can't access other object data that is attached to the same cache.
    pub fn object(&self) -> Result<Option<DetachedObject<'repo>>, find::Error> {
        self.access.find_object(&self.id)
    }
}

impl<'repo, A> Oid<'repo, A>
where
    A: Access + Sized,
{
    pub(crate) fn from_id(id: impl Into<ObjectId>, access: &'repo A) -> Self {
        Oid { id: id.into(), access }
    }

    pub fn id(&self) -> &oid {
        &self.id
    }

    pub fn into_id(self) -> ObjectId {
        self.id
    }

    pub fn detach(self) -> ObjectId {
        self.id
    }

    // TODO: tests
    pub fn peel_to_kind(&self, kind: Kind) -> Result<(ObjectId, DetachedObject<'repo>), peel_to_kind::Error> {
        let mut id = self.id;
        let mut buf = self.access.cache().buf.borrow_mut();
        let mut cursor =
            self.access
                .repo()
                .odb
                .find_existing(&id, &mut buf, self.access.cache().pack.borrow_mut().deref_mut())?;
        loop {
            match cursor.kind {
                any_kind if kind == any_kind => {
                    let kind = cursor.kind;
                    drop(cursor);
                    drop(buf);
                    return Ok((
                        id,
                        DetachedObject {
                            kind,
                            data: Ref::map(self.access.cache().buf.borrow(), |v| v.as_slice()),
                        },
                    ));
                }
                Kind::Commit => {
                    id = cursor.into_commit_iter().expect("commit").tree_id().expect("id");
                    cursor = self.access.repo().odb.find_existing(
                        id,
                        &mut buf,
                        self.access.cache().pack.borrow_mut().deref_mut(),
                    )?;
                }
                Kind::Tag => {
                    id = cursor
                        .into_tag_iter()
                        .expect("tag")
                        .target_id()
                        .expect("target present");
                    cursor = self.access.repo().odb.find_existing(
                        id,
                        &mut buf,
                        self.access.cache().pack.borrow_mut().deref_mut(),
                    )?;
                }
                Kind::Tree | Kind::Blob => return Err(peel_to_kind::Error::NotFound { id, kind }),
            }
        }
    }
}
