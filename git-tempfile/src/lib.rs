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

impl Drop for Registration {
    fn drop(&mut self) {
        REGISTER.take(self.index);
    }
}

pub fn new(containing_directory: impl AsRef<Path>) -> io::Result<Registration> {
    Ok(Registration {
        index: REGISTER
            .insert(NamedTempFile::new_in(containing_directory)?)
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "slab at capacity"))?,
    })
}
