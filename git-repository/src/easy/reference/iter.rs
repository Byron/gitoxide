//!
use std::path::Path;

use git_odb::pack::Find;
use git_ref::file::ReferenceExt;

use crate::easy;

/// A platform to create iterators over references.
#[must_use = "Iterators should be obtained from this iterator platform"]
pub struct Platform<'r> {
    pub(crate) platform: git_ref::file::iter::Platform<'r>,
    pub(crate) handle: &'r easy::Handle,
}

/// An iterator over references, with or without filter.
pub struct Iter<'r> {
    inner: git_ref::file::iter::LooseThenPacked<'r, 'r>,
    peel: bool,
    handle: &'r easy::Handle,
}

impl<'r> Platform<'r> {
    /// Return an iterator over all references in the repository.
    ///
    /// Even broken or otherwise unparsible or inaccessible references are returned and have to be handled by the caller on a
    /// case by case basis.
    pub fn all(&self) -> Result<Iter<'_>, init::Error> {
        Ok(Iter {
            inner: self.platform.all()?,
            peel: false,
            handle: self.handle,
        })
    }

    /// Return an iterator over all references that match the given `prefix`.
    ///
    /// These are of the form `refs/heads` or `refs/remotes/origin`, and must not contain relative paths components like `.` or `..`.
    pub fn prefixed(&self, prefix: impl AsRef<Path>) -> Result<Iter<'_>, init::Error> {
        Ok(Iter {
            inner: self.platform.prefixed(prefix)?,
            peel: false,
            handle: self.handle,
        })
    }
}

impl<'r> Iter<'r> {
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

impl<'r> Iterator for Iter<'r> {
    type Item = Result<easy::Reference<'r>, Box<dyn std::error::Error + Send + Sync + 'static>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|res| {
            res.map_err(|err| Box::new(err) as Box<dyn std::error::Error + Send + Sync + 'static>)
                .and_then(|mut r| {
                    if self.peel {
                        let handle = &self.handle;
                        r.peel_to_id_in_place(&handle.refs, |oid, buf| {
                            handle
                                .objects
                                .try_find(oid, buf)
                                .map(|po| po.map(|(o, _l)| (o.kind, o.data)))
                        })
                        .map_err(|err| Box::new(err) as Box<dyn std::error::Error + Send + Sync + 'static>)
                        .map(|_| r)
                    } else {
                        Ok(r)
                    }
                })
                .map(|r| easy::Reference::from_ref(r, self.handle))
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
    }
}
pub use error::Error;
