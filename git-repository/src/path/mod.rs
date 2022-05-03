use std::path::PathBuf;

#[cfg(all(feature = "unstable", feature = "git-path"))]
pub use git_path::*;

use crate::{Kind, Path};

///
pub mod create;
///
pub mod discover;
pub use discover::function::{discover, discover_opts};
///
pub mod is;

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
            crate::Path::WorkTree(working_tree) => (working_tree.join(".git"), Some(working_tree)),
            crate::Path::Repository(repository) => (repository, None),
        }
    }
}

pub(crate) fn install_dir() -> std::io::Result<std::path::PathBuf> {
    std::env::current_exe().and_then(|exe| {
        exe.parent()
            .map(ToOwned::to_owned)
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "no parent for current executable"))
    })
}
