/// The error returned by methods of the [Find](crate::Find) trait.
pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

///
pub mod existing {
    use gix_hash::ObjectId;

    /// The error returned by the [`find(…)`](crate::FindExt::find()) trait methods.
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
pub mod existing_object {
    use gix_hash::ObjectId;

    /// The error returned by the various [`find_*`](crate::FindExt::find_commit()) trait methods.
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error(transparent)]
        Find(crate::find::Error),
        #[error(transparent)]
        Decode(gix_object::decode::Error),
        #[error("An object with id {} could not be found", .oid)]
        NotFound { oid: ObjectId },
        #[error("Expected object of kind {} something else", .expected)]
        ObjectKind { expected: gix_object::Kind },
    }
}

///
pub mod existing_iter {
    use gix_hash::ObjectId;

    /// The error returned by the various [`find_*`](crate::FindExt::find_commit()) trait methods.
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error(transparent)]
        Find(crate::find::Error),
        #[error("An object with id {} could not be found", .oid)]
        NotFound { oid: ObjectId },
        #[error("Expected object of kind {} something else", .expected)]
        ObjectKind { expected: gix_object::Kind },
    }
}

/// An Entry in a pack providing access to its data.
///
/// Its commonly retrieved by reading from a pack index file followed by a read from a pack data file.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[allow(missing_docs)]
pub struct Entry {
    /// The pack-data encoded bytes of the pack data entry as present in the pack file, including the header followed by compressed data.
    pub data: Vec<u8>,
    /// The version of the pack file containing `data`
    pub version: crate::data::Version,
}
