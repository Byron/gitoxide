//!

/// An empty array of a type usable with the `git::easy` API to help declaring no parents should be used
pub const NO_PARENT_IDS: [git_hash::ObjectId; 0] = [];

mod error {
    use crate::easy;

    /// The error returned by [`commit(…)`][easy::Handle::commit()].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error(transparent)]
        ReferenceNameValidation(#[from] git_ref::name::Error),
        #[error(transparent)]
        WriteObject(#[from] easy::object::write::Error),
        #[error(transparent)]
        ReferenceEdit(#[from] easy::reference::edit::Error),
    }
}
pub use error::Error;
