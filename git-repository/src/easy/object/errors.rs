///
pub mod conversion {

    /// The error returned by [`easy::ObjectRef::try_to_()`][crate::easy::ObjectRef::try_to_commit()].
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
    pub(crate) type OdbError = git_odb::compound::find::Error;

    ///
    pub mod existing {
        pub(crate) type OdbError = git_odb::find::existing::Error<git_odb::compound::find::Error>;
    }
}

///
pub mod write {
    /// An error to indicate writing to the loose object store failed.
    pub type Error = git_odb::loose::write::Error;
}
