use std::io;
use std::path::Path;

use bstr::BStr;
use gix_index as index;

use super::Modification;

/// The status of an index entry in a worktree
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Status {
    /// The file in the worktree is identical to the index entry
    Unchanged,
    /// An index entry has no corresponding file in the worktree.
    Removed,
    /// A worktree file has been modified in some form as indicated by `change`.
    ///
    /// Note that this doesn't necessarily mean that the *content* of the file changed.
    Modified(Modification),
    /// An index entry that correspond to an untracked worktree file marked with `git add`
    Added,
}

/// The error returned by [`compare_to_index()`].
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error("Could not convert path to UTF8")]
    IllformedUtf8,
    #[error("The clock was off when reading file related metadata after updating a file on disk")]
    Time(#[from] std::time::SystemTimeError),
    #[error("IO error while writing blob or reading file metadata or changing filetype")]
    Io(#[from] io::Error),
}

///
pub trait Visit<'index> {
    ///
    fn visit_entry(
        &mut self,
        entry: &'index index::Entry,
        status: Result<Status, Error>,
        path: Result<&Path, &BStr>,
        conflict: bool,
    );
}

///
pub trait VisitPrallel<'index> {
    ///
    fn visit_entry(
        &self,
        entry: &'index index::Entry,
        status: Result<Status, Error>,
        path: Result<&Path, &BStr>,
        conflict: bool,
    );
}
