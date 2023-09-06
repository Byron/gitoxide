///
pub mod conversion {

    /// The error returned by [`crate::object::try_to_()`][crate::Object::try_to_commit_ref()].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error(transparent)]
        Decode(#[from] gix_object::decode::Error),
        #[error("Expected object type {}, but got {}", .expected, .actual)]
        UnexpectedType {
            expected: gix_object::Kind,
            actual: gix_object::Kind,
        },
    }
}

///
pub mod find {
    /// Indicate that an error occurred when trying to find an object.
    #[derive(Debug, thiserror::Error)]
    #[error(transparent)]
    pub struct Error(#[from] pub gix_odb::find::Error);

    ///
    pub mod existing {
        /// An object could not be found in the database, or an error occurred when trying to obtain it.
        pub type Error = gix_odb::find::existing::Error;
    }
}

///
pub mod write {
    /// An error to indicate writing to the loose object store failed.
    #[derive(Debug, thiserror::Error)]
    #[error(transparent)]
    pub struct Error(#[from] pub gix_odb::find::Error);
}
