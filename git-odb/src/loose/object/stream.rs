use crate::zlib::stream::InflateReader;
use std::io::BufReader;

/// Either a file-based reader, or a decompressed buffer as the result of a read operation from the filesystem
pub enum Reader<'a> {
    /// A file based reader with `(header size, decompressing reader)`
    File(usize, InflateReader<BufReader<std::fs::File>>),
    /// A reader providing bytes from a slice.
    Buffer(&'a [u8]),
}

/// A [`Read`][std::io::Read] implementation for reading from a file or from borrowed data.
impl<'a> Reader<'a> {
    /// Instantiate a `Reader` from a `file`, skipping `header_size` bytes when it's read.
    pub fn from_file(header_size: usize, file: std::fs::File) -> Reader<'a> {
        Reader::File(header_size, InflateReader::from_read(std::io::BufReader::new(file)))
    }

    /// Instantiate a `Reader` from decompressed `data`, ignoring the first `header_size` bytes of it.
    pub fn from_data(header_size: usize, data: &'a [u8]) -> Reader<'a> {
        Reader::Buffer(&data[header_size..])
    }
}

impl<'a> std::io::Read for Reader<'a> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            Reader::Buffer(data) => data.read(buf),
            Reader::File(header_size_left, r) => {
                if *header_size_left == 0 {
                    r.read(buf)
                } else {
                    // We must assure we return at least one byte - otherwise it's considered EOF
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
