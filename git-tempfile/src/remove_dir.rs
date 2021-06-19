//!
#![allow(missing_docs)]

use std::path::Path;

/// The amount of retries to do during various aspects of the directory deletion.
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub struct Retries {
    /// How many times we can try to delete the whole directory while being disturbed by racy interference.
    /// This count combats racy situations where another process is trying to create a directory that we want to delete,
    /// and is deliberately lower than those who do creation. That way, creation usually wins which is preferable as we run
    /// as part of the cleanup.
    pub to_delete_entire_directory_tree_until_boundary: usize,
    /// How often to retry to delete a single directory if an interrupt happens, as caused by signals.
    pub on_interrupt: usize,
}

impl Default for Retries {
    fn default() -> Self {
        Retries {
            on_interrupt: 10,
            to_delete_entire_directory_tree_until_boundary: 1,
        }
    }
}

mod error {
    use crate::create_dir::Retries;
    use std::{fmt, path::Path};

    /// The error returned by [empty_until_boundary()][super::empty_until_boundary()].
    #[allow(missing_docs)]
    #[derive(Debug)]
    pub enum Error<'a> {
        /// A failure we will probably recover from by trying again.
        Intermediate { dir: &'a Path, kind: std::io::ErrorKind },
        /// A failure that ends the operation.
        Permanent {
            dir: &'a Path,
            err: std::io::Error,
            retries_left: Retries,
        },
    }

    impl<'a> fmt::Display for Error<'a> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Error::Intermediate { dir, kind } => write!(
                    f,
                    "Intermediae failure creating {:?} with error: {:?}",
                    dir.display(),
                    kind
                ),
                Error::Permanent {
                    err: _,
                    dir,
                    retries_left,
                } => write!(
                    f,
                    "Permanently failing to create directory {:?} ({:?})",
                    dir, retries_left
                ),
            }
        }
    }

    impl<'a> std::error::Error for Error<'a> {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            match self {
                Error::Permanent { err, .. } => Some(err),
                _ => None,
            }
        }
    }
}
pub use error::Error;

/// Delete all empty directories from `delete_dir` upward and until (not including) the `boundary_dir`.
///
/// Note that `boundary_dir` must contain `delete_dir` or an error is returned.
pub fn empty_until_boundary<'a>(_delete_dir: &'a Path, _boundary_dir: &Path) -> std::io::Result<&'a Path> {
    todo!("delete empty with iterator")
}
