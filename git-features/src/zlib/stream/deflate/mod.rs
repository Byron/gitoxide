use flate2::Compress;

const BUF_SIZE: usize = 4096 * 8;

/// A utility to zlib compress anything that is written via its [Write][std::io::Write] implementation.
///
/// Be sure to call `flush()` when done to finalize the deflate stream.
pub struct Write<W> {
    compressor: Compress,
    inner: W,
    buf: [u8; BUF_SIZE],
}

mod impls {
    use std::io;

    use flate2::{Compress, Compression, FlushCompress, Status};

    use crate::zlib::stream::deflate;

    impl<W> deflate::Write<W>
    where
        W: io::Write,
    {
        /// Create a new instance writing compressed bytes to `inner`.
        pub fn new(inner: W) -> deflate::Write<W> {
            deflate::Write {
                compressor: Compress::new(Compression::fast(), true),
                inner,
                buf: [0; deflate::BUF_SIZE],
            }
        }

        /// Reset the compressor, starting a new compression stream.
        ///
        /// That way multiple streams can be written to the same inner writer.
        pub fn reset(&mut self) {
            self.compressor.reset();
        }

        /// Consume `self` and return the inner writer.
        pub fn into_inner(self) -> W {
            self.inner
        }

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

    impl<W: io::Write> io::Write for deflate::Write<W> {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.write_inner(buf, FlushCompress::None)
        }

        fn flush(&mut self) -> io::Result<()> {
            self.write_inner(&[], FlushCompress::Finish).map(|_| ())
        }
    }
}

#[cfg(test)]
mod tests;
