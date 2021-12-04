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

    use crate::easy;

    pub(crate) type OdbError = git_odb::compound::find::Error;

    /// The error returned by [`ObjectAccessExt::try_find_object()`][easy::ext::ObjectAccessExt::try_find_object()].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error(transparent)]
        Find(#[from] OdbError),
        #[error("BUG: Part of interior state could not be borrowed.")]
        BorrowState(#[from] easy::borrow::state::Error),
    }

    ///
    pub mod existing {
        use crate::easy;

        pub(crate) type OdbError = git_odb::find::existing::Error<git_odb::compound::find::Error>;

        /// The error returned by [`ObjectAccessExt::find_object()`][easy::ext::ObjectAccessExt::find_object()].
        #[derive(Debug, thiserror::Error)]
        #[allow(missing_docs)]
        pub enum Error {
            #[error(transparent)]
            FindExisting(#[from] OdbError),
            #[error("BUG: Part of interior state could not be borrowed.")]
            BorrowState(#[from] easy::borrow::state::Error),
        }
    }
}

///
pub mod write {
    /// An error to indicate writing to the loose object store failed.
    pub type Error = git_odb::loose::write::Error;
}
