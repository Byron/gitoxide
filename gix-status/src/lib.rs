//! This crate includes the various diffs `git` can do between different representations
//! of the repository state, like comparisons between…
//!
//! * index and working tree
//! * index and tree
//! * find untracked files
//!
//! While also being able to check check if the working tree is dirty, quickly.
#![deny(missing_docs, rust_2018_idioms, unsafe_code)]

pub mod index_as_worktree;
pub use index_as_worktree::function::index_as_worktree;

/// A stack that validates we are not going through a symlink in a way that is read-only.
///
/// It can efficiently validate paths when these are queried in sort-order, which leads to each component
/// to only be checked once.
pub struct SymlinkCheck {
    inner: gix_fs::Stack,
}

mod stack;
