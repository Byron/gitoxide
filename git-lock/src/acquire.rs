//!
use crate::{backoff, File, Marker, DOT_SUFFIX};
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
    /// The error returned when acquiring a [`File`] or [`Marker`].
    #[derive(Debug)]
    #[allow(missing_docs)]
    pub enum Error {
        Io(err: std::io::Error) {
            display("Another IO error occurred while obtaining the lock")
            from()
            source(err)
        }
        PermanentlyLocked { resource_path: PathBuf, mode: Fail, attempts: usize } {
            display("The lock for resource '{} could not be obtained {} after {} attempt(s). The lockfile at '{}{}' might need manual deletion.", resource_path.display(), mode, attempts, resource_path.display(), super::DOT_SUFFIX)
        }
    }
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
    /// Like [`acquire_to_update_resource()`][File::acquire_to_update_resource()] but _without_ the possibility to make changes
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
            created_from_file: false,
            inner: lock_with_mode(at_path.as_ref(), mode, boundary_directory, |p, d, c| {
                git_tempfile::mark_at(p, d, c)
            })?,
        })
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
    use std::io::ErrorKind::*;
    let (directory, cleanup) = dir_cleanup(boundary_directory);
    let lock_path = add_lock_suffix(resource);
    let mut attempts = 1;
    match mode {
        Fail::Immediately => try_lock(&lock_path, directory, cleanup),
        Fail::AfterDurationWithBackoff(time) => {
            for wait in backoff::Exponential::default_with_random().until_no_remaining(time) {
                attempts += 1;
                match try_lock(&lock_path, directory, cleanup.clone()) {
                    Ok(v) => return Ok(v),
                    Err(err) if err.kind() == AlreadyExists => {
                        std::thread::sleep(wait);
                        continue;
                    }
                    Err(err) => return Err(Error::from(err)),
                }
            }
            try_lock(&lock_path, directory, cleanup)
        }
    }
    .map_err(|err| match err.kind() {
        AlreadyExists => Error::PermanentlyLocked {
            resource_path: resource.into(),
            mode,
            attempts,
        },
        _ => Error::Io(err),
    })
}

fn add_lock_suffix(resource_path: &Path) -> PathBuf {
    resource_path.with_extension(resource_path.extension().map_or_else(
        || DOT_SUFFIX.to_string(),
        |ext| format!("{}{}", ext.to_string_lossy(), DOT_SUFFIX),
    ))
}
