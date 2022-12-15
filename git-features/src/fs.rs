//! Filesystem utilities
//!
//! These are will be parallel if the `parallel` feature is enabled, at the expense of compiling additional dependencies
//! along with runtime costs for maintaining a global [`rayon`](https://docs.rs/rayon) thread pool.
//!
//! For information on how to use the [`WalkDir`] type, have a look at
//! * [`jwalk::WalkDir`](https://docs.rs/jwalk/0.5.1/jwalk/type.WalkDir.html) if `parallel` feature is enabled
//! * [walkdir::WalkDir](https://docs.rs/walkdir/2.3.1/walkdir/struct.WalkDir.html) otherwise

#[cfg(any(feature = "walkdir", feature = "fs-walkdir-parallel"))]
mod shared {
    /// The desired level of parallelism.
    pub enum Parallelism {
        /// Do not parallelize at all by making a serial traversal on the current thread.
        Serial,
        /// Create a new thread pool for each traversal with up to 16 threads or the amount of logical cores of the machine.
        ThreadPoolPerTraversal {
            /// The base name of the threads we create as part of the thread-pool.
            thread_name: &'static str,
        },
    }
}

///
#[cfg(feature = "fs-walkdir-parallel")]
pub mod walkdir {
    pub use super::shared::Parallelism;
    pub use jwalk::{DirEntry as DirEntryGeneric, DirEntryIter as DirEntryIterGeneric, Error, WalkDir};
    use std::path::Path;

    /// An alias for an uncustomized directory entry to match the one of the non-parallel version offered by `walkdir`.
    pub type DirEntry = DirEntryGeneric<((), ())>;

    impl From<Parallelism> for jwalk::Parallelism {
        fn from(v: Parallelism) -> Self {
            match v {
                Parallelism::Serial => jwalk::Parallelism::Serial,
                Parallelism::ThreadPoolPerTraversal { thread_name } => {
                    let pool = jwalk::rayon::ThreadPoolBuilder::new()
                        .num_threads(num_cpus::get().min(16))
                        .stack_size(128 * 1024)
                        .thread_name(move |idx| format!("{thread_name} {idx}"))
                        .build()
                        .expect("we only set options that can't cause a build failure");
                    jwalk::Parallelism::RayonExistingPool {
                        pool: pool.into(),
                        busy_timeout: None,
                    }
                }
            }
        }
    }

    /// Instantiate a new directory iterator which will not skip hidden files, with the given level of `parallelism`.
    pub fn walkdir_new(root: impl AsRef<Path>, parallelism: Parallelism) -> WalkDir {
        WalkDir::new(root).skip_hidden(false).parallelism(parallelism.into())
    }

    /// Instantiate a new directory iterator which will not skip hidden files and is sorted
    pub fn walkdir_sorted_new(root: impl AsRef<Path>, parallelism: Parallelism) -> WalkDir {
        WalkDir::new(root)
            .skip_hidden(false)
            .sort(true)
            .parallelism(parallelism.into())
    }

    /// The Iterator yielding directory items
    pub type DirEntryIter = DirEntryIterGeneric<((), ())>;
}

#[cfg(all(feature = "walkdir", not(feature = "fs-walkdir-parallel")))]
///
pub mod walkdir {
    pub use super::shared::Parallelism;
    use std::path::Path;

    pub use walkdir::{DirEntry, Error, WalkDir};

    /// Instantiate a new directory iterator which will not skip hidden files, with the given level of `parallelism`.
    pub fn walkdir_new(root: impl AsRef<Path>, _: Parallelism) -> WalkDir {
        WalkDir::new(root)
    }

    /// Instantiate a new directory iterator which will not skip hidden files and is sorted, with the given level of `parallelism`.
    pub fn walkdir_sorted_new(root: impl AsRef<Path>, _: Parallelism) -> WalkDir {
        WalkDir::new(root).sort_by_file_name()
    }

    /// The Iterator yielding directory items
    pub type DirEntryIter = walkdir::IntoIter;
}

#[cfg(any(feature = "walkdir", feature = "fs-walkdir-parallel"))]
pub use self::walkdir::{walkdir_new, walkdir_sorted_new, WalkDir};

/// Prepare open options which won't follow symlinks when the file is opened.
///
/// Note: only effective on unix currently.
pub fn open_options_no_follow() -> std::fs::OpenOptions {
    #[cfg_attr(not(unix), allow(unused_mut))]
    let mut options = std::fs::OpenOptions::new();
    #[cfg(unix)]
    {
        /// Make sure that it's impossible to follow through to the target of symlinks.
        /// Note that this will still follow symlinks in the path, which is what we assume
        /// has been checked separately.
        use std::os::unix::fs::OpenOptionsExt;
        options.custom_flags(libc::O_NOFOLLOW);
    }
    options
}

