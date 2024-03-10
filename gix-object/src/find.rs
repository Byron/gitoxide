/// The error type returned by the [`Find`](crate::Find) trait.
pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
///
#[allow(clippy::empty_docs)]
pub mod existing {
    use gix_hash::ObjectId;

    /// The error returned by the [`find(…)`][crate::FindExt::find()] trait methods.
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error(transparent)]
        Find(crate::find::Error),
        #[error("An object with id {} could not be found", .oid)]
        NotFound { oid: ObjectId },
    }
}

///
#[allow(clippy::empty_docs)]
pub mod existing_object {
    use gix_hash::ObjectId;

    /// The error returned by the various [`find_*()`][crate::FindExt::find_commit()] trait methods.
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error(transparent)]
        Find(crate::find::Error),
        #[error("Could not decode object at {oid}")]
        Decode {
            oid: ObjectId,
            source: crate::decode::Error,
        },
        #[error("An object with id {oid} could not be found")]
        NotFound { oid: ObjectId },
        #[error("Expected object of kind {expected} but got {actual} at {oid}")]
        ObjectKind {
            oid: ObjectId,
            actual: crate::Kind,
            expected: crate::Kind,
        },
    }
}

///
#[allow(clippy::empty_docs)]
pub mod existing_iter {
    use gix_hash::ObjectId;

    /// The error returned by the various [`find_*_iter()`][crate::FindExt::find_commit_iter()] trait methods.
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error(transparent)]
        Find(crate::find::Error),
        #[error("An object with id {oid} could not be found")]
        NotFound { oid: ObjectId },
        #[error("Expected object of kind {expected} but got {actual} at {oid}")]
        ObjectKind {
            oid: ObjectId,
            actual: crate::Kind,
            expected: crate::Kind,
        },
    }
}

/// An implementation of all traits that never fails, but also never finds anything.
#[derive(Debug, Copy, Clone)]
pub struct Never;

impl super::FindHeader for Never {
    fn try_header(&self, _id: &gix_hash::oid) -> Result<Option<crate::Header>, Error> {
        Ok(None)
    }
}

impl super::Find for Never {
    fn try_find<'a>(&self, _id: &gix_hash::oid, _buffer: &'a mut Vec<u8>) -> Result<Option<crate::Data<'a>>, Error> {
        Ok(None)
    }
}

impl super::Exists for Never {
    fn exists(&self, _id: &gix_hash::oid) -> bool {
        false
    }
}
