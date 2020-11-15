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
use crate::compound;
use std::{fs, io, path::PathBuf};

#[allow(missing_docs)]
pub mod parse;
#[allow(missing_docs)]
pub mod unquote;

/// Returned by [`resolve()`]
#[derive(thiserror::Error, Debug)]
#[allow(missing_docs)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Parse(#[from] parse::Error),
    #[error(transparent)]
    Init(#[from] compound::init::Error),
    #[error("Alternates form a cycle: {} -> {}", .0.iter().map(|p| format!("'{}'", p.display())).collect::<Vec<_>>().join(" -> "), .0.first().expect("more than one directories").display())]
    Cycle(Vec<PathBuf>),
}

/// Given an objects directory, try to resolve alternate object directories possibly located in the
/// `./info/alternates` file.
/// If no alternate object database was resolved, the reesulting `Vec` is empty, and it is not an error
/// if there are no alternates, or if there is a cycle while there is at least one valid alternate.
pub fn resolve(objects_directory: impl Into<PathBuf>) -> Result<Vec<compound::Db>, Error> {
    let relative_base = objects_directory.into();
    let mut dirs = vec![(0, relative_base.clone())];
    let mut out = Vec::new();
    let mut seen = vec![relative_base.canonicalize()?];
    while let Some((depth, dir)) = dirs.pop() {
        match fs::read(dir.join("info").join("alternates")) {
            Ok(input) => {
                for path in parse::content(&input)?.into_iter() {
                    let path = relative_base.join(path);
                    let path_canonicalized = path.canonicalize()?;
                    if seen.contains(&path_canonicalized) {
                        continue;
                    }
                    seen.push(path_canonicalized);
                    dirs.push((depth + 1, path));
                }
            }
            Err(err) if err.kind() == io::ErrorKind::NotFound => {
                // Only resolve for repositories with at least one link, otherwise the line below causes infinite recursion
                if depth != 0 {
                    // The tail of a chain doesn't have alternates, and thus is the real deal
                    out.push(compound::Db::at(dir)?);
                }
            }
            Err(err) => return Err(err.into()),
        };
    }

    if out.is_empty() && seen.len() > 1 {
        return Err(Error::Cycle(seen));
    }
    Ok(out)
}