mod snapshot {
    use std::ops::Deref;

    use crate::threading::{get_mut, get_ref, MutableOnDemand, OwnShared};

    /// A structure holding enough information to reload a value if its on-disk representation changes as determined by its modified time.
    #[derive(Debug)]
    pub struct Snapshot<T: std::fmt::Debug> {
        value: T,
        modified: std::time::SystemTime,
    }

    impl<T: Clone + std::fmt::Debug> Clone for Snapshot<T> {
        fn clone(&self) -> Self {
            Self {
                value: self.value.clone(),
                modified: self.modified,
            }
        }
    }

    /// A snapshot of a resource which is up-to-date in the moment it is retrieved.
    pub type SharedSnapshot<T> = OwnShared<Snapshot<T>>;

    /// Use this type for fields in structs that are to store the [`Snapshot`], typically behind an [`OwnShared`].
    ///
    /// Note that the resource itself is behind another [`OwnShared`] to allow it to be used without holding any kind of lock, hence
    /// without blocking updates while it is used.
    #[derive(Debug, Default)]
    pub struct MutableSnapshot<T: std::fmt::Debug>(pub MutableOnDemand<Option<SharedSnapshot<T>>>);

    impl<T: std::fmt::Debug> Deref for Snapshot<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.value
        }
    }

    impl<T: std::fmt::Debug> Deref for MutableSnapshot<T> {
        type Target = MutableOnDemand<Option<SharedSnapshot<T>>>;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl<T: std::fmt::Debug> MutableSnapshot<T> {
        /// Create a new instance of this type.
        ///
        /// Useful in case `Default::default()` isn't working for some reason.
        pub fn new() -> Self {
            MutableSnapshot(MutableOnDemand::new(None))
        }

        /// Refresh `state` forcefully by re-`open`ing the resource. Note that `open()` returns `None` if the resource isn't
        /// present on disk, and that it's critical that the modified time is obtained _before_ opening the resource.
        pub fn force_refresh<E>(
            &self,
            open: impl FnOnce() -> Result<Option<(std::time::SystemTime, T)>, E>,
        ) -> Result<(), E> {
            let mut state = get_mut(&self.0);
            *state = open()?.map(|(modified, value)| OwnShared::new(Snapshot { value, modified }));
            Ok(())
        }

        /// Assure that the resource in `state` is up-to-date by comparing the `current_modification_time` with the one we know in `state`
        /// and by acting accordingly.
        /// Returns the potentially updated/reloaded resource if it is still present on disk, which then represents a snapshot that is up-to-date
        /// in that very moment, or `None` if the underlying file doesn't exist.
        ///
        /// Note that even though this is racy, each time a request is made there is a chance to see the actual state.
        pub fn recent_snapshot<E>(
            &self,
            mut current_modification_time: impl FnMut() -> Option<std::time::SystemTime>,
            open: impl FnOnce() -> Result<Option<T>, E>,
        ) -> Result<Option<SharedSnapshot<T>>, E> {
            let state = get_ref(self);
            let recent_modification = current_modification_time();
            let buffer = match (&*state, recent_modification) {
                (None, None) => (*state).clone(),
                (Some(_), None) => {
                    drop(state);
                    let mut state = get_mut(self);
                    *state = None;
                    (*state).clone()
                }
                (Some(snapshot), Some(modified_time)) => {
                    if snapshot.modified < modified_time {
                        drop(state);
                        let mut state = get_mut(self);

                        if let (Some(_snapshot), Some(modified_time)) = (&*state, current_modification_time()) {
                            *state = open()?.map(|value| {
                                OwnShared::new(Snapshot {
                                    value,
                                    modified: modified_time,
                                })
                            });
                        }

                        (*state).clone()
                    } else {
                        // Note that this relies on sub-section precision or else is a race when the packed file was just changed.
                        // It's nothing we can know though, so… up to the caller unfortunately.
                        Some(snapshot.clone())
                    }
                }
                (None, Some(_modified_time)) => {
                    drop(state);
                    let mut state = get_mut(self);
                    // Still in the same situation? If so, load the buffer. This compensates for the trampling herd
                    // during lazy-loading at the expense of another mtime check.
                    if let (None, Some(modified_time)) = (&*state, current_modification_time()) {
                        *state = open()?.map(|value| {
                            OwnShared::new(Snapshot {
                                value,
                                modified: modified_time,
                            })
                        });
                    }
                    (*state).clone()
                }
            };
            Ok(buffer)
        }
    }
}
pub use snapshot::{MutableSnapshot, SharedSnapshot, Snapshot};
