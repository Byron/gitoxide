//!
use std::path::Path;

use git_odb::pack::Find;
use git_ref::file::ReferenceExt;

/// A platform to create iterators over references.
#[must_use = "Iterators should be obtained from this iterator platform"]
pub struct Platform<'r> {
    pub(crate) platform: git_ref::file::iter::Platform<'r>,
    pub(crate) repo: &'r crate::Repository,
}

/// An iterator over references, with or without filter.
pub struct Iter<'r> {
    inner: git_ref::file::iter::LooseThenPacked<'r, 'r>,
    peel: bool,
    repo: &'r crate::Repository,
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
            repo: self.repo,
        })
    }

    /// Return an iterator over all references that match the given `prefix`.
    ///
    /// These are of the form `refs/heads` or `refs/remotes/origin`, and must not contain relative paths components like `.` or `..`.
    // TODO: Create a custom `Path` type that enforces the requirements of git naturally, this type is surprising possibly on windows
    //       and when not using a trailing '/' to signal directories.
    pub fn prefixed(&self, prefix: impl AsRef<Path>) -> Result<Iter<'_>, init::Error> {
        Ok(Iter {
            inner: self.platform.prefixed(prefix)?,
            peel: false,
            repo: self.repo,
        })
    }

    // TODO: tests
    /// Return an iterator over all references that are tags.
    ///
    /// They are all prefixed with `refs/tags`.
    pub fn tags(&self) -> Result<Iter<'_>, init::Error> {
        Ok(Iter {
            inner: self.platform.prefixed("refs/tags/")?,
            peel: false,
            repo: self.repo,
        })
    }

    // TODO: tests
    /// Return an iterator over all branches.
    ///
    /// They are all prefixed with `refs/heads`.
    pub fn branches(&self) -> Result<Iter<'_>, init::Error> {
        Ok(Iter {
            inner: self.platform.prefixed("refs/heads/")?,
            peel: false,
            repo: self.repo,
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
    type Item = Result<crate::Reference<'r>, Box<dyn std::error::Error + Send + Sync + 'static>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|res| {
            res.map_err(|err| Box::new(err) as Box<dyn std::error::Error + Send + Sync + 'static>)
                .and_then(|mut r| {
                    if self.peel {
                        let handle = &self.repo;
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
                .map(|r| crate::Reference::from_ref(r, self.repo))
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

/// The error returned by [references()][crate::Repository::references()].
pub type Error = git_ref::packed::buffer::open::Error;
