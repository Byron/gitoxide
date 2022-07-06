//! A file with directories of other git object databases to use when reading objects.
//!
//! This inherently makes alternates read-only.
//!
//! An alternate file in `<git-dir>/info/alternates` can look as follows:
//!
//! ```text
//! # a comment, empty lines are also allowed
//! # relative paths resolve relative to the parent git repository
//! ../path/relative/to/repo/.git
//! /absolute/path/to/repo/.git
//!
//! "/a/ansi-c-quoted/path/with/tabs\t/.git"
//!
//! # each .git directory should indeed be a directory, and not a file
//! ```
//!
//! Based on the [canonical implementation](https://github.com/git/git/blob/master/sha1-file.c#L598:L609).
use git_path::realpath::MAX_SYMLINKS;
use std::{fs, io, path::PathBuf};

///
pub mod parse;

/// Returned by [`resolve()`]
#[derive(thiserror::Error, Debug)]
#[allow(missing_docs)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Realpath(#[from] git_path::realpath::Error),
    #[error(transparent)]
    Parse(#[from] parse::Error),
    #[error("Alternates form a cycle: {} -> {}", .0.iter().map(|p| format!("'{}'", p.display())).collect::<Vec<_>>().join(" -> "), .0.first().expect("more than one directories").display())]
    Cycle(Vec<PathBuf>),
}

/// Given an objects directory, try to resolve alternate object directories possibly located in the
/// `./info/alternates` file into canonical paths.
/// If no alternate object database was resolved, the resulting `Vec` is empty (it is not an error
/// if there are no alternates).
/// It is an error once a repository is seen again as it would lead to a cycle.
pub fn resolve(objects_directory: impl Into<PathBuf>) -> Result<Vec<PathBuf>, Error> {
    let relative_base = objects_directory.into();
    let mut dirs = vec![(0, relative_base.clone())];
    let mut out = Vec::new();
    let cwd = std::env::current_dir()?;
    let mut seen = vec![git_path::realpath_opts(&relative_base, &cwd, MAX_SYMLINKS)?];
    while let Some((depth, dir)) = dirs.pop() {
        match fs::read(dir.join("info").join("alternates")) {
            Ok(input) => {
                for path in parse::content(&input)?.into_iter() {
                    let path = relative_base.join(path);
                    let path_canonicalized = git_path::realpath_opts(&path, &cwd, MAX_SYMLINKS)?;
                    if seen.contains(&path_canonicalized) {
                        return Err(Error::Cycle(seen));
                    }
                    seen.push(path_canonicalized);
                    dirs.push((depth + 1, path));
                }
            }
            Err(err) if err.kind() == io::ErrorKind::NotFound => {}
            Err(err) => return Err(err.into()),
        };
        if depth != 0 {
            out.push(dir);
        }
    }
    Ok(out)
}
