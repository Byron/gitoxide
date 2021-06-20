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
#![deny(unsafe_code, rust_2018_idioms)]
#![allow(missing_docs)]

use git_tempfile::handle::{Closed, Writable};

const DOT_SUFFIX: &str = ".lock";

/// Locks a resource to eventually be overwritten with the content of this file.
///
/// Dropping the file without [committing][File::commit] will delete it, leaving the underlying resource unchanged.
pub struct File {
    inner: git_tempfile::Handle<Writable>,
}

/// Locks a resource for other markers or [files][File] that intend to update it.
///
/// As opposed to the [File] type this one won't keep the tempfile open for writing and thus consumes no
/// system resources.
pub struct Marker {
    _inner: git_tempfile::Handle<Closed>,
}

pub mod acquire {
    use crate::{File, Marker, DOT_SUFFIX};
    use git_tempfile::{AutoRemove, ContainingDirectory};
    use quick_error::quick_error;
    use std::{
        fmt,
        path::{Path, PathBuf},
        time::Duration,
    };

    /// Describe what to do if a lock cannot be obtained as it's already held elsewhere.
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
    pub enum Fail {
        /// Fail after the first unsuccessful attempt of obtaining a lock.
        Immediately,
        /// Retry after failure with exponentially longer sleep times to block the current thread.
        /// Fail once the given duration is exceeded, similar to [Fail::Immediately]
        AfterDurationWithBackoff(Duration),
    }

    impl Default for Fail {
        fn default() -> Self {
            Fail::Immediately
        }
    }

    impl fmt::Display for Fail {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Fail::Immediately => f.write_str("immediately"),
                Fail::AfterDurationWithBackoff(duration) => {
                    write!(f, "after {:.02}s", duration.as_secs_f32())
                }
            }
        }
    }

    quick_error! {
        #[derive(Debug)]
        pub enum Error {
            Io(err: std::io::Error) {
                display("Another IO error occurred while obtaining the lock")
                from()
                source(err)
            }
            PermanentlyLocked { resource_path: PathBuf, mode: Fail } {
                display("The lock for resource '{} could not be obtained {}. The lockfile at '{}{}' might need manual deletion.", resource_path.display(), mode, resource_path.display(), super::DOT_SUFFIX)
            }
        }
    }

    fn dir_cleanup(boundary: Option<PathBuf>) -> (ContainingDirectory, AutoRemove) {
        match boundary {
            None => (ContainingDirectory::Exists, AutoRemove::Tempfile),
            Some(boundary_directory) => (
                ContainingDirectory::CreateAllRaceProof(Default::default()),
                AutoRemove::TempfileAndEmptyParentDirectoriesUntil { boundary_directory },
            ),
        }
    }

    fn lock_with_mode<T>(
        resource: &Path,
        mode: Fail,
        boundary_directory: Option<PathBuf>,
        try_lock: impl Fn(&Path, ContainingDirectory, AutoRemove) -> std::io::Result<T>,
    ) -> Result<T, Error> {
        let (directory, cleanup) = dir_cleanup(boundary_directory);
        let lock_path = add_lock_suffix(resource);
        match mode {
            Fail::Immediately => try_lock(&lock_path, directory, cleanup).map_err(Error::from),
            Fail::AfterDurationWithBackoff(_duration) => todo!("fail after timeout"),
        }
    }

    fn add_lock_suffix(resource_path: &Path) -> PathBuf {
        resource_path.with_extension(resource_path.extension().map_or_else(
            || DOT_SUFFIX.to_string(),
            |ext| format!("{}{}", ext.to_string_lossy(), DOT_SUFFIX),
        ))
    }

    impl File {
        /// Create a writable lock file with failure `mode` whose content will eventually overwrite the given resource `at_path`.
        ///
        /// If `boundary_directory` is given, non-existing directories will be created automatically and removed in the case of
        /// a rollback. Otherwise the containing directory is expected to exist, even though the resource doesn't have to.
        pub fn acquire_to_update_resource(
            at_path: impl AsRef<Path>,
            mode: Fail,
            boundary_directory: Option<PathBuf>,
        ) -> Result<File, Error> {
            Ok(File {
                inner: lock_with_mode(at_path.as_ref(), mode, boundary_directory, |p, d, c| {
                    git_tempfile::writable_at(p, d, c)
                })?,
            })
        }
    }

    impl Marker {
        /// Like [`acquire_to_update_resource()`][File::acquire_to_update_resource()] but without the possibility to make changes
        /// and commit them.
        ///
        /// If `boundary_directory` is given, non-existing directories will be created automatically and removed in the case of
        /// a rollback.
        pub fn acquire_to_hold_resource(
            at_path: impl AsRef<Path>,
            mode: Fail,
            boundary_directory: Option<PathBuf>,
        ) -> Result<Marker, Error> {
            Ok(Marker {
                _inner: lock_with_mode(at_path.as_ref(), mode, boundary_directory, |p, d, c| {
                    git_tempfile::mark_at(p, d, c)
                })?,
            })
        }
    }
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
        pub fn commit(self) -> std::io::Result<()> {
            let tf = self.inner.take().expect("tempfile is always present");
            let resource_path = strip_lock_suffix(tf.path());
            tf.persist(resource_path)?;
            Ok(())
        }
    }
}
