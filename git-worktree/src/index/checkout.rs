#![allow(missing_docs)]
use bstr::BString;
use gix_attributes::Attributes;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Collision {
    /// the path that collided with something already present on disk.
    pub path: BString,
    /// The io error we encountered when checking out `path`.
    pub error_kind: std::io::ErrorKind,
}

pub struct ErrorRecord {
    /// the path that encountered the error.
    pub path: BString,
    /// The error
    pub error: Box<dyn std::error::Error + Send + Sync + 'static>,
}

#[derive(Default)]
pub struct Outcome {
    /// The amount of files updated, or created.
    pub files_updated: usize,
    /// The amount of bytes written to disk,
    pub bytes_written: u64,
    pub collisions: Vec<Collision>,
    pub errors: Vec<ErrorRecord>,
}

#[derive(Clone)]
pub struct Options {
    /// capabilities of the file system
    pub fs: crate::fs::Capabilities,
    /// If set, don't use more than this amount of threads.
    /// Otherwise, usually use as many threads as there are logical cores.
    /// A value of 0 is interpreted as no-limit
    pub thread_limit: Option<usize>,
    /// If true, we assume no file to exist in the target directory, and want exclusive access to it.
    /// This should be enabled when cloning to avoid checks for freshness of files. This also enables
    /// detection of collisions based on whether or not exclusive file creation succeeds or fails.
    pub destination_is_initially_empty: bool,
    /// If true, default false, worktree entries on disk will be overwritten with content from the index
    /// even if they appear to be changed. When creating directories that clash with existing worktree entries,
    /// these will try to delete the existing entry.
    /// This is similar in behaviour as `git checkout --force`.
    pub overwrite_existing: bool,
    /// If true, default false, try to checkout as much as possible and don't abort on first error which isn't
    /// due to a conflict.
    /// The checkout operation will never fail, but count the encountered errors instead along with their paths.
    pub keep_going: bool,
    /// If true, a files creation time is taken into consideration when checking if a file changed.
    /// Can be set to false in case other tools alter the creation time in ways that interfere with our operation.
    ///
    /// Default true.
    pub trust_ctime: bool,
    /// If true, all stat fields will be used when checking for up-to-date'ness of the entry. Otherwise
    /// nano-second parts of mtime and ctime,uid, gid, inode and device number _will not_ be used, leaving only
    /// the whole-second part of ctime and mtime and the file size to be checked.
    ///
    /// Default true.
    pub check_stat: bool,
    /// A group of attribute patterns that are applied globally, i.e. aren't rooted within the repository itself.
    pub attribute_globals: gix_attributes::MatchGroup<Attributes>,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            fs: Default::default(),
            thread_limit: None,
            destination_is_initially_empty: false,
            keep_going: false,
            trust_ctime: true,
            check_stat: true,
            overwrite_existing: false,
            attribute_globals: Default::default(),
        }
    }
}
#[derive(Debug, thiserror::Error)]
pub enum Error<E: std::error::Error + Send + Sync + 'static> {
    #[error("Could not convert path to UTF8: {}", .path)]
    IllformedUtf8 { path: BString },
    #[error("The clock was off when reading file related metadata after updating a file on disk")]
    Time(#[from] std::time::SystemTimeError),
    #[error("IO error while writing blob or reading file metadata or changing filetype")]
    Io(#[from] std::io::Error),
    #[error("object {} for checkout at {} could not be retrieved from object database", .oid.to_hex(), .path.display())]
    Find {
        #[source]
        err: E,
        oid: gix_hash::ObjectId,
        path: std::path::PathBuf,
    },
}
