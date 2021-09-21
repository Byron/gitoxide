//!
use std::ops::Deref;

use git_hash::{oid, ObjectId};

use crate::{
    easy,
    easy::{ext::ObjectAccessExt, object::find, ObjectRef, Oid},
};

/// An [object id][ObjectId] infused with `Easy`.
impl<'repo, A> Oid<'repo, A>
where
    A: easy::Access + Sized,
{
    /// Find the [`ObjectRef`] associated with this object id, and consider it an error if it doesn't exist.
    ///
    /// # Note
    ///
    /// There can only be one `ObjectRef` per `Easy`. To increase that limit, clone the `Easy`.
    pub fn object(&self) -> Result<ObjectRef<'repo, A>, find::existing::Error> {
        self.access.find_object(self.inner)
    }

    /// Try to find the [`ObjectRef`] associated with this object id, and return `None` if it's not available locally.
    ///
    /// # Note
    ///
    /// There can only be one `ObjectRef` per `Easy`. To increase that limit, clone the `Easy`.
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

/// A platform to traverse commit ancestors, also referred to as commit history.
pub struct Ancestors<'repo, A>
where
    A: easy::Access + Sized,
{
    repo: A::RepoRef,
    access: &'repo A,
    tips: Box<dyn Iterator<Item = ObjectId>>,
}

///
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
        /// Obtain a platform for traversing ancestors of this commit.
        pub fn ancestors(&self) -> Result<Ancestors<'repo, A>, Error> {
            let repo = self.access.repo()?;
            Ok(Ancestors {
                repo,
                access: self.access,
                tips: Box::new(Some(self.inner).into_iter()),
            })
        }
    }

    impl<'repo, A> Ancestors<'repo, A>
    where
        A: easy::Access + Sized,
    {
        /// Return an iterator to traverse all commits in the history of the commit the parent [Oid] is pointing to.
        pub fn all(&mut self) -> Iter<'_, 'repo, A> {
            let tips = std::mem::replace(&mut self.tips, Box::new(None.into_iter()));
            Iter {
                access: self.access,
                inner: Box::new(git_traverse::commit::Ancestors::new(
                    tips,
                    git_traverse::commit::ancestors::State::default(),
                    move |oid, buf| {
                        let state = self.access.state();
                        let mut object_cache = state.try_borrow_mut_object_cache().ok()?;
                        if let Some(c) = object_cache.deref_mut() {
                            if let Some(kind) = c.get(&oid.to_owned(), buf) {
                                return git_pack::data::Object::new(kind, buf).try_into_commit_iter();
                            }
                        }
                        match self
                            .repo
                            .deref()
                            .odb
                            .try_find(
                                oid,
                                buf,
                                state
                                    .try_borrow_mut_pack_cache()
                                    .expect("BUG: pack cache is already borrowed")
                                    .deref_mut(),
                            )
                            .ok()
                            .flatten()
                            .and_then(|obj| obj.try_into_commit_iter())
                        {
                            Some(_) => {
                                if let Some(c) = object_cache.deref_mut() {
                                    c.put(oid.to_owned(), git_object::Kind::Commit, buf);
                                }
                                Some(git_object::CommitRefIter::from_bytes(buf))
                            }
                            None => None,
                        }
                    },
                )),
            }
        }
    }

    /// The iterator returned by [`Ancestors::all()`].
    pub struct Iter<'a, 'repo, A>
    where
        A: easy::Access + Sized,
    {
        access: &'repo A,
        inner: Box<dyn Iterator<Item = Result<git_hash::ObjectId, git_traverse::commit::ancestors::Error>> + 'a>,
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

        /// The error returned by [`Oid::ancestors()`][super::Oid::ancestors()].
        #[derive(Debug, thiserror::Error)]
        #[allow(missing_docs)]
        pub enum Error {
            #[error(transparent)]
            BorrowRepo(#[from] easy::borrow::repo::Error),
            #[error(transparent)]
            BorrowBufMut(#[from] easy::borrow::state::Error),
        }
    }
    pub use error::Error;

    use crate::ext::ObjectIdExt;
    use git_pack::cache::Object;
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
