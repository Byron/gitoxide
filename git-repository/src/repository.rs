#![allow(missing_docs)]
mod access {
    use crate::{Kind, Repository};

    impl Repository {
        pub fn kind(&self) -> Kind {
            match self.work_tree {
                Some(_) => Kind::WorkTree,
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
}

pub mod from_path {
    use std::convert::TryFrom;

    use crate::Path;

    pub type Error = git_odb::linked::init::Error;

    impl TryFrom<crate::Path> for crate::Repository {
        type Error = Error;

        // TODO: Don't use this for cases where input paths are given to save on IOPs, there may be
        //       duplicate file checks here.
        fn try_from(value: Path) -> Result<Self, Self::Error> {
            let (git_dir, working_tree) = value.into_repository_and_work_tree_directories();
            Ok(crate::Repository {
                odb: git_odb::linked::Store::at(git_dir.join("objects"))?,
                refs: git_ref::file::Store::at(
                    git_dir,
                    if working_tree.is_none() {
                        git_ref::file::WriteReflog::Disable
                    } else {
                        git_ref::file::WriteReflog::Normal
                    },
                ),
                work_tree: working_tree,
            })
        }
    }
}

pub mod open {
    use crate::Repository;

    use quick_error::quick_error;
    use std::convert::TryInto;

    quick_error! {
        #[derive(Debug)]
        pub enum Error {
            NotARepository(err: crate::path::is_git::Error) {
                display("The provided path doesn't appear to be a git repository")
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
        pub fn open(path: impl Into<std::path::PathBuf>) -> Result<Self, Error> {
            let path = path.into();
            let (path, kind) = match crate::path::is_git(&path) {
                Ok(kind) => (path, kind),
                Err(_) => {
                    let git_dir = path.join(".git");
                    crate::path::is_git(&git_dir).map(|kind| (git_dir, kind))?
                }
            };
            crate::Path::from_dot_git_dir(path, kind).try_into().map_err(Into::into)
        }
    }
}

pub mod init {
    use std::path::Path;

    use quick_error::quick_error;

    quick_error! {
        #[derive(Debug)]
        pub enum Error {
            Init(err: crate::path::create::Error) {
                display("Failed to initialize a new repository")
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

    use std::convert::TryInto;

    use crate::Repository;

    impl Repository {
        /// Create a repository with work-tree within `directory`, creating intermediate directories as needed.
        ///
        /// Fails without action if there is already a `.git` repository inside of `directory`, but
        /// won't mind if the `directory` otherwise is non-empty.
        pub fn init(directory: impl AsRef<Path>) -> Result<Self, Error> {
            let path = crate::path::create::into(directory.as_ref())?;
            Ok(path.try_into()?)
        }
    }
}

pub mod discover {
    use std::{convert::TryInto, path::Path};

    use quick_error::quick_error;

    use crate::{path::discover, Repository};

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
            Ok(path.try_into()?)
        }
    }
}

mod impls {
    use crate::Repository;

    impl std::fmt::Debug for Repository {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "Repository(git = '{}', working_tree: {:?}",
                self.git_dir().display(),
                self.work_tree
            )
        }
    }

    impl PartialEq<Repository> for Repository {
        fn eq(&self, other: &Repository) -> bool {
            self.git_dir() == other.git_dir() && self.work_tree == other.work_tree
        }
    }
}
