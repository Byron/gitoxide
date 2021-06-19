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

pub use super::error::Error;

/// Delete all empty directories from `delete_dir` upward and until (not including) the `boundary_dir`.
///
/// Note that `boundary_dir` must contain `delete_dir` or an error is returned.
pub fn empty_until_boundary<'a>(_delete_dir: &'a Path, _boundary_dir: &Path) -> std::io::Result<&'a Path> {
    todo!("delete empty with iterator")
}
