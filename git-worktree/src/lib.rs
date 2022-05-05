//! ## Feature Flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![deny(unsafe_code)]
#![deny(rust_2018_idioms)]

/// Everything there is to know about a worktree.
pub struct State;

mod state {
    use crate::State;
    use bstr::BString;

    impl State {
        /// Return true if this worktree is the main worktree associated with a non-bare git repository.
        ///
        /// It cannot be removed.
        pub fn is_main() -> bool {
            todo!()
        }

        /// Return true if the worktree cannot be pruned, moved or deleted, which is useful if it is located on an external storage device.
        pub fn is_locked(&self) -> bool {
            todo!()
        }
        /// Provide a reason for the locking of this worktree, if it is locked at all.
        ///
        /// Note that we squelch errors in case the file cannot be read in which case the
        /// reason is an empty string.
        pub fn lock_reason(&self) -> Option<BString> {
            todo!()
        }
    }
}

/// file system related utilities
pub mod fs;

pub mod index;

pub(crate) mod os;
