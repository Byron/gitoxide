//! Find git repositories or search them upwards from a starting point.
#![forbid(unsafe_code, rust_2018_idioms)]
#![deny(missing_docs)]

///
pub mod repository {
    use std::path::PathBuf;
    /// A repository path which either points to a work tree or the `.git` repository itself.
    #[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
    pub enum Path {
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
                    Path::WorkTree(path) | Path::Repository(path) => path,
                }
            }
        }

        impl Path {
            /// Instantiate a new path from `dir` which is expected to be the `.git` directory, with `kind` indicating
            /// whether it's a bare repository or not.
            pub fn from_dot_git_dir(dir: impl Into<PathBuf>, kind: Kind) -> Self {
                let dir = dir.into();
                match kind {
                    Kind::WorkTree => Path::WorkTree(if dir == std::path::Path::new(".git") {
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
                    Path::WorkTree(_) => Kind::WorkTree,
                    Path::Repository(_) => Kind::Bare,
                }
            }

            /// Consume and split this path into the location of the `.git` directory as well as an optional path to the work tree.
            pub fn into_repository_and_work_tree_directories(self) -> (PathBuf, Option<PathBuf>) {
                match self {
                    Path::WorkTree(working_tree) => (working_tree.join(".git"), Some(working_tree)),
                    Path::Repository(repository) => (repository, None),
                }
            }
        }
    }

    /// The kind of path
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
    pub enum Kind {
        /// A bare repository does not have a work tree, that is files on disk beyond the `git` repository itself.
        Bare,
        /// A `git` repository along with a checked out files in a work tree.
        WorkTree,
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
}

mod is;
pub use is::{bare as is_bare, git as is_git};

///
pub mod upwards;
pub use upwards::function::{discover as upwards, discover_opts as upwards_opts};
