use std::{
    borrow::Cow,
    path::{Component, Path},
};

use crate::path;

pub mod existing {
    use std::path::PathBuf;

    use quick_error::quick_error;

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
    let directory = maybe_canonicalize(directory.as_ref())
        .map_err(|_| existing::Error::InaccessibleDirectory(directory.as_ref().into()))?;
    if !directory.is_dir() {
        return Err(existing::Error::InaccessibleDirectory(directory.into_owned()));
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
            None => break Err(existing::Error::NoGitRepository(directory.into_owned())),
        }
    }
}

fn maybe_canonicalize(path: &Path) -> std::io::Result<Cow<'_, Path>> {
    let ends_with_relative_component = path
        .components()
        .last()
        .map_or(true, |c| matches!(c, Component::CurDir | Component::ParentDir));
    if ends_with_relative_component {
        path.canonicalize().map(Into::into)
    } else {
        Ok(path.into())
    }
}

#[cfg(test)]
mod maybe_canonicalize {
    use super::*;

    fn relative_component_count(path: impl AsRef<Path>) -> usize {
        path.as_ref()
            .components()
            .filter(|c| matches!(c, Component::CurDir | Component::ParentDir))
            .count()
    }

    #[test]
    fn empty_paths_are_invalid() {
        assert!(
            maybe_canonicalize(Path::new("")).is_err(),
            "empty paths are not equivalent to '.' but are non-existing"
        );
    }

    #[test]
    fn paths_starting_with_dot_but_end_with_normal_path_are_not_canonicalized() {
        assert_eq!(
            relative_component_count(maybe_canonicalize(&Path::new(".").join("hello")).unwrap()),
            1,
        );
    }

    #[test]
    fn paths_ending_with_non_normal_component_are_canonicalized() {
        assert_eq!(
            relative_component_count(maybe_canonicalize(&Path::new(".").join(".")).unwrap()),
            0,
        );
    }
}
