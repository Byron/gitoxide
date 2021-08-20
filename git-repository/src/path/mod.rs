#![allow(missing_docs)]
use std::path::PathBuf;

use crate::Kind;

pub mod discover;
pub mod is_git;
pub use is_git::{is_bare, is_git};

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Path {
    WorkTree(PathBuf),
    Repository(PathBuf),
}

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

    pub fn into_repository_directory(self) -> PathBuf {
        match self {
            Path::WorkTree(path) => path.join(".git"),
            Path::Repository(path) => path,
        }
    }
}
