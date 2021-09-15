//!
use std::{
    cell::Ref,
    ops::{Deref, DerefMut},
    path::Path,
};

use git_odb::Find;
use git_ref::file::ReferenceExt;

use crate::easy;

/// A platform to create iterators over references.
#[must_use = "Iterators should be obtained from this iterator platform"]
pub struct Platform<'r, A>
where
    A: easy::Access + Sized,
{
    pub(crate) repo: A::RepoRef,
    pub(crate) packed_refs: Ref<'r, easy::reference::packed::ModifieablePackedRefsBuffer>,
    pub(crate) access: &'r A,
}

/// An iterator over references, with or without filter.
pub struct Iter<'r, A> {
    inner: git_ref::file::iter::LooseThenPacked<'r, 'r>,
    packed_refs: Option<&'r git_ref::packed::Buffer>,
    peel: bool,
    access: &'r A,
}

impl<'r, A> Platform<'r, A>
where
    A: easy::Access + Sized,
{
    /// Return an iterator over all references in the repository.
    ///
    /// Even broken or otherwise unparsible or inaccessible references are returned and have to be handled by the caller on a
    /// case by case basis.
    pub fn all(&self) -> Result<Iter<'_, A>, init::Error> {
        let repo = self.repo.deref();
        Ok(Iter {
            inner: repo.refs.iter(self.packed_refs.buffer.as_ref())?,
            packed_refs: self.packed_refs.buffer.as_ref(),
            peel: false,
            access: self.access,
        })
    }

    /// Return an iterator over all references that match the given `prefix`.
    ///
    /// These are of the form `refs/heads` or `refs/remotes/origin`, and must not contain relative paths components like `.` or `..`.
    pub fn prefixed(&self, prefix: impl AsRef<Path>) -> Result<Iter<'_, A>, init::Error> {
        let repo = self.repo.deref();
        Ok(Iter {
            inner: repo.refs.iter_prefixed(self.packed_refs.buffer.as_ref(), prefix)?,
            packed_refs: self.packed_refs.buffer.as_ref(),
            peel: false,
            access: self.access,
        })
    }
}

impl<'r, A> Iter<'r, A> {
    /// Automatically peel references before yielding them during iteration.
    ///
    /// This has the same effect as using `iter.map(|r| {r.peel_to_id_in_place(); r})`.
    ///
    /// # Note
    ///
    /// Doing this is necessary as the packed-refs buffer is already held by the iterator, disallowing the consumer of the iterator
    /// to peel the returned references themselves.
    pub fn peeled(mut self) -> Self {
        self.peel = true;
        self
    }
}

impl<'r, A> Iterator for Iter<'r, A>
where
    A: easy::Access + Sized,
{
    type Item = Result<easy::Reference<'r, A>, Box<dyn std::error::Error + Send + Sync + 'static>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|res| {
            res.map_err(|err| Box::new(err) as Box<dyn std::error::Error + Send + Sync + 'static>)
                .and_then(|mut r| {
                    if self.peel {
                        let repo = self.access.repo()?;
                        let state = self.access.state();
                        let mut pack_cache = state.try_borrow_mut_pack_cache()?;
                        r.peel_to_id_in_place(&repo.refs, self.packed_refs, |oid, buf| {
                            repo.odb
                                .try_find(oid, buf, pack_cache.deref_mut())
                                .map(|po| po.map(|o| (o.kind, o.data)))
                        })
                        .map_err(|err| Box::new(err) as Box<dyn std::error::Error + Send + Sync + 'static>)
                        .map(|_| r)
                    } else {
                        Ok(r)
                    }
                })
                .map(|r| easy::Reference::from_ref(r, self.access))
        })
    }
}

///
pub mod init {
    /// The error returned by [`Platform::all()`][super::Platform::all()] or [`Platform::prefixed()`][super::Platform::prefixed()].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error(transparent)]
        Io(#[from] std::io::Error),
    }
}

///
mod error {
    use crate::easy;

    /// The error returned by [ReferenceAccessExt::references()][easy::ext::ReferenceAccessExt::references()].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error(transparent)]
        PackedRefsOpen(#[from] git_ref::packed::buffer::open::Error),
        #[error("BUG: Part of interior state could not be borrowed.")]
        BorrowState(#[from] easy::borrow::state::Error),
        #[error("BUG: The repository could not be borrowed")]
        BorrowRepo(#[from] easy::borrow::repo::Error),
    }

    impl From<easy::reference::packed::Error> for Error {
        fn from(err: easy::reference::packed::Error) -> Self {
            match err {
                easy::reference::packed::Error::PackedRefsOpen(err) => Error::PackedRefsOpen(err),
                easy::reference::packed::Error::BorrowState(err) => Error::BorrowState(err),
            }
        }
    }
}
pub use error::Error;
