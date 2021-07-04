//! git-style registered lock files to make altering resources atomic.
//!
//! In this model, reads are always atomic and can be performed directly while writes are facilitated by a locking mechanism
//! implemented here.
//!
//! Lock files mostly `git-tempfile` with its auto-cleanup and the following:
//!
//! * consistent naming of lock files
//! * block the thread (with timeout) or fail immediately if a lock cannot be obtained right away
//! * commit lock files to atomically put them into the location of the originally locked file
//!
//! # Limitations
//!
//! * As the lock file is separate from the actual resource, locking is merely a convention rather than being enforced.
//! * The limitations of `git-tempfile` apply.
#![deny(missing_docs, unsafe_code, rust_2018_idioms)]

use git_tempfile::handle::{Closed, Writable};

const DOT_LOCK_SUFFIX: &str = ".lock";

pub mod acquire;
mod backoff;

/// Locks a resource to eventually be overwritten with the content of this file.
///
/// Dropping the file without [committing][File::commit] will delete it, leaving the underlying resource unchanged.
#[must_use = "A File that is immediately dropped doesn't allow resource updates"]
#[derive(Debug)]
pub struct File {
    inner: git_tempfile::Handle<Writable>,
}

/// Locks a resource to allow related resources to be updated using [files][File].
///
/// As opposed to the [File] type this one won't keep the tempfile open for writing and thus consumes no
/// system resources, nor can it be persisted.
#[must_use = "A Marker that is immediately dropped doesn't lock a resource meaningfully"]
#[derive(Debug)]
pub struct Marker {
    inner: git_tempfile::Handle<Closed>,
    created_from_file: bool,
}

///
pub mod file;
