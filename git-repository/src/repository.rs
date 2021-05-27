pub mod discover {
    use crate::{path::discover, Repository};
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
