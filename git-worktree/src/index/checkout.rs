use crate::fs;
use bstr::BString;

/// A cache for directory creation to reduce the amount of stat calls when creating
/// directories safely, that is without following symlinks that might be on the way.
///
/// As a special case, it offers a 'prefix' which (by itself) is assumed to exist and may contain symlinks.
/// Everything past that prefix boundary must not contain a symlink. We do this by allowing any input path.
///
/// Another added benefit is its ability to store the path of full path of the entry to which leading directories
/// are to be created to avoid allocating memory.
///
/// For this to work, it remembers the last 'good' path to a directory and assumes that all components of it
/// are still valid, too.
/// As directories are created, the cache will be adjusted to reflect the latest seen directory.
///
/// The caching is only useful if consecutive calls to create a directory are using a sorted list of entries.
#[allow(unused)]
pub struct PathCache {
    stack: fs::Stack,
    /// If there is a symlink or a file in our path, try to unlink it before creating the directory.
    pub unlink_on_collision: bool,

    /// just for testing
    #[cfg(debug_assertions)]
    pub test_mkdir_calls: usize,
}

mod cache {
    use std::path::{Path, PathBuf};

    use super::PathCache;
    use crate::{fs, os};

    impl PathCache {
        /// Create a new instance with `root` being the base for all future paths we handle, assuming it to be valid which includes
        /// symbolic links to be included in it as well.
        pub fn new(root: impl Into<PathBuf>) -> Self {
            let root = root.into();
            PathCache {
                stack: fs::Stack::new(root),
                #[cfg(debug_assertions)]
                test_mkdir_calls: 0,
                unlink_on_collision: false,
            }
        }

        /// Append the `relative` path to the root directory the cache contains and efficiently create leading directories
        /// unless `mode` indicates `relative` points to a directory itself in which case the entire resulting path is created as directory.
        ///
        /// The full path to `relative` will be returned for use on the file system.
        pub fn append_relative_path_assure_leading_dir(
            &mut self,
            relative: impl AsRef<Path>,
            mode: git_index::entry::Mode,
        ) -> std::io::Result<&Path> {
            #[cfg(debug_assertions)]
            let mkdir_calls = &mut self.test_mkdir_calls;
            let unlink_on_collision = self.unlink_on_collision;
            self.stack.make_relative_path_current(
                relative,
                |components, stack: &fs::Stack| {
                    let target_is_dir = mode == git_index::entry::Mode::COMMIT || mode == git_index::entry::Mode::DIR;
                    if components.peek().is_some() || target_is_dir {
                        #[cfg(debug_assertions)]
                        {
                            *mkdir_calls += 1;
                        }
                        match std::fs::create_dir(stack.current()) {
                            Ok(()) => {}
                            Err(err) if err.kind() == std::io::ErrorKind::AlreadyExists => {
                                let meta = stack.current().symlink_metadata()?;
                                if !meta.is_dir() {
                                    if unlink_on_collision {
                                        if meta.is_symlink() {
                                            os::remove_symlink(stack.current())?;
                                        } else {
                                            std::fs::remove_file(stack.current())?;
                                        }
                                        #[cfg(debug_assertions)]
                                        {
                                            *mkdir_calls += 1;
                                        }
                                        std::fs::create_dir(stack.current())?;
                                    } else {
                                        return Err(err);
                                    }
                                }
                            }
                            Err(err) => return Err(err),
                        }
                    }
                    Ok(())
                },
                |_| {},
            )?;
            Ok(self.stack.current())
        }
    }
}

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

pub struct Outcome {
    /// The amount of files updated, or created.
    pub files_updated: usize,
    /// The amount of bytes written to disk,
    pub bytes_written: u64,
    pub collisions: Vec<Collision>,
    pub errors: Vec<ErrorRecord>,
}

#[derive(Clone, Copy)]
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
            thread_limit: None,
            destination_is_initially_empty: false,
            keep_going: false,
            trust_ctime: true,
            check_stat: true,
            overwrite_existing: false,
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
        oid: git_hash::ObjectId,
        path: std::path::PathBuf,
    },
}
