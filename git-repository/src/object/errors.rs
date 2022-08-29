///
pub mod conversion {

    /// The error returned by [`crate::object::try_to_()`][crate::Object::try_to_commit_ref()].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error(transparent)]
        Decode(#[from] git_object::decode::Error),
        #[error("Expected object type {}, but got {}", .expected, .actual)]
        UnexpectedType {
            expected: git_object::Kind,
            actual: git_object::Kind,
        },
    }
}

///
pub mod find {
    /// Indicate that an error occoured when trying to find an object.
    pub type Error = git_odb::store::find::Error;

    ///
    pub mod existing {
        /// An object could not be found in the database, or an error occurred when trying to obtain it.
        pub type Error = git_odb::find::existing::Error<git_odb::store::find::Error>;
    }
}

///
pub mod write {
    /// An error to indicate writing to the loose object store failed.
    pub type Error = git_odb::store::write::Error;
}
