///
pub mod existing {
    use git_hash::ObjectId;

    /// The error returned by the [`find(…)`][crate::FindExt::find()] trait methods.
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error<T: std::error::Error + 'static> {
        #[error(transparent)]
        Find(T),
        #[error("An object with id {} could not be found", .oid)]
        NotFound { oid: ObjectId },
    }
}

///
pub mod existing_object {
    use git_hash::ObjectId;

    /// The error returned by the various [`find_*()`][crate::FindExt::find_commit()] trait methods.
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error<T: std::error::Error + 'static> {
        #[error(transparent)]
        Find(T),
        #[error(transparent)]
        Decode(git_object::decode::Error),
        #[error("An object with id {} could not be found", .oid)]
        NotFound { oid: ObjectId },
        #[error("Expected object of kind {} something else", .expected)]
        ObjectKind { expected: git_object::Kind },
    }
}

///
pub mod existing_iter {
    use git_hash::ObjectId;

    /// The error returned by the various [`find_*_iter()`][crate::FindExt::find_commit_iter()] trait methods.
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error<T: std::error::Error + 'static> {
        #[error(transparent)]
        Find(T),
        #[error("An object with id {} could not be found", .oid)]
        NotFound { oid: ObjectId },
        #[error("Expected object of kind {} something else", .expected)]
        ObjectKind { expected: git_object::Kind },
    }
}
