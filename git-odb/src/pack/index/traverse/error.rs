use crate::{pack, pack::index};
use git_object::owned;
use quick_error::quick_error;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Processor(err: Box<dyn std::error::Error + Send + Sync>) {
            display("One of the traversal processors failed")
            source(&**err)
            from()
        }
        Verify(err: index::verify::Error) {
            display("Index file, pack file or object verification failed")
            source(err)
            from()
        }
        Tree(err: pack::tree::from_offsets::Error) {
            display("The pack delta tree index could not be built")
            from()
            source(err)
        }
        TreeTraversal(err: pack::tree::traverse::Error) {
            display("The tree traversal failed")
            from()
            source(err)
        }
        PackChecksum(err: pack::data::verify::Error) {
            display("The pack of this index file failed to verify its checksums")
            from()
            source(err)
        }
        PackDecode(err: pack::data::decode::Error, id: owned::Id, offset: u64) {
            display("Object {} at offset {} could not be decoded", id, offset)
            source(err)
        }
        PackMismatch { expected: owned::Id, actual: owned::Id } {
            display("The packfiles checksum didn't match the index file checksum: expected {}, got {}", expected, actual)
        }
        PackObjectMismatch { expected: owned::Id, actual: owned::Id, offset: u64, kind: git_object::Kind} {
            display("The SHA1 of {} object at offset {} didn't match the checksum in the index file: expected {}, got {}", kind, offset, expected, actual)
        }
        Crc32Mismatch { expected: u32, actual: u32, offset: u64, kind: git_object::Kind} {
            display("The CRC32 of {} object at offset {} didn't match the checksum in the index file: expected {}, got {}", kind, offset, expected, actual)
        }
        Interrupted {
            display("Interrupted")
        }
    }
}
