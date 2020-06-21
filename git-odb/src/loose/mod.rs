const HEADER_READ_COMPRESSED_BYTES: usize = 256;
const HEADER_READ_UNCOMPRESSED_BYTES: usize = 512;

mod db;
mod object;

pub mod io {
    use crate::zlib::stream::InflateReader;
    use std::io::{BufReader, Bytes, Read};
    use std::iter::Skip;

    pub enum ObjectReader {
        File(InflateReader<BufReader<std::fs::File>>),
    }

    impl ObjectReader {
        pub fn from_read(header_size: usize, file: std::fs::File) -> ObjectReader {
            ObjectReader::File(InflateReader::new(file))
        }
    }

    impl std::io::Read for ObjectReader {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            match self {
                ObjectReader::File(r) => r.read(buf),
            }
        }
    }
}

pub use db::{Db, Error as DbError};
pub use object::*;
