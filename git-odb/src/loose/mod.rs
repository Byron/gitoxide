const HEADER_READ_COMPRESSED_BYTES: usize = 256;
const HEADER_READ_UNCOMPRESSED_BYTES: usize = 512;

mod db;
mod object;

pub mod io {
    use crate::zlib::stream::InflateReader;
    use std::io::BufReader;

    pub enum ObjectReader {
        File(usize, InflateReader<BufReader<std::fs::File>>),
    }

    impl ObjectReader {
        pub fn from_read(header_size: usize, file: std::fs::File) -> ObjectReader {
            ObjectReader::File(header_size, InflateReader::new(file))
        }
    }

    impl std::io::Read for ObjectReader {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            match self {
                ObjectReader::File(header_size_left, r) => {
                    if *header_size_left == 0 {
                        r.read(buf)
                    } else {
                        // We must assure we return at least one byte - otherwise it's considered EOF, thus '>='
                        while *header_size_left != 0 {
                            let bytes_to_read = buf.len().min(*header_size_left);
                            r.read_exact(&mut buf[..bytes_to_read])?;
                            *header_size_left -= bytes_to_read;
                        }
                        r.read(buf)
                    }
                }
            }
        }
    }
}

pub use db::{Db, Error as DbError};
pub use object::*;
