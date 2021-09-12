#![allow(missing_docs)]
use std::cell::Ref;

use crate::easy;

/// An iterator over references
#[must_use]
pub struct State<'r, A>
where
    A: easy::Access + Sized,
{
    pub(crate) repo: A::RepoRef,
    pub(crate) packed_refs: Ref<'r, easy::reference::packed::ModifieablePackedRefsBuffer>,
    pub(crate) access: &'r A,
}

pub struct Iter<'r, A> {
    inner: git_ref::file::iter::LooseThenPacked<'r, 'r>,
    access: &'r A,
}

impl<'r, A> State<'r, A>
where
    A: easy::Access + Sized,
{
    pub fn all(&self) -> Result<Iter<'_, A>, init::Error> {
        let repo = self.repo.deref();
        Ok(Iter {
            inner: repo.refs.iter(self.packed_refs.packed_refs.as_ref())?,
            access: self.access,
        })
    }

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

pub mod init {
    #[derive(Debug, thiserror::Error)]
    pub enum Error {
        #[error(transparent)]
        Io(#[from] std::io::Error),
    }
}

mod error {
    use crate::easy;

    #[derive(Debug, thiserror::Error)]
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
