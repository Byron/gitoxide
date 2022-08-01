//! Filesystem utilities
//!
//! These are will be parallel if the `parallel` feature is enabled, at the expense of compiling additional dependencies
//! along with runtime costs for maintaining a global [`rayon`](https://docs.rs/rayon) thread pool.
//!
//! For information on how to use the [`WalkDir`] type, have a look at
//! * [`jwalk::WalkDir`](https://docs.rs/jwalk/0.5.1/jwalk/type.WalkDir.html) if `parallel` feature is enabled
//! * [walkdir::WalkDir](https://docs.rs/walkdir/2.3.1/walkdir/struct.WalkDir.html) otherwise
#[cfg(feature = "parallel")]
///
pub mod walkdir {
    use std::path::Path;

    pub use jwalk::{DirEntry as DirEntryGeneric, DirEntryIter as DirEntryIterGeneric, Error, WalkDir};

    /// An alias for an uncustomized directory entry to match the one of the non-parallel version offered by `walkdir`.
    pub type DirEntry = DirEntryGeneric<((), ())>;

    /// Instantiate a new directory iterator which will not skip hidden files.
    pub fn walkdir_new(root: impl AsRef<Path>) -> WalkDir {
        WalkDir::new(root).skip_hidden(false)
    }

    /// Instantiate a new directory iterator which will not skip hidden files and is sorted
    pub fn walkdir_sorted_new(root: impl AsRef<Path>) -> WalkDir {
        WalkDir::new(root).sort(true)
    }

    /// The Iterator yielding directory items
    pub type DirEntryIter = DirEntryIterGeneric<((), ())>;
}

#[cfg(all(feature = "walkdir", not(feature = "parallel")))]
///
pub mod walkdir {
    use std::path::Path;

    pub use walkdir::{DirEntry, Error, WalkDir};

    /// Instantiate a new directory iterator which will not skip hidden files.
    pub fn walkdir_new(root: impl AsRef<Path>) -> WalkDir {
        WalkDir::new(root)
    }

    /// Instantiate a new directory iterator which will not skip hidden files and is sorted
    pub fn walkdir_sorted_new(root: impl AsRef<Path>) -> WalkDir {
        WalkDir::new(root).sort_by_file_name()
    }

    /// The Iterator yielding directory items
    pub type DirEntryIter = walkdir::IntoIter;
}

#[cfg(any(feature = "walkdir", feature = "jwalk"))]
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
    use crate::threading::{get_mut, get_ref, MutableOnDemand, OwnShared};
    use std::ops::Deref;

    /// A snapshot of a resource which is up-to-date in the moment it is retrieved.
    pub type SharedSnapshot<T> = OwnShared<Snapshot<T>>;

    /// Use this type for fields in structs that are to store the [`Snapshot`], typically behind an [`OwnShared`].
    ///
    /// Note that the resource itself is behind another [`OwnShared`] to allow it to be used without holding any kind of lock, hence
    /// without blocking updates while it is used.
    pub type MutableSnapshot<T> = MutableOnDemand<Option<SharedSnapshot<T>>>;

    /// A structure holding enough information to reload a value if its on-disk representation changes as determined by its modified time.
    #[derive(Debug)]
    pub struct Snapshot<T: std::fmt::Debug> {
        value: T,
        modified: std::time::SystemTime,
    }

    impl<T: std::fmt::Debug> Deref for Snapshot<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.value
        }
    }

    impl<T: std::fmt::Debug> Snapshot<T> {
        /// Refresh `state` forcefully by re-`open`ing the resource. Note that `open()` returns `None` if the resource isn't
        /// present on disk, and that it's critical that the modified time is obtained _before_ opening the resource.
        pub fn force_refresh<E>(
            state: &MutableSnapshot<T>,
            open: impl FnOnce() -> Result<Option<(std::time::SystemTime, T)>, E>,
        ) -> Result<(), E> {
            let mut state = get_mut(state);
            *state = open()?.map(|(modified, value)| OwnShared::new(Snapshot { value, modified }));
            Ok(())
        }

        /// Assure that the resource in `state` is up-to-date by comparing the `current_modification_time` with the one we know in `state`
        /// and by acting accordingly.
        /// Returns the potentially updated/reloaded resource if it is still present on disk, which then represents a snapshot that is up-to-date
        /// in that very moment.
        ///
        /// Note that it is race-proof.
        pub fn recent_snapshot<E>(
            state: &MutableSnapshot<T>,
            mut current_modification_time: impl FnMut() -> Option<std::time::SystemTime>,
            open: impl FnOnce() -> Result<Option<T>, E>,
        ) -> Result<Option<SharedSnapshot<T>>, E> {
            let state_opt_lock = get_ref(state);
            let recent_modification = current_modification_time();
            let buffer = match (&*state_opt_lock, recent_modification) {
                (None, None) => (*state_opt_lock).clone(),
                (Some(_), None) => {
                    drop(state_opt_lock);
                    let mut state = get_mut(state);
                    // Still in the same situation? If so, drop the loaded buffer
                    if let (Some(_), None) = (&*state, current_modification_time()) {
                        *state = None;
                    }
                    (*state).clone()
                }
                (Some(state_shared), Some(modified_time)) => {
                    if state_shared.modified < modified_time {
                        drop(state_opt_lock);
                        let mut state = get_mut(state);

                        // in the common case, we check again and do what we do only if we are
                        // still in the same situation, writers pile up.
                        match (&mut *state, current_modification_time()) {
                            (Some(state_opt), Some(modified_time)) if state_opt.modified < modified_time => {
                                match open()? {
                                    Some(value) => {
                                        *state_opt = OwnShared::new(Snapshot {
                                            value,
                                            modified: modified_time,
                                        });
                                    }
                                    None => {
                                        *state = None;
                                    }
                                }
                            }
                            _ => {}
                        }
                        (*state).clone()
                    } else {
                        // Note that this relies on sub-section precision or else is a race when the packed file was just changed.
                        // It's nothing we can know though, so… up to the caller unfortunately.
                        Some(state_shared.clone())
                    }
                }
                (None, Some(_modified_time)) => {
                    drop(state_opt_lock);
                    let mut state = get_mut(state);
                    // Still in the same situation? If so, load the buffer.
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
