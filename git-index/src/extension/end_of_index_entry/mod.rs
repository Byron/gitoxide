use crate::{extension, extension::Signature};

/// The signature of the end-of-index-entry extension
pub const SIGNATURE: Signature = *b"EOIE";
/// The minimal size of the extension, depending on the shortest hash.
pub const MIN_SIZE: usize = 4 /* offset to extensions */ + gix_hash::Kind::shortest().len_in_bytes();
/// The smallest size of the extension varying by hash kind, along with the standard extension header.
pub const MIN_SIZE_WITH_HEADER: usize = extension::MIN_SIZE + MIN_SIZE;

mod decode;
pub use decode::decode;

mod write;
pub use write::write_to;
