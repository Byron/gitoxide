use bstr::BString;
use std::path::PathBuf;

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
    /// The prefix/root for all paths we handle.
    root: PathBuf,
    /// the most recent known cached that we know is valid.
    valid: PathBuf,
    /// The relative portion of `valid` that was added previously.
    valid_relative: PathBuf,
    /// The amount of path components of 'valid' beyond the roots components. If `root` has 2, and this is 2, `valid` has 4 components.
    valid_components: usize,

    /// If there is a symlink or a file in our path, try to unlink it before creating the directory.
    pub unlink_on_collision: bool,

    /// just for testing
    #[cfg(debug_assertions)]
    pub test_mkdir_calls: usize,
}

mod cache {
    use super::PathCache;
    use std::path::{Path, PathBuf};

    impl PathCache {
        /// Create a new instance with `root` being the base for all future paths we handle, assuming it to be valid which includes
        /// symbolic links to be included in it as well.
        pub fn new(root: impl Into<PathBuf>) -> Self {
            let root = root.into();
            PathCache {
                valid: root.clone(),
                valid_relative: PathBuf::with_capacity(128),
                valid_components: 0,
                root,
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
            let relative = relative.as_ref();
            debug_assert!(
                relative.is_relative(),
                "only index paths are handled correctly here, must be relative"
            );

            let mut components = relative.components().peekable();
            let mut existing_components = self.valid_relative.components();
            let mut matching_components = 0;
            while let (Some(existing_comp), Some(new_comp)) = (existing_components.next(), components.peek()) {
                if existing_comp == *new_comp {
                    components.next();
                    matching_components += 1;
                } else {
                    break;
                }
            }

            // TODO: handle valid state properly, handle _mode.
            for _ in 0..self.valid_components - matching_components {
                self.valid.pop();
            }

            self.valid_components = matching_components;

            let target_is_dir = mode == git_index::entry::Mode::COMMIT || mode == git_index::entry::Mode::DIR;
            while let Some(comp) = components.next() {
                self.valid.push(comp);
                self.valid_relative.push(comp);
                self.valid_components += 1;
                if components.peek().is_some() || target_is_dir {
                    #[cfg(debug_assertions)]
                    {
                        self.test_mkdir_calls += 1;
                    }
                    match std::fs::create_dir(&self.valid) {
                        Ok(()) => {}
                        Err(err) if err.kind() == std::io::ErrorKind::AlreadyExists => {
                            let meta = self.valid.symlink_metadata()?;
                            if !meta.is_dir() {
                                if self.unlink_on_collision {
                                    if meta.is_symlink() {
                                        symlink::remove_symlink_auto(&self.valid)?;
                                    } else {
                                        std::fs::remove_file(&self.valid)?;
                                    }
                                    #[cfg(debug_assertions)]
                                    {
                                        self.test_mkdir_calls += 1;
                                    }
                                    std::fs::create_dir(&self.valid)?;
                                    continue;
                                }
                                return Err(err);
                            }
                        }
                        Err(err) => return Err(err),
                    }
                }
            }

            Ok(&self.valid)
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
    pub collisions: Vec<Collision>,
    pub errors: Vec<ErrorRecord>,
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
            destination_is_initially_empty: false,
            keep_going: false,
            trust_ctime: true,
            check_stat: true,
            overwrite_existing: false,
        }
    }
}
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Could not convert path to UTF8: {}", .path)]
    IllformedUtf8 { path: BString },
    #[error("The clock was off when reading file related metadata after updating a file on disk")]
    Time(#[from] std::time::SystemTimeError),
    #[error("IO error while writing blob or reading file metadata or changing filetype")]
    Io(#[from] std::io::Error),
    #[error("object {} for checkout at {} not found in object database", .oid.to_hex(), .path.display())]
    ObjectNotFound {
        oid: git_hash::ObjectId,
        path: std::path::PathBuf,
    },
}
