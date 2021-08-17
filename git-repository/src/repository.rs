use crate::{Cache, Repository};

mod access {
    use crate::{Kind, Repository};

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
}

mod init {
    use std::path::Path;

    use crate::Repository;

    impl Repository {
        /// Really just a sketch at this point to help guide the API.
        pub fn create_and_init(directory: impl AsRef<Path>) -> Result<Self, crate::init::Error> {
            // TODO: proper error
            crate::init::repository(directory.as_ref())?;
            Ok(Repository::discover(directory).unwrap()) // TODO: a specialized method without discovery
        }
    }
}

pub mod discover {
    use std::path::Path;

    use quick_error::quick_error;

    use crate::{path::discover, Repository};
    use std::cell::RefCell;

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
                refs: git_ref::file::Store::at(
                    git_dir,
                    if working_tree.is_none() {
                        git_ref::file::WriteReflog::Disable
                    } else {
                        git_ref::file::WriteReflog::Normal
                    },
                ),
                working_tree,
            })
        }
    }
}
