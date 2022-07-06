/// The error returned by [`realpath()`][super::realpath()].
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error("The maximum allowed number {} of symlinks in path is exceeded", .max_symlinks)]
    MaxSymlinksExceeded { max_symlinks: u8 },
    #[error(transparent)]
    ReadLink(std::io::Error),
    #[error(transparent)]
    CurrentWorkingDir(std::io::Error),
    #[error("Empty is not a valid path")]
    EmptyPath,
    #[error("Ran out of path components while following parent component '..'")]
    MissingParent,
}

/// The default amount of symlinks we may follow when resolving a path in [`realpath`].
pub const MAX_SYMLINKS: u8 = 32;

pub(crate) mod function {
    use crate::realpath::MAX_SYMLINKS;
    use std::path::{
        Component::{CurDir, Normal, ParentDir, Prefix, RootDir},
        Path, PathBuf,
    };

    use super::Error;

    /// Check each component of `path` and see if it is a symlink. If so, resolve it.
    /// Do not fail for non-existing components, but assume these are as is.
    ///
    /// If `path` is relative, the current working directory be used to make it absolute.
    pub fn realpath(path: impl AsRef<Path>) -> Result<PathBuf, Error> {
        let cwd = path
            .as_ref()
            .is_relative()
            .then(std::env::current_dir)
            .unwrap_or_else(|| Ok(PathBuf::default()))
            .map_err(Error::CurrentWorkingDir)?;
        realpath_opts(path, cwd, MAX_SYMLINKS)
    }

    /// The same as [`realpath()`], but allow to configure `max_symlinks` to configure how many symbolic links we are going to follow.
    /// This serves to avoid running into cycles or doing unreasonable amounts of work.
    pub fn realpath_opts(path: impl AsRef<Path>, cwd: impl AsRef<Path>, max_symlinks: u8) -> Result<PathBuf, Error> {
        let path = path.as_ref();
        if path.as_os_str().is_empty() {
            return Err(Error::EmptyPath);
        }

        let mut real_path = PathBuf::new();
        if path.is_relative() {
            real_path.push(cwd);
        }

        let mut num_symlinks = 0;
        let mut path_backing: PathBuf;
        let mut components = path.components();
        while let Some(component) = components.next() {
            match component {
                part @ RootDir | part @ Prefix(_) => real_path.push(part),
                CurDir => {}
                ParentDir => {
                    if !real_path.pop() {
                        return Err(Error::MissingParent);
                    }
                }
                Normal(part) => {
                    real_path.push(part);
                    if real_path.is_symlink() {
                        num_symlinks += 1;
                        if num_symlinks > max_symlinks {
                            return Err(Error::MaxSymlinksExceeded { max_symlinks });
                        }
                        let mut link_destination = std::fs::read_link(real_path.as_path()).map_err(Error::ReadLink)?;
                        if link_destination.is_absolute() {
                            // pushing absolute path to real_path resets it to the pushed absolute path
                        } else {
                            assert!(real_path.pop(), "we just pushed a component");
                        }
                        link_destination.extend(components);
                        path_backing = link_destination;
                        components = path_backing.components();
                    }
                }
            }
        }
        Ok(real_path)
    }
}
