use crate::index;

/// Returned by [`index::File::traverse_with_index()`] and [`index::File::traverse_with_lookup`]
#[derive(thiserror::Error, Debug)]
#[allow(missing_docs)]
pub enum Error<E: std::error::Error + Send + Sync + 'static> {
    #[error("One of the traversal processors failed")]
    Processor(#[source] E),
    #[error("Index file, pack file or object verification failed")]
    Verify(#[from] index::verify::Error),
    #[error("The pack delta tree index could not be built")]
    Tree(#[from] crate::tree::from_offsets::Error),
    #[error("The tree traversal failed")]
    TreeTraversal(#[from] crate::tree::traverse::Error),
    #[error("The pack of this index file failed to verify its checksums")]
    PackChecksum(#[from] crate::data::verify::Error),
    #[error("Object {id} at offset {offset} could not be decoded")]
    PackDecode {
        id: git_hash::ObjectId,
        offset: u64,
        source: crate::data::decode_entry::Error,
    },
    #[error("The packfiles checksum didn't match the index file checksum: expected {expected}, got {actual}")]
    PackMismatch {
        expected: git_hash::ObjectId,
        actual: git_hash::ObjectId,
    },
    #[error("The SHA1 of {kind} object at offset {offset} didn't match the checksum in the index file: expected {expected}, got {actual}")]
    PackObjectMismatch {
        expected: git_hash::ObjectId,
        actual: git_hash::ObjectId,
        offset: u64,
        kind: git_object::Kind,
    },
    #[error("The CRC32 of {kind} object at offset {offset} didn't match the checksum in the index file: expected {expected}, got {actual}")]
    Crc32Mismatch {
        expected: u32,
        actual: u32,
        offset: u64,
        kind: git_object::Kind,
    },
    #[error("Interrupted")]
    Interrupted,
}
