#![allow(missing_docs)]

use std::path::PathBuf;

pub use is_git::{is_bare, is_git};

use crate::{Kind, Path};

///
pub mod create;
pub mod discover;
pub mod is_git;

impl AsRef<std::path::Path> for Path {
    fn as_ref(&self) -> &std::path::Path {
        match self {
            Path::WorkTree(path) | Path::Repository(path) => path,
        }
    }
}

impl Path {
    pub fn from_dot_git_dir(dir: impl Into<PathBuf>, kind: Kind) -> Self {
        let dir = dir.into();
        match kind {
            Kind::WorkTree => Path::WorkTree(dir.parent().expect("this is a sub-directory").to_owned()),
            Kind::Bare => Path::Repository(dir),
        }
    }
    pub fn kind(&self) -> Kind {
        match self {
            Path::WorkTree(_) => Kind::WorkTree,
            Path::Repository(_) => Kind::Bare,
        }
    }

    pub fn into_repository_and_work_tree_directories(self) -> (PathBuf, Option<PathBuf>) {
        match self {
            crate::Path::WorkTree(working_tree) => (working_tree.join(".git"), Some(working_tree)),
            crate::Path::Repository(repository) => (repository, None),
        }
    }
}
