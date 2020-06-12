const HEADER_READ_COMPRESSED_BYTES: usize = 512;
const HEADER_READ_UNCOMPRESSED_BYTES: usize = 1024;

mod db;
mod object;

pub use db::{Db, Error as DbError};
pub use object::*;
