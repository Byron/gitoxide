#[cfg(not(feature = "zlib-ng"))]
use super::Status;
#[cfg(feature = "zlib-ng")]
use flate2::{Compress, Compression, FlushCompress, Status};
#[cfg(not(feature = "zlib-ng"))]
use miniz_oxide::{deflate, deflate::core::CompressorOxide, MZError, MZFlush, MZStatus};
use std::io;

#[derive(thiserror::Error, Debug)]
#[cfg(not(feature = "zlib-ng"))]
pub enum Error {
    #[error("Need dictionary")]
    ZLibNeedDict,
    #[error("A compression error occurred: {0:?}")]
    Error(MZError),
}

#[cfg(not(feature = "zlib-ng"))]
pub struct Deflate {
    inner: CompressorOxide,
    total_in: u64,
    total_out: u64,
}

#[cfg(not(feature = "zlib-ng"))]
impl Default for Deflate {
    fn default() -> Self {
        Deflate {
            inner: CompressorOxide::default(),
            total_in: 0,
            total_out: 0,
        }
    }
}

#[cfg(not(feature = "zlib-ng"))]
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
#[cfg(not(feature = "zlib-ng"))]
pub struct DeflateWriter<W> {
    compressor: Deflate,
    inner: W,
    buf: [u8; BUF_SIZE],
}

#[cfg(feature = "zlib-ng")]
pub struct DeflateWriter<W> {
    compressor: Compress,
    inner: W,
    buf: [u8; BUF_SIZE],
}

impl<W> DeflateWriter<W>
where
    W: io::Write,
{
    pub fn new(inner: W) -> DeflateWriter<W> {
        #[cfg(not(feature = "zlib-ng"))]
        return DeflateWriter {
            compressor: Default::default(),
            inner,
            buf: [0; BUF_SIZE],
        };
        #[cfg(feature = "zlib-ng")]
        return DeflateWriter {
            compressor: Compress::new(Compression::fast(), true),
            inner,
            buf: [0; BUF_SIZE],
        };
    }

    pub fn reset(&mut self) {
        #[cfg(not(feature = "zlib-ng"))]
        self.compressor.inner.reset();
        #[cfg(feature = "zlib-ng")]
        self.compressor.reset();
    }

    pub fn into_inner(self) -> W {
        self.inner
    }

    #[cfg(not(feature = "zlib-ng"))]
    fn write_inner(&mut self, mut buf: &[u8], flush: MZFlush) -> io::Result<usize> {
        let total_in_when_start = self.compressor.total_in;
        loop {
            let last_total_in = self.compressor.total_in;
            let last_total_out = self.compressor.total_out;

            let status = self
                .compressor
                .compress(buf, &mut self.buf, flush)
                .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;

            let written = self.compressor.total_out - last_total_out;
            if written > 0 {
                self.inner.write_all(&self.buf[..written as usize])?;
            }

            match status {
                Status::StreamEnd => return Ok((self.compressor.total_in - total_in_when_start) as usize),
                Status::Ok | Status::BufError => {
                    let consumed = self.compressor.total_in - last_total_in;
                    buf = &buf[consumed as usize..];

                    // output buffer still makes progress
                    if self.compressor.total_out > last_total_out {
                        continue;
                    }
                    // input still makes progress
                    if self.compressor.total_in > last_total_in {
                        continue;
                    }
                    // input also makes no progress anymore, need more so leave with what we have
                    return Ok((self.compressor.total_in - total_in_when_start) as usize);
                }
            }
        }
    }

    #[cfg(feature = "zlib-ng")]
    fn write_inner(&mut self, mut buf: &[u8], flush: FlushCompress) -> io::Result<usize> {
        let total_in_when_start = self.compressor.total_in();
        loop {
            let last_total_in = self.compressor.total_in();
            let last_total_out = self.compressor.total_out();

            let status = self
                .compressor
                .compress(buf, &mut self.buf, flush)
                .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;

            let written = self.compressor.total_out() - last_total_out;
            if written > 0 {
                self.inner.write_all(&self.buf[..written as usize])?;
            }

            match status {
                Status::StreamEnd => return Ok((self.compressor.total_in() - total_in_when_start) as usize),
                Status::Ok | Status::BufError => {
                    let consumed = self.compressor.total_in() - last_total_in;
                    buf = &buf[consumed as usize..];

                    // output buffer still makes progress
                    if self.compressor.total_out() > last_total_out {
                        continue;
                    }
                    // input still makes progress
                    if self.compressor.total_in() > last_total_in {
                        continue;
                    }
                    // input also makes no progress anymore, need more so leave with what we have
                    return Ok((self.compressor.total_in() - total_in_when_start) as usize);
                }
            }
        }
    }
}

impl<W: io::Write> io::Write for DeflateWriter<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        #[cfg(not(feature = "zlib-ng"))]
        return self.write_inner(buf, MZFlush::None);
        #[cfg(feature = "zlib-ng")]
        return self.write_inner(buf, FlushCompress::None);
    }

    fn flush(&mut self) -> io::Result<()> {
        #[cfg(not(feature = "zlib-ng"))]
        return self.write_inner(&[], MZFlush::Finish).map(|_| ());
        #[cfg(feature = "zlib-ng")]
        return self.write_inner(&[], FlushCompress::Finish).map(|_| ());
    }
}

#[cfg(test)]
mod tests;
