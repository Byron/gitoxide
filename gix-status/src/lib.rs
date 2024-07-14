//! This crate includes the various diffs `git` can do between different representations
//! of the repository state, like comparisons between…
//!
//! * index and working tree
//! * index and tree
//! * find untracked files
//!
//! While also being able to check check if the working tree is dirty, quickly.
//!
//! ### Feature Flags
#![cfg_attr(
    all(doc, feature = "document-features"),
    doc = ::document_features::document_features!()
)]
#![cfg_attr(all(doc, feature = "document-features"), feature(doc_cfg, doc_auto_cfg))]
#![deny(missing_docs, rust_2018_idioms, unsafe_code)]

#[cfg(target_has_atomic = "64")]
use std::sync::atomic::AtomicU64;

#[cfg(not(target_has_atomic = "64"))]
use portable_atomic::AtomicU64;

pub mod index_as_worktree;
pub use index_as_worktree::function::index_as_worktree;

#[cfg(feature = "worktree-rewrites")]
pub mod index_as_worktree_with_renames;
#[cfg(feature = "worktree-rewrites")]
pub use index_as_worktree_with_renames::function::index_as_worktree_with_renames;

/// A stack that validates we are not going through a symlink in a way that is read-only.
///
/// It can efficiently validate paths when these are queried in sort-order, which leads to each component
/// to only be checked once.
pub struct SymlinkCheck {
    inner: gix_fs::Stack,
}

mod stack;

fn is_dir_to_mode(is_dir: bool) -> gix_index::entry::Mode {
    if is_dir {
        gix_index::entry::Mode::DIR
    } else {
        gix_index::entry::Mode::FILE
    }
}
