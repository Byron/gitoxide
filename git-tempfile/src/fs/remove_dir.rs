//!
#![allow(missing_docs)]

pub use super::error::Error;
use std::path::Path;

/// A special iterator which communicates its operation through results where…
///
/// * `Some(Ok(removed_directory))` is yielded once or more success, followed by `None`
/// * `Some(Err(std::io::Error))` is yielded exactly once on failure.
pub struct Iter<'a> {
    cursor: Option<&'a Path>,
    boundary: &'a Path,
}

/// Construction
impl<'a> Iter<'a> {
    /// Create a new instance that deletes `target` but will stop at `boundary`, without deleting the latter.
    /// Returns an error if `boundary` doesn't contain `target`
    ///
    /// **Note** that we don't canonicalize the path for performance reasons.
    pub fn new(target: &'a Path, boundary: &'a Path) -> std::io::Result<Self> {
        if !target.starts_with(boundary) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!(
                    "Removal target {:?} must be contained in boundary {:?}",
                    target, boundary
                ),
            ));
        }
        let cursor = if target == boundary { None } else { Some(target) };
        Ok(Iter { cursor, boundary })
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = std::io::Result<&'a Path>;

    fn next(&mut self) -> Option<Self::Item> {
        use std::io::ErrorKind::*;
        match self.cursor.take() {
            Some(dir) => {
                let next = match std::fs::remove_dir(dir) {
                    Ok(()) => Some(Ok(dir)),
                    Err(err) => match err.kind() {
                        NotFound => Some(Ok(dir)),
                        _other_error_kind => return Some(Err(err)),
                    },
                };
                self.cursor = match dir.parent() {
                    Some(parent) => {
                        if parent == self.boundary {
                            None
                        } else {
                            Some(parent)
                        }
                    },
                    None => unreachable!("directory {:?} ran out of parents, this really shouldn't happen before hitting the boundary {:?}"),
                };
                next
            }
            None => None,
        }
    }
}

/// Delete all empty directories from `delete_dir` upward and until (not including) the `boundary_dir`.
///
/// Note that `boundary_dir` must contain `delete_dir` or an error is returned, otherwise `delete_dir` is returned on success.
pub fn empty_until_boundary<'a>(delete_dir: &'a Path, boundary_dir: &'a Path) -> std::io::Result<&'a Path> {
    for item in Iter::new(delete_dir, boundary_dir)? {
        match item {
            Ok(_dir) => continue,
            Err(err) => return Err(err),
        }
    }
    Ok(delete_dir)
}
