#![allow(missing_docs)]
mod error {
    use crate::easy;

    #[derive(Debug, thiserror::Error)]
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
