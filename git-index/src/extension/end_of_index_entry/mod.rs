use crate::{extension, extension::Signature};

pub const SIGNATURE: Signature = *b"EOIE";
pub const SIZE: usize = 4 /* offset to extensions */ + git_hash::Kind::Sha1.len_in_bytes();
pub const SIZE_WITH_HEADER: usize = extension::MIN_SIZE + SIZE;

mod decode;
pub use decode::decode;
