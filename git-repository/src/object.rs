use std::{cell::Ref, ops::DerefMut};

pub use git_object::Kind;

use crate::{
    hash::{oid, ObjectId},
    object, odb,
    odb::FindExt,
    Access, Object,
};

impl<'repo, A, B> PartialEq<Object<'repo, A>> for Object<'repo, B> {
    fn eq(&self, other: &Object<'repo, A>) -> bool {
        self.id == other.id
    }
}

impl<'repo, A> std::fmt::Debug for Object<'repo, A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.id.fmt(f)
    }
}

pub struct Data<'repo> {
    pub kind: Kind,
    pub bytes: Ref<'repo, [u8]>,
}

pub mod find {
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

    /// Makes all lower level objects and iterators accessible
    fn raw(&self) -> Result<Data<'repo>, find::existing::Error> {
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

    // TODO: tests
    pub fn peel_to_kind(&self, kind: Kind) -> Result<(Self, Data<'repo>), peel_to_kind::Error> {
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
                    return Ok((
                        Object::from_id(id, self.access),
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
