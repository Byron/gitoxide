use std::{cell::Ref, ops::DerefMut};

pub use git_object::Kind;

use crate::odb::Find;
use crate::{
    hash::{oid, ObjectId},
    object, odb,
    odb::FindExt,
    Access, Object,
};
use std::borrow::Borrow;

mod impls {
    use super::Object;
    use crate::hash::{oid, ObjectId};

    impl<'repo, A, B> PartialEq<Object<'repo, A>> for Object<'repo, B> {
        fn eq(&self, other: &Object<'repo, A>) -> bool {
            self.id == other.id
        }
    }

    impl<'repo, A> PartialEq<ObjectId> for Object<'repo, A> {
        fn eq(&self, other: &ObjectId) -> bool {
            &self.id == other
        }
    }

    impl<'repo, A> std::fmt::Debug for Object<'repo, A> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.id.fmt(f)
        }
    }

    impl<'repo, A> AsRef<oid> for Object<'repo, A> {
        fn as_ref(&self) -> &oid {
            &self.id
        }
    }

    impl<'repo, A> From<Object<'repo, A>> for ObjectId {
        fn from(v: Object<'repo, A>) -> Self {
            v.id
        }
    }
}

pub struct Data<'repo> {
    pub kind: Kind,
    pub bytes: Ref<'repo, [u8]>,
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

impl<'repo, A> Object<'repo, A>
where
    A: Access + Sized,
{
    pub(crate) fn from_id(id: impl Into<ObjectId>, access: &'repo A) -> Self {
        Object { id: id.into(), access }
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

    // NOTE: Can't access other object data that is attached to the same cache.
    pub fn existing_data(&self) -> Result<Data<'repo>, find::existing::Error> {
        let mut buf = self.access.cache().buf.borrow_mut();
        let kind = {
            let obj = self.access.repo().odb.find_existing(
                &self.id,
                &mut buf,
                self.access.cache().pack.borrow_mut().deref_mut(),
            )?;
            obj.kind
        };

        Ok(Data {
            kind,
            bytes: Ref::map(self.access.cache().buf.borrow(), |v| v.as_slice()),
        })
    }

    // NOTE: Can't access other object data that is attached to the same cache.
    pub fn data(&self) -> Result<Option<Data<'repo>>, find::Error> {
        let mut buf = self.access.cache().buf.borrow_mut();
        Ok(self
            .access
            .repo()
            .odb
            .find(&self.id, &mut buf, self.access.cache().pack.borrow_mut().deref_mut())?
            .map(|obj| {
                let kind = obj.kind;
                drop(obj);
                Data {
                    kind,
                    bytes: Ref::map(self.access.cache().buf.borrow(), |v| v.as_slice()),
                }
            }))
    }

    // TODO: tests
    pub fn peel_to_kind(&self, kind: Kind) -> Result<(ObjectId, Data<'repo>), peel_to_kind::Error> {
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
                        Data {
                            kind,
                            bytes: Ref::map(self.access.cache().buf.borrow(), |v| v.as_slice()),
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
