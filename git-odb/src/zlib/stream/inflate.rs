use super::Status;
use miniz_oxide::{inflate, inflate::stream::InflateState, DataFormat, MZError, MZFlush, MZStatus};
use quick_error::quick_error;
use std::{io, io::BufRead};

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Decompression {
            display("The decompression failed due to an unknown error")
        }
        ZLibNeedDict(adler: u32) {
            display("Probably the stream is damaged, adler value is {}", adler)
        }
    }
}

pub(crate) struct Inflate {
    state: InflateState,
    pub(crate) total_in: u64,
    total_out: u64,
}

impl Default for Inflate {
    fn default() -> Self {
        Inflate {
            state: InflateState::new(DataFormat::Zlib),
            total_in: 0,
            total_out: 0,
        }
    }
}

impl Inflate {
    pub fn reset(&mut self) {
        self.state.reset(DataFormat::Zlib);
        self.total_in = 0;
        self.total_out = 0;
    }

    fn decompress(&mut self, input: &[u8], output: &mut [u8], flush: MZFlush) -> Result<Status, Error> {
        let res = inflate::stream::inflate(&mut self.state, input, output, flush);
        self.total_in += res.bytes_consumed as u64;
        self.total_out += res.bytes_written as u64;

        match res.status {
            Ok(status) => match status {
                MZStatus::Ok => Ok(Status::Ok),
                MZStatus::StreamEnd => Ok(Status::StreamEnd),
                MZStatus::NeedDict => Err(Error::ZLibNeedDict(self.state.decompressor().adler32().unwrap_or(0))),
            },
            Err(status) => match status {
                MZError::Buf => Ok(Status::BufError),
                _ => Err(Error::Decompression),
            },
        }
    }
}

/// Provide streaming decompression using the `std::io::Read` trait.
/// If `std::io::BufReader` is used, an allocation for the input buffer will be performed.
pub struct InflateReader<R> {
    pub(crate) inner: R,
    pub(crate) decompressor: Inflate,
}

impl<R> InflateReader<R>
where
    R: io::BufRead,
{
    pub fn from_read(read: R) -> InflateReader<R> {
        // TODO: Performance opportunity - a buf reader that doesn't allocate
        InflateReader {
            decompressor: Inflate::default(),
            inner: read,
        }
    }

    pub fn reset(&mut self, read: R) {
        self.inner = read;
        self.decompressor.reset();
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

/// Adapted from [flate2](https://github.com/alexcrichton/flate2-rs/blob/57972d77dab09acad4aa2fa3beedb1f69fa64b27/src/zio.rs#L118)
fn read<R>(obj: &mut R, data: &mut Inflate, dst: &mut [u8]) -> io::Result<usize>
where
    R: BufRead,
{
    loop {
        let (read, consumed, ret, eof);
        {
            let input = obj.fill_buf()?;
            eof = input.is_empty();
            let before_out = data.total_out;
            let before_in = data.total_in;
            let flush = if eof { MZFlush::Finish } else { MZFlush::None };
            ret = data.decompress(input, dst, flush);
            read = (data.total_out - before_out) as usize;
            consumed = (data.total_in - before_in) as usize;
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
