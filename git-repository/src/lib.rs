#![forbid(unsafe_code)]
#![deny(rust_2018_idioms)]
#![allow(missing_docs)]
use std::path::PathBuf;

pub mod discover;
pub mod init;

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

pub mod repository {
    pub mod discover {
        use crate::{discover, Repository};
        use quick_error::quick_error;
        use std::path::Path;

        quick_error! {
            #[derive(Debug)]
            pub enum Error {
                Discover(err: discover::existing::Error) {
                    display("Could not find a valid git repository directory")
                    from()
                    source(err)
                }
                ObjectStoreInitialization(err: git_odb::linked::init::Error) {
                    display("Could not initialize the object database")
                    from()
                    source(err)
                }
            }
        }

        impl Repository {
            pub fn discover(directory: impl AsRef<Path>) -> Result<Self, Error> {
                let path = discover::existing(directory)?;
                let (git_dir, working_tree) = match path {
                    crate::Path::WorkingTree(working_tree) => (working_tree.join(".git"), Some(working_tree)),
                    crate::Path::Repository(repository) => (repository, None),
                };
                Ok(Repository {
                    odb: git_odb::linked::Store::at(git_dir.join("objects"))?,
                    refs: git_ref::file::Store::at(git_dir),
                    working_tree,
                })
            }
        }
    }
}

pub fn discover(directory: impl AsRef<std::path::Path>) -> Result<Repository, repository::discover::Error> {
    Repository::discover(directory)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Kind {
    Bare,
    WorkingTree,
}

mod path {
    use crate::Kind;
    use std::path::PathBuf;

    #[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
    pub enum Path {
        WorkingTree(PathBuf),
        Repository(PathBuf),
    }

    impl AsRef<std::path::Path> for Path {
        fn as_ref(&self) -> &std::path::Path {
            match self {
                Path::WorkingTree(path) | Path::Repository(path) => path,
            }
        }
    }

    impl Path {
        pub fn from_dot_git_dir(dir: impl Into<PathBuf>, kind: Kind) -> Self {
            let dir = dir.into();
            match kind {
                Kind::WorkingTree => Path::WorkingTree(dir.parent().expect("this is a sub-directory").to_owned()),
                Kind::Bare => Path::Repository(dir),
            }
        }
        pub fn kind(&self) -> Kind {
            match self {
                Path::WorkingTree(_) => Kind::WorkingTree,
                Path::Repository(_) => Kind::Bare,
            }
        }

        pub fn into_repository_directory(self) -> PathBuf {
            match self {
                Path::WorkingTree(path) => path.join(".git"),
                Path::Repository(path) => path,
            }
        }
    }
}
pub use path::Path;

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
