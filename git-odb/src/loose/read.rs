use crate::zlib::stream::InflateReader;
use std::io::BufReader;

pub enum ObjectReader<'data> {
    File(usize, InflateReader<BufReader<std::fs::File>>),
    Buffer(&'data [u8]),
}

impl<'data> ObjectReader<'data> {
    pub fn from_read(header_size: usize, file: std::fs::File) -> ObjectReader<'data> {
        ObjectReader::File(header_size, InflateReader::new(file))
    }
    pub fn from_data(header_size: usize, data: &'data [u8]) -> ObjectReader<'data> {
        ObjectReader::Buffer(&data[header_size..])
    }
}

impl<'data> std::io::Read for ObjectReader<'data> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            ObjectReader::Buffer(data) => data.read(buf),
            ObjectReader::File(header_size_left, r) => {
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
