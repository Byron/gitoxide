const HEADER_READ_COMPRESSED_BYTES: usize = 256;
const HEADER_READ_UNCOMPRESSED_BYTES: usize = 512;

mod db;
mod object;

pub mod io;

pub use db::{Db, Error as DbError};
pub use object::*;
