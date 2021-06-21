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

    pub fn randomize(backoff_ms: usize) -> usize {
        let new_value = (fastrand::usize(750..=1250) * backoff_ms) / 1000;
        if new_value == 0 {
            backoff_ms
        } else {
            new_value
        }
    }

    struct Exponential<Fn> {
        multiplier: usize,
        max_multiplier: usize,
        exponent: usize,
        transform: Fn,
    }
    impl Default for Exponential<fn(usize) -> usize> {
        fn default() -> Self {
            Exponential {
                multiplier: 1,
                max_multiplier: 1000,
                exponent: 1,
                transform: std::convert::identity,
            }
        }
    }

    impl Exponential<fn(usize) -> usize> {
        fn default_with_random() -> Self {
            Exponential {
                multiplier: 1,
                max_multiplier: 1000,
                exponent: 1,
                transform: randomize,
            }
        }
    }

    impl<Transform> Exponential<Transform>
    where
        Transform: Fn(usize) -> usize,
    {
        fn until_no_remaining(&mut self, time: Duration) -> impl Iterator<Item = Duration> + '_ {
            let mut elapsed = Duration::default();
            let mut stop_next_iteration = false;
            self.take_while(move |d| {
                if stop_next_iteration {
                    false
                } else {
                    elapsed += *d;
                    if elapsed > time {
                        stop_next_iteration = true;
                    }
                    true
                }
            })
        }
    }

    impl<Transform> Iterator for Exponential<Transform>
    where
        Transform: Fn(usize) -> usize,
    {
        type Item = Duration;

        fn next(&mut self) -> Option<Self::Item> {
            let wait = Duration::from_millis((self.transform)(self.multiplier) as u64);

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
        use std::convert::TryInto;

        const EXPECTED_TILL_SECOND: &[usize] = &[
            1usize, 4, 9, 16, 25, 36, 49, 64, 81, 100, 121, 144, 169, 196, 225, 256, 289, 324, 361, 400, 441, 484, 529,
            576, 625, 676, 729, 784, 841, 900, 961, 1000, 1000,
        ];

        #[test]
        fn random_exponential_produces_values_in_the_correct_range() {
            let mut num_identities = 0;
            for (actual, expected) in Exponential::default_with_random().zip(EXPECTED_TILL_SECOND) {
                let actual: usize = actual.as_millis().try_into().unwrap();
                if actual == *expected {
                    num_identities += 1;
                }
                assert!(
                    actual * 1000 >= expected * 750,
                    "value too small: {} < {}",
                    actual,
                    expected
                );
                assert!(
                    actual * 1000 <= expected * 1250,
                    "value too big: {} > {}",
                    actual,
                    expected
                );
            }
            assert!(
                num_identities < EXPECTED_TILL_SECOND.len(),
                "too many untransformed values: {}",
                num_identities
            );
        }

        #[test]
        fn how_many_iterations_for_a_second_of_waittime() {
            let max = Duration::from_millis(1000);
            assert_eq!(Exponential::default().until_no_remaining(max).count(), 14);
            assert_eq!(
                Exponential::default()
                    .until_no_remaining(max)
                    .reduce(|acc, n| acc + n)
                    .unwrap(),
                Duration::from_millis(1015),
                "a little overshoot"
            );
        }

        #[test]
        fn output_with_default_settings() {
            assert_eq!(
                Exponential::default().take(33).collect::<Vec<_>>(),
                EXPECTED_TILL_SECOND
                    .iter()
                    .map(|n| Duration::from_millis(*n as u64))
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
