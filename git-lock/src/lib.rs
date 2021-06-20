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

use git_tempfile::registration::Writable;
use std::time::Duration;

/// Describe what to do if a lock cannot be obtained as it's already held elsewhere.
pub enum Fail {
    /// Fail after the first unsuccessful attempt of obtaining a lock.
    Immediately,
    /// Retry after failure with exponentially longer sleep times to block the current thread.
    /// Fail once the given duration is exceeded, similar to [Fail::Immediately]
    AfterDurationWithBackoff(Duration),
}

/// Locks a resource to eventually be overwritten with the content of this file.
pub struct File {
    _inner: git_tempfile::Registration<Writable>,
}

mod file {
    use crate::{Fail, File};
    use std::path::{Path, PathBuf};

    mod error {
        use quick_error::quick_error;
        quick_error! {
            #[derive(Debug)]
            pub enum Error {
                Tbd
            }
        }
    }
    use error::Error;

    impl File {
        /// Create a writable lock file with failure `mode` whose content will eventually overwrite the given resource `at_path`.
        pub fn hold_to_update_resource(_at_path: impl AsRef<Path>, _mode: Fail) -> Result<File, Error> {
            Err(Error::Tbd)
        }

        /// Commit the changes written to this lock file and overwrite the original resource atomically, returning the resource path
        /// on success.
        pub fn commit(self) -> std::io::Result<PathBuf> {
            todo!("commit")
        }
    }
}
