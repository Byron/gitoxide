//! git-style registered tempfiles that are removed upon typical termination signals.
//!
//! This crate installs signal handlers the first time its facilities are used.
//! These are powered by [`signal-hook`] to get notified when the application is told to shut down
//! using signals to assure these are deleted. The deletion is filtered by process id to allow forks to have their own
//! set of tempfiles that won't get deleted when the parent process exits.
//!
//! As typical handlers for `TERMination` are installed on first use and effectively overriding the defaults, we install
//! default handlers to restore this behaviour.
//!
//! # Note
//!
//! Applications setting their own signal handlers on termination and want to be called after the ones of this crate
//! can call [`force_setup()`] to install handlers without other side-effects.
//!
//! # Limitations
//!
//! ## Tempfiles might remain on disk
//!
//! * Uninterruptible signals are received like `SIGKILL`
//! * The application is performing a write operation on the tempfile when a signal arrives, preventing this tempfile to be removed,
//!   but not others. Any other operation dealing with the tempfile suffers from the same issue.
//!
//! [signal-hook]: https://docs.rs/signal-hook
#![deny(unsafe_code, rust_2018_idioms)]
#![allow(missing_docs)]

use dashmap::DashMap;
use once_cell::sync::Lazy;
use std::{io, path::Path, sync::atomic::AtomicUsize};
use tempfile::NamedTempFile;

static SIGNAL_HANDLER_MODE: AtomicUsize = AtomicUsize::new(SignalHandlerMode::default() as usize);
static NEXT_MAP_INDEX: AtomicUsize = AtomicUsize::new(0);
static REGISTER: Lazy<DashMap<usize, Option<ForksafeTempfile>>> = Lazy::new(|| {
    for sig in signal_hook::consts::TERM_SIGNALS {
        // SAFETY: handlers are considered unsafe because a lot can go wrong. See `cleanup_tempfiles()` for details on safety.
        #[allow(unsafe_code)]
        unsafe { signal_hook::low_level::register(*sig, handler::cleanup_tempfiles) }
            .expect("signals can always be installed");
    }
    DashMap::new()
});

mod handler {
    use crate::REGISTER;

    /// Note that Mutexes of any kind are not allowed, and so aren't allocation or deallocation of memory.
    pub fn cleanup_tempfiles() {
        let current_pid = std::process::id();
        for mut tempfile in REGISTER.iter_mut() {
            if tempfile
                .as_ref()
                .map_or(false, |tf| tf.owning_process_id == current_pid)
            {
                if let Some(tempfile) = tempfile.take() {
                    let (file, temppath) = tempfile.inner.into_parts();
                    std::fs::remove_file(&temppath).ok();
                    std::mem::forget(temppath); // leak memory to prevent deallocation
                    file.sync_all().ok();
                }
            }
        }
    }
}

pub enum SignalHandlerMode {
    HandleTermination = 0,
    HandleTerminationAndRestoreDefaultBehaviour = 1,
}

impl SignalHandlerMode {
    const fn default() -> Self {
        #[cfg(not(test))]
        return SignalHandlerMode::HandleTerminationAndRestoreDefaultBehaviour;
        #[cfg(test)]
        return SignalHandlerMode::HandleTermination;
    }
}

/// # Note
///
/// Signals interrupting the calling thread right after taking ownership of the registered tempfile
/// will cause all but this tempfile to be removed automatically. In the common case it will persist on disk as destructors
/// were not called or didn't get to remove the file.
///
/// In the best case the file is a true temporary with a non-clashing name that 'only' fills up the disk,
/// in the worst case the temporary file is used as a lock file which may leave the repository in a locked
/// state forever.
///
/// This kind of raciness exists whenever [`take()`][Registration::take()] is used and can't be circumvented.
pub struct Registration {
    id: usize,
}

struct ForksafeTempfile {
    inner: NamedTempFile,
    owning_process_id: u32,
}

impl From<NamedTempFile> for ForksafeTempfile {
    fn from(inner: NamedTempFile) -> Self {
        ForksafeTempfile {
            inner,
            owning_process_id: std::process::id(),
        }
    }
}

mod registration {
    use crate::{Registration, NEXT_MAP_INDEX, REGISTER};
    use std::{io, path::Path};
    use tempfile::NamedTempFile;

    impl Registration {
        pub fn at_path(path: impl AsRef<Path>) -> io::Result<Registration> {
            let path = path.as_ref();
            let id = NEXT_MAP_INDEX.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            expect_none(REGISTER.insert(
                id,
                Some({
                    let mut builder = tempfile::Builder::new();
                    let dot_ext_storage;
                    match path.file_stem() {
                        Some(stem) => builder.prefix(stem),
                        None => builder.prefix(""),
                    };
                    if let Some(ext) = path.extension() {
                        dot_ext_storage = format!(".{}", ext.to_string_lossy());
                        builder.suffix(&dot_ext_storage);
                    }
                    builder
                        .rand_bytes(0)
                        .tempfile_in(path.parent().expect("parent directory is present"))?
                        .into()
                }),
            ));
            Ok(Registration { id })
        }

        pub fn new(containing_directory: impl AsRef<Path>) -> io::Result<Registration> {
            let id = NEXT_MAP_INDEX.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            expect_none(REGISTER.insert(id, Some(NamedTempFile::new_in(containing_directory)?.into())));
            Ok(Registration { id })
        }

        /// Take ownership of the temporary file.
        ///
        pub fn take(self) -> Option<NamedTempFile> {
            let res = REGISTER.remove(&self.id);
            std::mem::forget(self);
            res.and_then(|(_k, v)| v.map(|v| v.inner))
        }
    }

    fn expect_none<T>(v: Option<T>) {
        assert!(
            v.is_none(),
            "there should never be conflicts or old values as ids are never reused."
        );
    }

    impl Drop for Registration {
        fn drop(&mut self) {
            REGISTER.remove(&self.id);
        }
    }
}

pub fn new(containing_directory: impl AsRef<Path>) -> io::Result<Registration> {
    Registration::new(containing_directory)
}

pub fn at_path(path: impl AsRef<Path>) -> io::Result<Registration> {
    Registration::at_path(path)
}

/// Explicitly (instead of lazily) initialize signal handlers and other state to keep track of tempfiles.
/// Only has an effect the first time it is called.
///
/// This is required if the application wants to install their own signal handlers _after_ the ones defined here.
pub fn force_setup(mode: SignalHandlerMode) {
    SIGNAL_HANDLER_MODE.store(mode as usize, std::sync::atomic::Ordering::Relaxed);
    Lazy::force(&REGISTER);
}
