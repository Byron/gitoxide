const HEADER_READ_COMPRESSED_BYTES: usize = 256;
const HEADER_READ_UNCOMPRESSED_BYTES: usize = 512;

pub(crate) mod db;

pub mod object;
pub use object::Object;

pub use db::{Db, Error};
