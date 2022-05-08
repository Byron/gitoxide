//! Find git repositories or search them upwards from a starting point, or determine if a directory looks like a git repository.
//!
//! Note that detection methods are educated guesses using the presence of files, without looking too much into the details.
#![forbid(unsafe_code, rust_2018_idioms)]
#![deny(missing_docs)]

///
pub mod repository {
    use std::path::PathBuf;
    /// A repository path which either points to a work tree or the `.git` repository itself.
    #[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
    pub enum Path {
        /// The currently checked out linked worktree along with its connected and existing git directory.
        LinkedWorkTree {
            /// The base of the work tree.
            work_dir: PathBuf,
            /// The worktree-private git dir, located within the main git directory which holds most of the information.
            git_dir: PathBuf,
        },
        /// The currently checked out or nascent work tree of a git repository
        WorkTree(PathBuf),
        /// The git repository itself
        Repository(PathBuf),
    }

    mod path {
        use crate::repository::{Kind, Path};
        use std::path::PathBuf;

        impl AsRef<std::path::Path> for Path {
            fn as_ref(&self) -> &std::path::Path {
                match self {
                    Path::WorkTree(path)
                    | Path::Repository(path)
                    | Path::LinkedWorkTree {
                        work_dir: _,
                        git_dir: path,
                    } => path,
                }
            }
        }

        impl Path {
            /// Instantiate a new path from `dir` which is expected to be the `.git` directory, with `kind` indicating
            /// whether it's a bare repository or not.
            pub fn from_dot_git_dir(dir: impl Into<PathBuf>, kind: Kind) -> Self {
                let dir = dir.into();
                match kind {
                    Kind::WorkTree { linked_git_dir: _ } => Path::WorkTree(if dir == std::path::Path::new(".git") {
                        PathBuf::from(".")
                    } else {
                        dir.parent().expect("this is a sub-directory").to_owned()
                    }),
                    Kind::Bare => Path::Repository(dir),
                }
            }
            /// Returns the [kind][Kind] of this repository path.
            pub fn kind(&self) -> Kind {
                match self {
                    Path::LinkedWorkTree { work_dir: _, git_dir } => Kind::WorkTree {
                        linked_git_dir: Some(git_dir.to_owned()),
                    },
                    Path::WorkTree(_) => Kind::WorkTree { linked_git_dir: None },
                    Path::Repository(_) => Kind::Bare,
                }
            }

            /// Consume and split this path into the location of the `.git` directory as well as an optional path to the work tree.
            pub fn into_repository_and_work_tree_directories(self) -> (PathBuf, Option<PathBuf>) {
                match self {
                    Path::LinkedWorkTree { work_dir, git_dir } => (git_dir, Some(work_dir)),
                    Path::WorkTree(working_tree) => (working_tree.join(".git"), Some(working_tree)),
                    Path::Repository(repository) => (repository, None),
                }
            }
        }
    }

    /// The kind of repository path.
    #[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
    pub enum Kind {
        /// A bare repository does not have a work tree, that is files on disk beyond the `git` repository itself.
        Bare,
        /// A `git` repository along with a checked out files in a work tree.
        WorkTree {
            /// If set, this is the git dir associated with this _linked_ worktree.
            /// If `None`, the git_dir is the `.git` directory inside the _main_ worktree we represent.
            linked_git_dir: Option<PathBuf>,
        },
    }

    impl Kind {
        /// Returns true if this is a bare repository, one without a work tree.
        pub fn is_bare(&self) -> bool {
            matches!(self, Kind::Bare)
        }
    }
}

///
pub mod is_git {
    use std::path::PathBuf;

    /// The error returned by [`crate::is_git()`].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error("Could not find a valid HEAD reference")]
        FindHeadRef(#[from] git_ref::file::find::existing::Error),
        #[error("Expected HEAD at '.git/HEAD', got '.git/{}'", .name)]
        MisplacedHead { name: bstr::BString },
        #[error("Expected an objects directory at '{}'", .missing.display())]
        MissingObjectsDirectory { missing: PathBuf },
        #[error("Expected a refs directory at '{}'", .missing.display())]
        MissingRefsDirectory { missing: PathBuf },
    }

    /// Options for [`crate::is_git()`].
    pub struct Options {}
}

mod is;
pub use is::{bare as is_bare, git as is_git};

///
pub mod upwards;
pub use upwards::function::{discover as upwards, discover_opts as upwards_opts};

///
pub mod path {
    use std::io::Read;
    use std::path::PathBuf;

    ///
    pub mod from_gitdir_file {
        /// The error returned by [`from_gitdir_file()`][crate::path::from_gitdir_file()].
        #[derive(Debug, thiserror::Error)]
        #[allow(missing_docs)]
        pub enum Error {
            #[error(transparent)]
            Io(#[from] std::io::Error),
            #[error(transparent)]
            Parse(#[from] crate::parse::gitdir::Error),
        }
    }

    fn read_regular_file_content_with_size_limit(path: impl AsRef<std::path::Path>) -> std::io::Result<Vec<u8>> {
        let path = path.as_ref();
        let mut file = std::fs::File::open(path)?;
        let max_file_size = 2048;
        let file_size = file.metadata()?.len();
        if file_size > max_file_size {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "Refusing to open files larger than {} bytes, '{}' was {} bytes large",
                    max_file_size,
                    path.display(),
                    file_size
                ),
            ));
        }
        let mut buf = Vec::with_capacity(512);
        file.read_to_end(&mut buf)?;
        Ok(buf)
    }

    /// Reads a plain path from a file that contains it as its only content, with trailing newlines trimmed.
    pub fn from_plain_file(path: impl AsRef<std::path::Path>) -> Option<std::io::Result<PathBuf>> {
        use bstr::ByteSlice;
        let mut buf = match read_regular_file_content_with_size_limit(path) {
            Ok(buf) => buf,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => return None,
            Err(err) => return Some(Err(err)),
        };
        let trimmed_len = buf.trim_end().len();
        buf.truncate(trimmed_len);
        Some(Ok(git_path::from_bstring(buf)))
    }

    /// Reads typical `gitdir: ` files from disk as used by worktrees and submodules.
    pub fn from_gitdir_file(path: impl AsRef<std::path::Path>) -> Result<PathBuf, from_gitdir_file::Error> {
        let path = path.as_ref();
        let buf = read_regular_file_content_with_size_limit(path)?;
        let mut gitdir = crate::parse::gitdir(&buf)?;
        if let Some(parent) = path.parent() {
            gitdir = parent.join(gitdir);
        }
        Ok(gitdir)
    }
}

///
pub mod parse;
