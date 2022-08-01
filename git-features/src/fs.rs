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

#[allow(missing_docs)]
mod reload_on_demand {
    use crate::threading::{get_mut, get_ref, MutableOnDemand, OwnShared};
    use std::ops::Deref;

    pub type ReloadIfChangedStorage<T> = MutableOnDemand<Option<OwnShared<ReloadIfChanged<T>>>>;

    #[derive(Debug)]
    pub struct ReloadIfChanged<T: std::fmt::Debug> {
        value: T,
        modified: std::time::SystemTime,
    }

    impl<T: std::fmt::Debug> Deref for ReloadIfChanged<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.value
        }
    }

    impl<T: std::fmt::Debug> ReloadIfChanged<T> {
        pub fn force_refresh<E>(
            state: &ReloadIfChangedStorage<T>,
            open: impl FnOnce() -> Result<Option<(std::time::SystemTime, T)>, E>,
        ) -> Result<(), E> {
            let mut state = get_mut(state);
            *state = open()?.map(|(modified, value)| OwnShared::new(ReloadIfChanged { value, modified }));
            Ok(())
        }

        pub fn assure_uptodate<E>(
            state: &ReloadIfChangedStorage<T>,
            mut current_modification_time: impl FnMut() -> Option<std::time::SystemTime>,
            open: impl FnOnce() -> Result<Option<T>, E>,
        ) -> Result<Option<OwnShared<ReloadIfChanged<T>>>, E> {
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
                                        *state_opt = OwnShared::new(ReloadIfChanged {
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
                    drop(state);
                    let mut state = get_mut(state);
                    // Still in the same situation? If so, load the buffer.
                    if let (None, Some(modified_time)) = (&*state, current_modification_time()) {
                        *state = open()?.map(|value| {
                            OwnShared::new(ReloadIfChanged {
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
pub use reload_on_demand::{ReloadIfChanged, ReloadIfChangedStorage};
