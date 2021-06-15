//! git-style registered tempfiles that are removed upon typical termination signals.
//!
//! This crate installs signal handlers powered by [`signal-hook`] to get notified when the application is told to shut down
//! using signals to assure these are deleted.
//!
//! As typical handlers for `TERM`ination are installed on first use and effectively overriding the defaults, we install
//! default handlers to restore this behaviour.
//!
//! # Why tempfiles might remain on disk nonetheless
//!
//! * Uninterruptible signals are received like `SIGKILL`
//! * The application is performing a write operation on the tempfile when a signal arrives, preventing this tempfile to be removed,
//!   but not others.
//!
//! # Limitations
//!
//! * The amount of temporary files open at a time are not only limited by the amount of open file handles, but also
//!   the amount of items storable in the concurrent slab serving as backend.
//!
//! [signal-hook]: https://docs.rs/signal-hook
#![deny(unsafe_code, rust_2018_idioms)]
#![allow(missing_docs)]

use once_cell::sync::Lazy;
use sharded_slab::Slab;
use std::{io, path::Path};
use tempfile::NamedTempFile;

static REGISTER: Lazy<Slab<NamedTempFile>> = Lazy::new(|| Slab::new());

pub struct Registration {
    index: usize,
}

mod registration {
    use crate::{Registration, REGISTER};
    use std::{io, path::Path};
    use tempfile::NamedTempFile;

    impl Registration {
        pub fn new(containing_directory: impl AsRef<Path>) -> io::Result<Registration> {
            Ok(Registration {
                index: REGISTER
                    .insert(NamedTempFile::new_in(containing_directory)?)
                    .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "slab at capacity"))?,
            })
        }

        /// Take ownership of the temporary file.
        ///
        /// # Note
        ///
        /// Signals interrupting the calling thread right after taking ownership will cause all but this
        /// tempfile to be removed automatically. In the common case it will persist on disk as destructors
        /// were not called.
        ///
        /// In the best case the file is a true temporary with a non-clashing name that 'only' fills up the disk,
        /// in the worst case the temporary file is used as a lock file which may leave the repository in a locked
        /// state forever.
        pub fn take(self) -> Option<NamedTempFile> {
            let res = REGISTER.take(self.index);
            std::mem::forget(self); // no need for another slab access in destructor
            res
        }
    }

    impl Drop for Registration {
        fn drop(&mut self) {
            REGISTER.take(self.index);
        }
    }
}

pub fn new(containing_directory: impl AsRef<Path>) -> io::Result<Registration> {
    Registration::new(containing_directory)
}
