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
    pub use jwalk::{DirEntry as DirEntryGeneric, Error, WalkDir};
    use std::path::PathBuf;

    /// An alias for an uncustomized directory entry to match the one of the non-parallel version offered by `walkdir`.
    pub type DirEntry = DirEntryGeneric<((), ())>;

    /// Enable sorting by filename
    pub fn sorted(w: WalkDir) -> WalkDir {
        w.sort(true)
    }

    /// Obtain the owned, full path the `entry` points to
    pub fn direntry_path(entry: &DirEntry) -> PathBuf {
        entry.path()
    }
}

#[cfg(not(feature = "parallel"))]
///
pub mod walkdir {
    use std::path::PathBuf;
    pub use walkdir::{DirEntry, Error, WalkDir};

    /// Enable sorting by filename
    pub fn sorted(w: WalkDir) -> WalkDir {
        w.sort_by(|lhs, rhs| lhs.file_name().cmp(rhs.file_name()))
    }

    /// Obtain the owned, full path the `entry` points to
    pub fn direntry_path(entry: &DirEntry) -> PathBuf {
        entry.path().to_owned()
    }
}

pub use self::walkdir::{direntry_path, sorted, DirEntry, WalkDir};
