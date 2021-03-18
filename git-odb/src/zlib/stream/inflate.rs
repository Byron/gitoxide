use flate2::{Decompress, FlushDecompress, Status};
use std::{io, io::BufRead};

/// Provide streaming decompression using the `std::io::Read` trait.
/// If `std::io::BufReader` is used, an allocation for the input buffer will be performed.
pub struct InflateReader<R> {
    pub(crate) inner: R,
    pub(crate) decompressor: Decompress,
}

impl<R> InflateReader<R>
where
    R: io::BufRead,
{
    pub fn from_read(read: R) -> InflateReader<R> {
        // TODO: Performance opportunity - a buf reader that doesn't allocate
        InflateReader {
            decompressor: Decompress::new(true),
            inner: read,
        }
    }

    pub fn reset(&mut self, read: R) {
        self.inner = read;
        self.decompressor.reset(true);
    }
}

impl<R> io::Read for InflateReader<R>
where
    R: BufRead,
{
    fn read(&mut self, into: &mut [u8]) -> io::Result<usize> {
        read(&mut self.inner, &mut self.decompressor, into)
    }
}

/// The boxed variant is faster for what we do (moving the decompressor in and out a lot)
pub struct InflateReaderBoxed<R> {
    pub(crate) inner: R,
    pub(crate) decompressor: Box<Decompress>,
}

impl<R> io::Read for InflateReaderBoxed<R>
where
    R: BufRead,
{
    fn read(&mut self, into: &mut [u8]) -> io::Result<usize> {
        read(&mut self.inner, &mut self.decompressor, into)
    }
}

fn read<R>(obj: &mut R, data: &mut Decompress, dst: &mut [u8]) -> io::Result<usize>
where
    R: BufRead,
{
    loop {
        let (read, consumed, ret, eof);
        {
            let input = obj.fill_buf()?;
            eof = input.is_empty();
            let before_out = data.total_out();
            let before_in = data.total_in();
            let flush = if eof {
                FlushDecompress::Finish
            } else {
                FlushDecompress::None
            };
            ret = data.decompress(input, dst, flush);
            read = (data.total_out() - before_out) as usize;
            consumed = (data.total_in() - before_in) as usize;
        }
        obj.consume(consumed);

        match ret {
            // If we haven't ready any data and we haven't hit EOF yet,
            // then we need to keep asking for more data because if we
            // return that 0 bytes of data have been read then it will
            // be interpreted as EOF.
            Ok(Status::Ok) | Ok(Status::BufError) if read == 0 && !eof && !dst.is_empty() => continue,
            Ok(Status::Ok) | Ok(Status::BufError) | Ok(Status::StreamEnd) => return Ok(read),

            Err(..) => return Err(io::Error::new(io::ErrorKind::InvalidInput, "corrupt deflate stream")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use git_object::bstr::ByteSlice;
    use std::{io::Read, path::PathBuf};

    fn fixture_path(path: &str) -> PathBuf {
        PathBuf::from("tests/fixtures").join(path)
    }

    #[test]
    fn small_file_decompress() -> Result<(), Box<dyn std::error::Error>> {
        let r = InflateReader::from_read(io::BufReader::new(std::fs::File::open(fixture_path(
            "objects/37/d4e6c5c48ba0d245164c4e10d5f41140cab980",
        ))?));
        let mut bytes = r.bytes();
        let content = bytes.by_ref().take(16).collect::<Result<Vec<_>, _>>()?;
        assert_eq!(content.as_slice().as_bstr(), b"blob 9\0hi there\n".as_bstr());
        assert!(bytes.next().is_none());
        Ok(())
    }
}
