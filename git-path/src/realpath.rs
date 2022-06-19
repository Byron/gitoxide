use std::path::PathBuf;

/// The error returned by [`realpath()`][super::realpath()].
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error("The maximum allowed number {} of symlinks in path is exceeded", .max_symlinks)]
    MaxSymlinksExceeded { max_symlinks: u8 },
    #[error(transparent)]
    ReadLink(#[from] std::io::Error),
    #[error("Empty is not a valid path")]
    EmptyPath,
    #[error("Parent component of {:?} does not exist: {}", .path, .msg)]
    MissingParent { path: PathBuf, msg: &'static str },
}

pub(crate) mod function {
    use std::path::{
        Component::{CurDir, Normal, ParentDir, Prefix, RootDir},
        Path, PathBuf,
    };

    use super::Error;

    /// Check each component of `path` and see if it is a symlink. If so, resolve it.
    /// Do not fail for non-existing components, but assume these are as is.
    ///
    /// If `path` is relative, `cwd` will be used to make it absolute (assuming `cwd` is absolute too).
    pub fn realpath(path: impl AsRef<Path>, cwd: impl AsRef<Path>) -> Result<PathBuf, Error> {
        let git_default = 32;
        realpath_opts(path, cwd, git_default)
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
                        return Err(Error::MissingParent {
                            path: real_path,
                            msg: "parent path must exist",
                        });
                    }
                }
                Normal(part) => {
                    real_path.push(part);
                    if real_path.is_symlink() {
                        num_symlinks += 1;
                        if num_symlinks > max_symlinks {
                            return Err(Error::MaxSymlinksExceeded { max_symlinks });
                        }
                        let mut link_destination = std::fs::read_link(real_path.as_path())?;
                        if link_destination.is_absolute() {
                            // pushing absolute path to real_path resets it to the pushed absolute path
                        } else if !real_path.pop() {
                            return Err(Error::MissingParent {
                                path: real_path,
                                msg: "we just pushed a component",
                            });
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
