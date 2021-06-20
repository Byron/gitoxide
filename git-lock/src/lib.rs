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

const DOT_SUFFIX: &str = ".lock";

pub mod acquire;

/// Locks a resource to eventually be overwritten with the content of this file.
///
/// Dropping the file without [committing][File::commit] will delete it, leaving the underlying resource unchanged.
pub struct File {
    inner: git_tempfile::Handle<Writable>,
}

/// Locks a resource to allow related resources to be updated using [files][File].
///
/// As opposed to the [File] type this one won't keep the tempfile open for writing and thus consumes no
/// system resources, nor can it be persisted.
pub struct Marker {
    _inner: git_tempfile::Handle<Closed>,
}

///
pub mod file {
    use crate::{File, DOT_SUFFIX};
    use std::path::{Path, PathBuf};

    fn strip_lock_suffix(lock_path: &Path) -> PathBuf {
        lock_path.with_extension(lock_path.extension().map_or("".to_string(), |ext| {
            let ext = ext.to_string_lossy();
            ext.split_at(ext.len().saturating_sub(DOT_SUFFIX.len())).0.to_string()
        }))
    }

    impl File {
        /// Obtain a mutable reference to the write handle and call `f(out)` with it.
        pub fn with_mut<T>(&mut self, f: impl FnOnce(&mut std::fs::File) -> std::io::Result<T>) -> std::io::Result<T> {
            self.inner.with_mut(|tf| f(tf.as_file_mut())).and_then(|res| res)
        }
        /// Commit the changes written to this lock file and overwrite the original resource atomically, returning the resource path
        /// on success.
        ///
        /// If a file is not committed, it will be deleted on drop or on signal.
        pub fn commit(self) -> std::io::Result<()> {
            let tf = self.inner.take().expect("tempfile is always present");
            let resource_path = strip_lock_suffix(tf.path());
            tf.persist(resource_path)?;
            Ok(())
        }
    }
}
