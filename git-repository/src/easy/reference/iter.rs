//!
use std::cell::Ref;

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
            inner: repo.refs.iter(self.packed_refs.packed_refs.as_ref())?,
            access: self.access,
        })
    }

    /// Return an iterator over all references that match the given `prefix`.
    ///
    /// These are of the form `refs/heads` or `refs/remotes/origin`, and must not contain relative paths components like `.` or `..`.
    pub fn prefixed(&self, prefix: impl AsRef<Path>) -> Result<Iter<'_, A>, init::Error> {
        let repo = self.repo.deref();
        Ok(Iter {
            inner: repo.refs.iter_prefixed(self.packed_refs.packed_refs.as_ref(), prefix)?,
            access: self.access,
        })
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
                .map(|r| easy::Reference::from_ref(r, self.access))
        })
    }
}

///
pub mod init {
    /// The error returned by [`State::all()`][super::State::all()] or [`State::prefixed()`][super::State::prefixed()].
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

use std::{ops::Deref, path::Path};

pub use error::Error;
