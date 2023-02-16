//! git-style registered tempfiles that are removed upon typical termination signals.
//!
//! To register signal handlers in a typical application that doesn't have its own, call
//! [`gix_tempfile::setup(Default::default())`][setup()] before creating the first tempfile.
//!
//! Signal handlers are powered by [`signal-hook`] to get notified when the application is told to shut down
//! to assure tempfiles are deleted. The deletion is filtered by process id to allow forks to have their own
//! set of tempfiles that won't get deleted when the parent process exits.
//!
//! ### Initial Setup
//!
//! As no handlers for `TERMination` are installed, it is required to call [`setup()`] before creating the first tempfile.
//! This also allows to control how `git-tempfiles` integrates with other handlers under application control.
//!
//! As a general rule of thumb, use `Default::default()` as argument to emulate the default behaviour and
//! abort the process after cleaning temporary files. Read more about options in [SignalHandlerMode].
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
#![deny(missing_docs, rust_2018_idioms, unsafe_code)]

use std::{
    io,
    marker::PhantomData,
    path::{Path, PathBuf},
    sync::atomic::AtomicUsize,
};

use dashmap::DashMap;
use once_cell::sync::Lazy;

mod fs;
pub use fs::{create_dir, remove_dir};

pub mod handler;

mod forksafe;
use forksafe::ForksafeTempfile;

pub mod handle;
use crate::handle::{Closed, Writable};

static SIGNAL_HANDLER_MODE: AtomicUsize = AtomicUsize::new(SignalHandlerMode::None as usize);
static NEXT_MAP_INDEX: AtomicUsize = AtomicUsize::new(0);
static REGISTER: Lazy<DashMap<usize, Option<ForksafeTempfile>>> = Lazy::new(|| {
    let mode = SIGNAL_HANDLER_MODE.load(std::sync::atomic::Ordering::SeqCst);
    if mode != SignalHandlerMode::None as usize {
        for sig in signal_hook::consts::TERM_SIGNALS {
            // SAFETY: handlers are considered unsafe because a lot can go wrong. See `cleanup_tempfiles()` for details on safety.
            #[allow(unsafe_code)]
            unsafe {
                #[cfg(not(windows))]
                {
                    signal_hook_registry::register_sigaction(*sig, handler::cleanup_tempfiles_nix)
                }
                #[cfg(windows)]
                {
                    signal_hook::low_level::register(*sig, handler::cleanup_tempfiles_windows)
                }
            }
            .expect("signals can always be installed");
        }
    }
    DashMap::new()
});

/// Define how our signal handlers act
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum SignalHandlerMode {
    /// Do not install a signal handler at all, but have somebody else call our handler directly.
    None = 0,
    /// Delete all remaining registered tempfiles on termination.
    DeleteTempfilesOnTermination = 1,
    /// Delete all remaining registered tempfiles on termination and emulate the default handler behaviour.
    ///
    /// This typically leads to the process being aborted.
    DeleteTempfilesOnTerminationAndRestoreDefaultBehaviour = 2,
}

impl Default for SignalHandlerMode {
    /// By default we will emulate the default behaviour and abort the process.
    ///
    /// While testing, we will not abort the process.
    fn default() -> Self {
        if cfg!(test) {
            SignalHandlerMode::DeleteTempfilesOnTermination
        } else {
            SignalHandlerMode::DeleteTempfilesOnTerminationAndRestoreDefaultBehaviour
        }
    }
}

/// A type expressing the ways we can deal with directories containing a tempfile.
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum ContainingDirectory {
    /// Assume the directory for the tempfile exists and cause failure if it doesn't
    Exists,
    /// Create the directory recursively with the given amount of retries in a way that is somewhat race resistant
    /// depending on the amount of retries.
    CreateAllRaceProof(create_dir::Retries),
}

/// A type expressing the ways we cleanup after ourselves to remove resources we created.
/// Note that cleanup has no effect if the tempfile is persisted.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum AutoRemove {
    /// Remove the temporary file after usage if it wasn't persisted.
    Tempfile,
    /// Remove the temporary file as well the containing directories if they are empty until the given `directory`.
    TempfileAndEmptyParentDirectoriesUntil {
        /// The directory which shall not be removed even if it is empty.
        boundary_directory: PathBuf,
    },
}

impl AutoRemove {
    fn execute_best_effort(self, directory_to_potentially_delete: &Path) -> Option<PathBuf> {
        match self {
            AutoRemove::Tempfile => None,
            AutoRemove::TempfileAndEmptyParentDirectoriesUntil { boundary_directory } => {
                remove_dir::empty_upward_until_boundary(directory_to_potentially_delete, &boundary_directory).ok();
                Some(boundary_directory)
            }
        }
    }
}

/// A registered temporary file which will delete itself on drop or if the program is receiving signals that
/// should cause it to terminate.
///
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
/// This kind of raciness exists whenever [`take()`][Handle::take()] is used and can't be circumvented.
#[derive(Debug)]
#[must_use = "A handle that is immediately dropped doesn't lock a resource meaningfully"]
pub struct Handle<Marker: std::fmt::Debug> {
    id: usize,
    _marker: PhantomData<Marker>,
}

/// A shortcut to [`Handle::<Writable>::new()`], creating a writable temporary file with non-clashing name in a directory.
pub fn new(
    containing_directory: impl AsRef<Path>,
    directory: ContainingDirectory,
    cleanup: AutoRemove,
) -> io::Result<Handle<Writable>> {
    Handle::<Writable>::new(containing_directory, directory, cleanup)
}

/// A shortcut to [`Handle::<Writable>::at()`] providing a writable temporary file at the given path.
pub fn writable_at(
    path: impl AsRef<Path>,
    directory: ContainingDirectory,
    cleanup: AutoRemove,
) -> io::Result<Handle<Writable>> {
    Handle::<Writable>::at(path, directory, cleanup)
}

/// A shortcut to [`Handle::<Closed>::at()`] providing a closed temporary file to mark the presence of something.
pub fn mark_at(
    path: impl AsRef<Path>,
    directory: ContainingDirectory,
    cleanup: AutoRemove,
) -> io::Result<Handle<Closed>> {
    Handle::<Closed>::at(path, directory, cleanup)
}

/// Initialize signal handlers and other state to keep track of tempfiles, and **must be called before the first tempfile is created**,
/// allowing to set the `mode` in which signal handlers are installed.
///
/// Only has an effect the first time it is called.
///
/// Note that it is possible to not call this function and instead call [handler::cleanup_tempfiles()][crate::handler::cleanup_tempfiles()]
/// from a handler under the applications control.
pub fn setup(mode: SignalHandlerMode) {
    SIGNAL_HANDLER_MODE.store(mode as usize, std::sync::atomic::Ordering::SeqCst);
    Lazy::force(&REGISTER);
}

/// DO NOT USE - use [`setup()`] instead.
///
/// Indeed this is merely the old name of `setup()`, which is now a required part of configuring gix-tempfile.
#[deprecated(
    since = "2.0.0",
    note = "call setup(…) instead, this function will be removed in the next major release"
)]
#[doc(hidden)]
pub fn force_setup(mode: SignalHandlerMode) {
    setup(mode)
}
