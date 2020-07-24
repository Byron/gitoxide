use super::Status;
use miniz_oxide::{deflate, deflate::core::CompressorOxide, MZError, MZFlush, MZStatus};
use quick_error::quick_error;
use std::io;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Compression {
            display("The compression failed due to an unknown error")
        }
        ZLibNeedDict {
            display("Need dictionary")
        }
        Error(err: MZError) {
            display("A compression error occurred: {:?}", err)
        }
    }
}

pub struct Deflate {
    inner: CompressorOxide,
    total_in: u64,
    total_out: u64,
}

impl Default for Deflate {
    fn default() -> Self {
        Deflate {
            inner: CompressorOxide::default(),
            total_in: 0,
            total_out: 0,
        }
    }
}

impl Deflate {
    fn compress(&mut self, input: &[u8], output: &mut [u8], flush: MZFlush) -> Result<Status, Error> {
        let res = deflate::stream::deflate(&mut self.inner, input, output, flush);
        self.total_in += res.bytes_consumed as u64;
        self.total_out += res.bytes_written as u64;

        match res.status {
            Ok(status) => match status {
                MZStatus::Ok => Ok(Status::Ok),
                MZStatus::StreamEnd => Ok(Status::StreamEnd),
                MZStatus::NeedDict => Err(Error::ZLibNeedDict),
            },
            Err(status) => match status {
                MZError::Buf => Ok(Status::BufError),
                _ => Err(Error::Error(status)),
            },
        }
    }
}

const BUF_SIZE: usize = 4096 * 8;
pub struct DeflateStream<W> {
    compressor: Deflate,
    inner: W,
    buf: [u8; BUF_SIZE],
}

impl<W> DeflateStream<W>
where
    W: io::Write,
{
    pub fn new(inner: W) -> DeflateStream<W> {
        DeflateStream {
            compressor: Default::default(),
            inner,
            buf: [0; BUF_SIZE],
        }
    }
}

impl<W> io::Write for DeflateStream<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        unimplemented!()
    }

    fn flush(&mut self) -> io::Result<()> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests;
