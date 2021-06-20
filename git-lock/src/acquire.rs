//!
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
    /// The error returned when acquiring a [`File`] or [`Marker`].
    #[derive(Debug)]
    #[allow(missing_docs)]
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
            _inner: lock_with_mode(at_path.as_ref(), mode, boundary_directory, |p, d, c| {
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
    let (directory, cleanup) = dir_cleanup(boundary_directory);
    let lock_path = add_lock_suffix(resource);
    match mode {
        Fail::Immediately => try_lock(&lock_path, directory, cleanup).map_err(Error::from),
        Fail::AfterDurationWithBackoff(_duration) => todo!("fail after timeout"),
    }
}

mod backoff {
    use std::time::Duration;

    struct Exponential {
        multiplier: usize,
        max_multiplier: usize,
        exponent: usize,
    }
    impl Default for Exponential {
        fn default() -> Self {
            Exponential {
                multiplier: 1,
                max_multiplier: 1000,
                exponent: 1,
            }
        }
    }

    impl Iterator for Exponential {
        type Item = Duration;

        fn next(&mut self) -> Option<Self::Item> {
            let wait = Duration::from_millis(self.multiplier as u64);

            self.multiplier += 2 * self.exponent + 1;
            if self.multiplier > self.max_multiplier {
                self.multiplier = self.max_multiplier;
            } else {
                self.exponent += 1;
            }
            Some(wait)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn how_many_iterations_for_a_second_of_waittime() {
            let mut remaining = Duration::from_millis(1000);
            assert_eq!(
                Exponential::default()
                    .take_while(|d| {
                        remaining = remaining.saturating_sub(*d);
                        !remaining.is_zero()
                    })
                    .count(),
                13
            );
        }

        #[test]
        fn output_with_default_settings() {
            assert_eq!(
                Exponential::default().take(33).collect::<Vec<_>>(),
                vec![
                    1, 4, 9, 16, 25, 36, 49, 64, 81, 100, 121, 144, 169, 196, 225, 256, 289, 324, 361, 400, 441, 484,
                    529, 576, 625, 676, 729, 784, 841, 900, 961, 1000, 1000
                ]
                .into_iter()
                .map(Duration::from_millis)
                .collect::<Vec<_>>()
            );
        }
    }
}

fn add_lock_suffix(resource_path: &Path) -> PathBuf {
    resource_path.with_extension(resource_path.extension().map_or_else(
        || DOT_SUFFIX.to_string(),
        |ext| format!("{}{}", ext.to_string_lossy(), DOT_SUFFIX),
    ))
}
