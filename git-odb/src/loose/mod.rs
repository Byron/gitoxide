const HEADER_READ_COMPRESSED_BYTES: usize = 256;
const HEADER_READ_UNCOMPRESSED_BYTES: usize = 512;

pub mod db;
#[doc(inline)]
pub use db::Db;

pub mod object;
#[doc(inline)]
pub use object::Object;
