//! git-style registered lock files to make altering resources atomic.
//!
//! In this model, reads are always atomic and can be performed directly while writes are facilitated by the locking mechanism
//! implemented here. Locks are acquired atomically, then written to, to finally atomically overwrite the actual resource.
//!
//! Lock files are wrapped [`gix-tempfile`](gix_tempfile)-handles and add the following:
//!
//! * consistent naming of lock files
//! * block the thread (with timeout) or fail immediately if a lock cannot be obtained right away
//! * commit lock files to atomically put them into the location of the originally locked file
//!
//! # Limitations
//!
//! * [All limitations of `gix-tempfile`](gix_tempfile) apply. **A highlight of such a limitation is resource leakage
//!   which results in them being permanently locked unless there is user-intervention.**
//! * As the lock file is separate from the actual resource, locking is merely a convention rather than being enforced.
#![deny(missing_docs, rust_2018_idioms, unsafe_code)]

use gix_tempfile::handle::{Closed, Writable};
use std::path::PathBuf;

pub use gix_tempfile as tempfile;

const DOT_LOCK_SUFFIX: &str = ".lock";

///
pub mod acquire;

pub use gix_utils::backoff;
///
pub mod commit;

/// Locks a resource to eventually be overwritten with the content of this file.
///
/// Dropping the file without [committing][File::commit] will delete it, leaving the underlying resource unchanged.
#[must_use = "A File that is immediately dropped doesn't allow resource updates"]
#[derive(Debug)]
pub struct File {
    inner: gix_tempfile::Handle<Writable>,
    lock_path: PathBuf,
}

/// Locks a resource to allow related resources to be updated using [files][File].
///
/// As opposed to the [File] type this one won't keep the tempfile open for writing and thus consumes no
/// system resources, nor can it be persisted.
#[must_use = "A Marker that is immediately dropped doesn't lock a resource meaningfully"]
#[derive(Debug)]
pub struct Marker {
    inner: gix_tempfile::Handle<Closed>,
    created_from_file: bool,
    lock_path: PathBuf,
}

///
pub mod file;
