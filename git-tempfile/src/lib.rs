//! git-style registered tempfiles that are removed upon typical termination signals.
//!
//! This crate installs signal handlers the first time its facilities are used.
//! These are powered by [`signal-hook`] to get notified when the application is told to shut down
//! using signals to assure these are deleted. The deletion is filtered by process id to allow forks to have their own
//! set of tempfiles that won't get deleted when the parent process exits.
//!
//! As typical handlers for `TERMination` are installed on first use and effectively overriding the defaults, we install
//! default handlers to restore this behaviour. Whether or not to do that can be controlled using [`force_setup()`].
//!
//! # Note
//!
//! Applications setting their own signal handlers on termination and want to be called after the ones of this crate
//! can call [`force_setup()`] to install their own handlers.
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
#![deny(missing_docs, unsafe_code, rust_2018_idioms)]

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
        unsafe { signal_hook_registry::register_sigaction(*sig, handler::cleanup_tempfiles) }
            .expect("signals can always be installed");
    }
    DashMap::new()
});

mod handler {
    use crate::{SignalHandlerMode, REGISTER, SIGNAL_HANDLER_MODE};
    use libc::siginfo_t;

    /// # Safety
    /// Note that Mutexes of any kind are not allowed, and so aren't allocation or deallocation of memory.
    /// We are usign lock-free datastructures and sprinkle in `std::mem::forget` to avoid deallocating.
    pub fn cleanup_tempfiles(sig: &siginfo_t) {
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
        let restore_original_behaviour =
            SignalHandlerMode::DeleteTempfilesOnTerminationAndRestoreDefaultBehaviour as usize;
        if SIGNAL_HANDLER_MODE.load(std::sync::atomic::Ordering::SeqCst) == restore_original_behaviour {
            signal_hook::low_level::emulate_default_handler(sig.si_signo).ok();
        }
    }

    #[cfg(test)]
    mod tests {
        use std::path::Path;
        fn filecount_in(path: impl AsRef<Path>) -> usize {
            std::fs::read_dir(path).expect("valid dir").count()
        }

        #[test]
        fn various_termination_signals_remove_tempfiles_unconditionally() -> Result<(), Box<dyn std::error::Error>> {
            let dir = tempfile::tempdir()?;
            for sig in signal_hook::consts::TERM_SIGNALS {
                let _tempfile = crate::new(dir.path())?;
                assert_eq!(
                    filecount_in(dir.path()),
                    1,
                    "only one tempfile exists no matter the iteration"
                );
                signal_hook::low_level::raise(*sig)?;
                assert_eq!(
                    filecount_in(dir.path()),
                    0,
                    "the signal triggers removal but won't terminate the process (anymore)"
                );
            }
            Ok(())
        }
    }
}

/// Define how our signal handlers act
pub enum SignalHandlerMode {
    /// Delete all remaining registered tempfiles on termination.
    DeleteTempfilesOnTermination = 0,
    /// Delete all remaining registered tempfiles on termination and emulate the default handler behaviour.
    ///
    /// This is the default, which leads to the process to be aborted.
    DeleteTempfilesOnTerminationAndRestoreDefaultBehaviour = 1,
}

impl SignalHandlerMode {
    /// By default we will emulate the default behaviour and abort the process.
    ///
    /// While testing, we will not abort the process.
    const fn default() -> Self {
        #[cfg(not(test))]
        return SignalHandlerMode::DeleteTempfilesOnTerminationAndRestoreDefaultBehaviour;
        #[cfg(test)]
        return SignalHandlerMode::DeleteTempfilesOnTermination;
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
        /// Create a registered tempfile at the given `path`, where `path` includes the desired filename.
        ///
        /// **Note** that intermediate directories will _not_ be created.
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

        /// Create a registered tempfile within `containing_directory` with a name that won't clash.
        /// **Note** that intermediate directories will _not_ be created.
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

/// A shortcut to [`Registration::new()`].
pub fn new(containing_directory: impl AsRef<Path>) -> io::Result<Registration> {
    Registration::new(containing_directory)
}

/// A shortcut to [`Registration::at_path()`].
pub fn at_path(path: impl AsRef<Path>) -> io::Result<Registration> {
    Registration::at_path(path)
}

/// Explicitly (instead of lazily) initialize signal handlers and other state to keep track of tempfiles.
/// Only has an effect the first time it is called and furthermore allows to set the `mode` in which signal handlers
/// are installed.
///
/// This is required if the application wants to install their own signal handlers _after_ the ones defined here.
pub fn force_setup(mode: SignalHandlerMode) {
    SIGNAL_HANDLER_MODE.store(mode as usize, std::sync::atomic::Ordering::Relaxed);
    Lazy::force(&REGISTER);
}
