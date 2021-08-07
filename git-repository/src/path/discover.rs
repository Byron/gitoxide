use crate::path;
use std::path::Path;

pub mod existing {
    use quick_error::quick_error;
    use std::path::PathBuf;

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
    // Canonicalize the path so that `Path::parent` _actually_ gives
    // us the parent directory. (`Path::parent` just strips off the last
    // path component, which means it will not do what you expect when
    // working with paths paths that contain '..'.)
    let directory = directory.as_ref();
    let directory = if let Ok(directory) = directory.canonicalize() {
        directory
    } else {
        return Err(existing::Error::InaccessibleDirectory(directory.into()));
    };
    if !directory.is_dir() {
        return Err(existing::Error::InaccessibleDirectory(directory));
    }

    let mut cursor: &Path = &directory;
    loop {
        if let Ok(kind) = path::is_git(cursor) {
            break Ok(crate::Path::from_dot_git_dir(cursor, kind));
        }
        let git_dir = cursor.join(".git");
        if let Ok(kind) = path::is_git(&git_dir) {
            break Ok(crate::Path::from_dot_git_dir(git_dir, kind));
        }
        match cursor.parent() {
            Some(parent) => cursor = parent,
            None => break Err(existing::Error::NoGitRepository(directory.to_owned())),
        }
    }
}
