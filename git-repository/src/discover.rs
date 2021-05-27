use std::path::Path;

pub mod existing {
    use quick_error::quick_error;
    use std::path::PathBuf;

    mod repository {
        use crate::{discover, Repository};
        use std::path::Path;

        impl Repository {
            pub fn discover(directory: impl AsRef<Path>) -> Result<Self, discover::existing::Error> {
                let path = discover::existing(directory)?;
                let (git_dir, working_tree) = match path {
                    crate::Path::WorkingTree(working_tree) => (working_tree.join(".git"), Some(working_tree)),
                    crate::Path::Repository(repository) => (repository, None),
                };
                Ok(Repository {
                    odb: git_odb::linked::Store::at(git_dir.join("objects")).unwrap(), // TODO: proper error
                    refs: git_ref::file::Store::at(git_dir),
                    working_tree,
                })
            }
        }
    }

    quick_error! {
        #[derive(Debug)]
        pub enum Error {
            InaccessibleDirectory(path: PathBuf) {
                display("Failed to access a directory, or path is not a direectory")
            }
            NoGitRepository(path: PathBuf) {
                display("Could find a git repository in '{}' or in any of its parents", path.display())
            }
        }
    }
}

/// Returns the working tree if possible and the found repository is not bare or the git repository itself.
pub fn existing(directory: impl AsRef<Path>) -> Result<crate::Path, existing::Error> {
    let directory = directory.as_ref();
    if !directory.is_dir() {
        return Err(existing::Error::InaccessibleDirectory(directory.into()));
    }

    let mut cursor = directory;
    loop {
        if let Ok(kind) = crate::is_git(cursor) {
            break Ok(crate::Path::from_dot_git_dir(cursor, kind));
        }
        let git_dir = cursor.join(".git");
        if let Ok(kind) = crate::is_git(&git_dir) {
            break Ok(crate::Path::from_dot_git_dir(git_dir, kind));
        }
        match cursor.parent() {
            Some(parent) => cursor = parent,
            None => break Err(existing::Error::NoGitRepository(directory.to_owned())),
        }
    }
}
