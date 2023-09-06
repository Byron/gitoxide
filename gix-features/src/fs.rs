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
    use std::path::Path;

    pub use jwalk::{DirEntry as DirEntryGeneric, DirEntryIter as DirEntryIterGeneric, Error, WalkDir};

    pub use super::shared::Parallelism;

    /// An alias for an uncustomized directory entry to match the one of the non-parallel version offered by `walkdir`.
    pub type DirEntry = DirEntryGeneric<((), ())>;

    impl From<Parallelism> for jwalk::Parallelism {
        fn from(v: Parallelism) -> Self {
            match v {
                Parallelism::Serial => jwalk::Parallelism::Serial,
                Parallelism::ThreadPoolPerTraversal { thread_name } => std::thread::available_parallelism()
                    .map_or_else(
                        |_| Parallelism::Serial.into(),
                        |threads| {
                            let pool = jwalk::rayon::ThreadPoolBuilder::new()
                                .num_threads(threads.get().min(16))
                                .stack_size(128 * 1024)
                                .thread_name(move |idx| format!("{thread_name} {idx}"))
                                .build()
                                .expect("we only set options that can't cause a build failure");
                            jwalk::Parallelism::RayonExistingPool {
                                pool: pool.into(),
                                busy_timeout: None,
                            }
                        },
                    ),
            }
        }
    }

    /// Instantiate a new directory iterator which will not skip hidden files, with the given level of `parallelism`.
    pub fn walkdir_new(root: &Path, parallelism: Parallelism) -> WalkDir {
        WalkDir::new(root).skip_hidden(false).parallelism(parallelism.into())
    }

    /// Instantiate a new directory iterator which will not skip hidden files and is sorted
    pub fn walkdir_sorted_new(root: &Path, parallelism: Parallelism) -> WalkDir {
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
    use std::path::Path;

    pub use walkdir::{DirEntry, Error, WalkDir};

    pub use super::shared::Parallelism;

    /// Instantiate a new directory iterator which will not skip hidden files, with the given level of `parallelism`.
    pub fn walkdir_new(root: &Path, _: Parallelism) -> WalkDir {
        WalkDir::new(root)
    }

    /// Instantiate a new directory iterator which will not skip hidden files and is sorted, with the given level of `parallelism`.
    pub fn walkdir_sorted_new(root: &Path, _: Parallelism) -> WalkDir {
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
