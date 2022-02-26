//!

/// An empty array of a type usable with the `git::easy` API to help declaring no parents should be used
pub const NO_PARENT_IDS: [git_hash::ObjectId; 0] = [];

mod error {
    /// The error returned by [`commit(…)`][sync::Handle::commit()].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error(transparent)]
        ReferenceNameValidation(#[from] git_ref::name::Error),
        #[error(transparent)]
        WriteObject(#[from] crate::object::write::Error),
        #[error(transparent)]
        ReferenceEdit(#[from] crate::reference::edit::Error),
    }
}
pub use error::Error;
