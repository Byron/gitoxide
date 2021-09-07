#![allow(missing_docs)]
use std::{cell::RefMut, ops::Deref};

use git_hash::{oid, ObjectId};

use crate::{
    easy,
    easy::{ext::ObjectAccessExt, object::find, ObjectRef, Oid},
};

impl<'repo, A> Oid<'repo, A>
where
    A: easy::Access + Sized,
{
    // NOTE: Can't access other object data that is attached to the same cache.
    /// Find the [`ObjectRef`] associated with this object id, and assume it exists.
    pub fn object(&self) -> Result<ObjectRef<'repo, A>, find::existing::Error> {
        self.access.find_object(self.inner)
    }

    // NOTE: Can't access other object data that is attached to the same cache.
    /// Try find the [`ObjectRef`] associated with this object id, it might not be available locally.
    pub fn try_object(&self) -> Result<Option<ObjectRef<'repo, A>>, find::Error> {
        self.access.try_find_object(self.inner)
    }
}

impl<'repo, A> Deref for Oid<'repo, A> {
    type Target = oid;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'repo, A> Oid<'repo, A>
where
    A: easy::Access + Sized,
{
    pub(crate) fn from_id(id: impl Into<ObjectId>, access: &'repo A) -> Self {
        Oid {
            inner: id.into(),
            access,
        }
    }

    /// Turn this instance into its bare [ObjectId].
    pub fn detach(self) -> ObjectId {
        self.inner
    }
}

pub struct Ancestors<'repo, A>
where
    A: easy::Access + Sized,
{
    repo: A::RepoRef,
    pack_cache: RefMut<'repo, easy::PackCache>,
    access: &'repo A,
    tip: ObjectId,
}

pub mod ancestors {
    use std::ops::{Deref, DerefMut};

    use git_odb::Find;

    use crate::{
        easy,
        easy::{oid::Ancestors, Oid},
    };

    impl<'repo, A> Oid<'repo, A>
    where
        A: easy::Access + Sized,
    {
        pub fn ancestors(&self) -> Result<Ancestors<'repo, A>, Error> {
            let pack_cache = self.access.state().try_borrow_mut_pack_cache()?;
            let repo = self.access.repo()?;
            Ok(Ancestors {
                pack_cache,
                repo,
                access: self.access,
                tip: self.inner,
            })
        }
    }

    pub struct Iter<'a, 'repo, A>
    where
        A: easy::Access + Sized,
    {
        access: &'repo A,
        inner: Box<dyn Iterator<Item = Result<git_hash::ObjectId, git_traverse::commit::ancestors::Error>> + 'a>,
    }

    impl<'repo, A> Ancestors<'repo, A>
    where
        A: easy::Access + Sized,
    {
        pub fn all(&mut self) -> Iter<'_, 'repo, A> {
            Iter {
                access: self.access,
                inner: Box::new(git_traverse::commit::Ancestors::new(
                    Some(self.tip),
                    git_traverse::commit::ancestors::State::default(),
                    move |oid, buf| {
                        self.repo
                            .deref()
                            .odb
                            .try_find(oid, buf, self.pack_cache.deref_mut())
                            .ok()
                            .flatten()
                            .and_then(|obj| obj.try_into_commit_iter())
                    },
                )),
            }
        }
    }

    impl<'a, 'repo, A> Iterator for Iter<'a, 'repo, A>
    where
        A: easy::Access + Sized,
    {
        type Item = Result<Oid<'repo, A>, git_traverse::commit::ancestors::Error>;

        fn next(&mut self) -> Option<Self::Item> {
            self.inner.next().map(|res| res.map(|oid| oid.attach(self.access)))
        }
    }

    mod error {
        use crate::easy;

        #[derive(Debug, thiserror::Error)]
        pub enum Error {
            #[error(transparent)]
            BorrowRepo(#[from] easy::borrow::repo::Error),
            #[error(transparent)]
            BorrowBufMut(#[from] easy::borrow::state::Error),
        }
    }
    use error::Error;

    use crate::ext::ObjectIdExt;
}

mod impls {
    use git_hash::{oid, ObjectId};

    use crate::easy::{Object, ObjectRef, Oid};

    impl<'repo, A, B> PartialEq<Oid<'repo, A>> for Oid<'repo, B> {
        fn eq(&self, other: &Oid<'repo, A>) -> bool {
            self.inner == other.inner
        }
    }

    impl<'repo, A> PartialEq<ObjectId> for Oid<'repo, A> {
        fn eq(&self, other: &ObjectId) -> bool {
            &self.inner == other
        }
    }

    impl<'repo, A> PartialEq<oid> for Oid<'repo, A> {
        fn eq(&self, other: &oid) -> bool {
            self.inner == other
        }
    }

    impl<'repo, A, B> PartialEq<ObjectRef<'repo, A>> for Oid<'repo, B> {
        fn eq(&self, other: &ObjectRef<'repo, A>) -> bool {
            self.inner == other.id
        }
    }

    impl<'repo, A> PartialEq<Object> for Oid<'repo, A> {
        fn eq(&self, other: &Object) -> bool {
            self.inner == other.id
        }
    }

    impl<'repo, A> std::fmt::Debug for Oid<'repo, A> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.inner.fmt(f)
        }
    }

    impl<'repo, A> AsRef<oid> for Oid<'repo, A> {
        fn as_ref(&self) -> &oid {
            &self.inner
        }
    }

    impl<'repo, A> From<Oid<'repo, A>> for ObjectId {
        fn from(v: Oid<'repo, A>) -> Self {
            v.inner
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_of_oid() {
        assert_eq!(
            std::mem::size_of::<Oid<'_, crate::Easy>>(),
            32,
            "size of oid shouldn't change without notice"
        )
    }
}
