use bstr::BString;
use quick_error::quick_error;
use std::path::PathBuf;

/// A cache for directory creation to reduce the amount of stat calls when creating
/// directories safely, that is without following symlinks that might be on the way.
///
/// As a special case, it offers a 'prefix' which (by itself) is assumed to exist and may contain symlinks.
/// Everything past that prefix boundary must not contain a symlink.
///
/// For this to work, it remembers the last 'good' path to a directory and assumes that all components of it
/// are still valid, too.
/// As directories are created, the cache will be adjusted to reflect the latest seen directory.
///
/// The caching is only useful if consecutive calls to create a directory are using a sorted list of entries.
#[allow(unused)]
pub(crate) struct DirCache {
    /// the most recent known cached that we know is valid.
    valid: PathBuf,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Collision {
    /// the path that collided with something already present on disk.
    pub path: BString,
    /// The io error we encountered when checking out `path`.
    pub error_kind: std::io::ErrorKind,
}

pub struct Outcome {
    pub collisions: Vec<Collision>,
}

#[derive(Clone, Copy)]
pub struct Options {
    /// capabilities of the file system
    pub fs: crate::fs::Capabilities,
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
    /// The operation will never fail, but count the encountered errors instead along with their paths.
    pub keep_going: bool,
    /// If true, a files creation time is taken into consideration when checking if a file changed.
    /// Can be set to false in case other tools alter the creation time in ways that interfere with our operation.
    ///
    /// Default true.
    pub trust_ctime: bool,
    /// If true, all stat fields will be used when checking for up-to-date'ness of the entry. Otherwise
    /// nano-second parts of mtime and ctime,uid, gid, inode and device number won't be used, leaving only
    /// the whole-second part of ctime and mtime and the file size to be checked.
    ///
    /// Default true.
    pub check_stat: bool,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            fs: Default::default(),
            destination_is_initially_empty: false,
            keep_going: false,
            trust_ctime: true,
            check_stat: true,
            overwrite_existing: false,
        }
    }
}

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        IllformedUtf8{ path: BString } {
            display("Could not convert path to UTF8: {}", path)
        }
        Time(err: std::time::SystemTimeError) {
            from()
            source(err)
            display("The clock was off when reading file related metadata after updating a file on disk")
        }
        Io(err: std::io::Error) {
            from()
            source(err)
            display("IO error while writing blob or reading file metadata or changing filetype")
        }
        ObjectNotFound{ oid: git_hash::ObjectId, path: std::path::PathBuf } {
            display("object {} for checkout at {} not found in object database", oid.to_hex(), path.display())
        }
    }
}
