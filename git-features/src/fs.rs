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
    pub use jwalk::{DirEntry as DirEntryGeneric, DirEntryIter as DirEntryIterGeneric, Error, WalkDir};
    use std::path::Path;

    /// An alias for an uncustomized directory entry to match the one of the non-parallel version offered by `walkdir`.
    pub type DirEntry = DirEntryGeneric<((), ())>;

    /// Instantiate a new directory iterator which will not skip hidden files.
    pub fn walkdir_new(root: impl AsRef<Path>) -> WalkDir {
        WalkDir::new(root).skip_hidden(false)
    }

    /// The Iterator yielding directory items
    pub type DirEntryIter = DirEntryIterGeneric<((), ())>;
}

#[cfg(not(feature = "parallel"))]
///
pub mod walkdir {
    use std::path::Path;
    pub use walkdir::{DirEntry, Error, WalkDir};

    /// Instantiate a new directory iterator which will not skip hidden files.
    pub fn walkdir_new(root: impl AsRef<Path>) -> WalkDir {
        WalkDir::new(root)
    }

    /// The Iterator yielding directory items
    pub type DirEntryIter = walkdir::IntoIter;
}

pub use self::walkdir::{walkdir_new, WalkDir};
