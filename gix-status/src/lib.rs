//! This crate includes the various diffs `git` can do between different representations
//! of the repository state, like comparisons between…
//!
//! * index and working tree
//! * index and tree
//! * find untracked files
//!
//! While also being able to check check if the working tree is dirty, quickly.
#![deny(missing_docs, rust_2018_idioms, unsafe_code)]

///
pub mod read;

pub mod index_as_worktree;
pub use index_as_worktree::function::index_as_worktree;
