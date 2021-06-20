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

const SUFFIX: &str = ".lock";

/// Locks a resource to eventually be overwritten with the content of this file.
///
/// Dropping the file without [committing][File::commit] will delete it, leaving the underlying resource unchanged.
pub struct File {
    _inner: git_tempfile::Handle<Writable>,
}

/// Locks a resource for other markers or [files][File] that intend to update it.
///
/// As opposed to the [File] type this one won't keep the tempfile open for writing and thus consumes no
/// system resources.
pub struct Marker {
    _inner: git_tempfile::Handle<Closed>,
}

pub mod acquire {
    use crate::{File, Marker};
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
                display("The lock for resource '{} could not be obtained {}. The lockfile at '{}{}' might need manual deletion.", resource_path.display(), mode, resource_path.display(), super::SUFFIX)
            }
        }
    }

    impl File {
        /// Create a writable lock file with failure `mode` whose content will eventually overwrite the given resource `at_path`.
        ///
        /// If `boundary_directory` is given, non-existing directories will be created automatically and removed in the case of
        /// a rollback. Otherwise the containing directory is expected to exist, even though the resource doesn't have to.
        pub fn acquire_to_update_resource(
            _at_path: impl AsRef<Path>,
            _mode: Fail,
            _boundary_directory: Option<PathBuf>,
        ) -> Result<File, Error> {
            todo!("acquire file")
        }
    }

    impl Marker {
        /// Like [`acquire_to_update_resource()`][File::acquire_to_update_resource()] but without the possibility to make changes
        /// and commit them.
        ///
        /// If `boundary_directory` is given, non-existing directories will be created automatically and removed in the case of
        /// a rollback.
        pub fn acquire_to_hold_resource(
            _at_path: impl AsRef<Path>,
            _mode: Fail,
            _boundary_directory: Option<PathBuf>,
        ) -> Result<Marker, Error> {
            todo!("acquire marker")
        }
    }
}

///
pub mod file {
    use crate::File;

    impl File {
        /// Obtain a mutable reference to the write handle and call `f(out)` with it.
        pub fn with_mut<T>(&mut self, _f: impl FnOnce(&mut std::fs::File) -> std::io::Result<T>) -> std::io::Result<T> {
            todo!("with mut")
        }
        /// Commit the changes written to this lock file and overwrite the original resource atomically, returning the resource path
        /// on success.
        pub fn commit(self) -> std::io::Result<()> {
            todo!("commit")
        }
    }
}
