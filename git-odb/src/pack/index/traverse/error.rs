use crate::{pack, pack::index};
use git_object::owned;

#[derive(thiserror::Error, Debug)]
pub enum Error<E: std::error::Error + Send + Sync + 'static> {
    #[error("One of the traversal processors failed")]
    Processor(#[source] E),
    #[error("Index file, pack file or object verification failed")]
    Verify(#[from] index::verify::Error),
    #[error("The pack delta tree index could not be built")]
    Tree(#[from] pack::tree::from_offsets::Error),
    #[error("The tree traversal failed")]
    TreeTraversal(#[from] pack::tree::traverse::Error),
    #[error("The pack of this index file failed to verify its checksums")]
    PackChecksum(#[from] pack::data::verify::Error),
    #[error("Object {id} at offset {offset} could not be decoded")]
    PackDecode {
        id: owned::Id,
        offset: u64,
        source: pack::data::decode::Error,
    },
    #[error("The packfiles checksum didn't match the index file checksum: expected {expected}, got {actual}")]
    PackMismatch { expected: owned::Id, actual: owned::Id },
    #[error("The SHA1 of {kind} object at offset {offset} didn't match the checksum in the index file: expected {expected}, got {actual}")]
    PackObjectMismatch {
        expected: owned::Id,
        actual: owned::Id,
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
