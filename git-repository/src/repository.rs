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
    use crate::Path;
    use std::convert::TryFrom;

    pub type Error = git_odb::linked::init::Error;

    impl TryFrom<crate::Path> for crate::Repository {
        type Error = Error;

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

pub mod init {
    use quick_error::quick_error;
    use std::path::Path;

    quick_error! {
        #[derive(Debug)]
        pub enum Error {
            Init(err: crate::init::Error) {
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

    use crate::Repository;
    use std::convert::TryInto;

    impl Repository {
        /// Create a repository with work-tree within `directory`, creating intermediate directories as needed.
        ///
        /// Fails without action if there is already a `.git` repository inside of `directory`, but
        /// won't mind if the `directory` otherwise is non-empty.
        pub fn init(directory: impl AsRef<Path>) -> Result<Self, Error> {
            let path = crate::init::into(directory.as_ref())?;
            Ok(path.try_into()?)
        }
    }
}

pub mod discover {
    use std::path::Path;

    use quick_error::quick_error;

    use crate::{path::discover, Repository};
    use std::convert::TryInto;

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
