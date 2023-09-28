//! This crate includes the various diffs `git` can do between different representations
//! of the repository state, like comparisons between…
//!
//! * index and working tree
//! * index and tree
//! * find untracked files
//!
//! While also being able to check check if the working tree is dirty, quickly.
#![deny(missing_docs, rust_2018_idioms, unsafe_code)]

use bstr::BStr;

pub mod index_as_worktree;
pub use index_as_worktree::function::index_as_worktree;

/// A trait to facilitate working working with pathspecs.
pub trait Pathspec {
    /// Return the portion of the prefix among all of the pathspecs involved in this search, or an empty string if
    /// there is none. It doesn't have to end at a directory boundary though, nor does it denote a directory.
    ///
    /// Note that the common_prefix is always matched case-sensitively, and it is useful to skip large portions of input.
    /// Further, excluded pathspecs don't participate which makes this common prefix inclusive. To work correclty though,
    /// one will have to additionally match paths that have the common prefix with that pathspec itself to assure it is
    /// not excluded.
    fn common_prefix(&self) -> &BStr;

    /// Return `true` if `relative_path` is included in this pathspec.
    /// `is_dir` is `true` if `relative_path` is a directory.
    fn is_included(&mut self, relative_path: &BStr, is_dir: Option<bool>) -> bool;
}

/// A stack that validates we are not going through a symlink in a way that is read-only.
///
/// It can efficiently validate paths when these are queried in sort-order, which leads to each component
/// to only be checked once.
pub struct SymlinkCheck {
    inner: gix_fs::Stack,
}

mod stack;
