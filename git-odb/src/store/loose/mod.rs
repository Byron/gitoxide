//! An object database storing each object in a zlib compressed file with its hash in the path
const HEADER_READ_UNCOMPRESSED_BYTES: usize = 512;

///
pub mod backend;
#[doc(inline)]
pub use backend::Backend;

///
pub mod object;
