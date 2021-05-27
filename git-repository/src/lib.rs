#![forbid(unsafe_code)]
#![deny(rust_2018_idioms)]
#![allow(missing_docs)]
use std::path::PathBuf;

pub mod init;

pub mod path;
pub use path::Path;

pub mod repository;

pub struct Repository {
    pub refs: git_ref::file::Store,
    pub working_tree: Option<PathBuf>,
    pub odb: git_odb::linked::Store,
}

impl Repository {
    pub fn kind(&self) -> Kind {
        match self.working_tree {
            Some(_) => Kind::WorkingTree,
            None => Kind::Bare,
        }
    }

    pub fn git_dir(&self) -> &std::path::Path {
        &self.refs.base
    }
    pub fn objects_dir(&self) -> &std::path::Path {
        &self.odb.dbs[0].loose.path
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Kind {
    Bare,
    WorkingTree,
}

pub mod is_git {
    use quick_error::quick_error;
    use std::path::{Path, PathBuf};

    quick_error! {
        #[derive(Debug)]
        pub enum Error {
            FindHeadRef(err: git_ref::file::find_one::existing::Error) {
                display("Could not find a valid HEAD reference")
                from()
                source(err)
            }
            MisplacedHead(relative_path: PathBuf) {
                display("Expected HEAD at '.git/HEAD', got '.git/{}'", relative_path.display())
            }
            MissingObjectsDirectory(missing: PathBuf) {
                display("Expected an objects directory at '{}'", missing.display())
            }
            MissingRefsDirectory(missing: PathBuf) {
                display("Expected a refs directory at '{}'", missing.display())
            }
        }
    }

    /// What constitutes a valid git repository, and what's yet to be implemented.
    ///
    /// * [x] a valid head
    /// * [ ] git common directory
    ///   * [ ] respect GIT_COMMON_DIR
    /// * [x] an objects directory
    ///   * [x] respect GIT_OBJECT_DIRECTORY
    /// * [x] a refs directory
    pub fn is_git(git_dir: impl AsRef<Path>) -> Result<crate::Kind, Error> {
        let dot_git = git_dir.as_ref();

        {
            let refs = git_ref::file::Store::at(&dot_git);
            let head = refs.find_one_existing("HEAD")?;
            if head.relative_path != Path::new("HEAD") {
                return Err(Error::MisplacedHead(head.relative_path));
            }
        }

        {
            let objects_path = std::env::var("GIT_OBJECT_DIRECTORY")
                .map(PathBuf::from)
                .unwrap_or_else(|_| dot_git.join("objects"));
            if !objects_path.is_dir() {
                return Err(Error::MissingObjectsDirectory(objects_path));
            }
        }
        {
            let refs_path = dot_git.join("refs");
            if !refs_path.is_dir() {
                return Err(Error::MissingRefsDirectory(refs_path));
            }
        }

        Ok(if dot_git.join("index").is_file() {
            crate::Kind::WorkingTree
        } else {
            crate::Kind::Bare
        })
    }
}
pub use is_git::is_git;

pub fn discover(directory: impl AsRef<std::path::Path>) -> Result<Repository, repository::discover::Error> {
    Repository::discover(directory)
}
