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
