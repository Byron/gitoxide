use flate2::{Decompress, FlushDecompress, Status};
use std::{io, io::BufRead};

/// The boxed variant is faster for what we do (moving the decompressor in and out a lot)
pub struct ReadBoxed<R> {
    /// The reader from which bytes should be decompressed.
    pub inner: R,
    /// The decompressor doing all the work.
    pub decompressor: Box<Decompress>,
}

impl<R> io::Read for ReadBoxed<R>
where
    R: BufRead,
{
    fn read(&mut self, into: &mut [u8]) -> io::Result<usize> {
        read(&mut self.inner, &mut self.decompressor, into)
    }
}

/// Read bytes from `rd` and decompress them using `state` into a pre-allocated fitting buffer `dst`, returning the amount of bytes written.
pub fn read(rd: &mut impl BufRead, state: &mut Decompress, mut dst: &mut [u8]) -> io::Result<usize> {
    loop {
        let (written, consumed, ret, eof);
        {
            let input = rd.fill_buf()?;
            eof = input.is_empty();
            let before_out = state.total_out();
            let before_in = state.total_in();
            let flush = if eof {
                FlushDecompress::Finish
            } else {
                FlushDecompress::None
            };
            ret = state.decompress(input, dst, flush);
            written = (state.total_out() - before_out) as usize;
            dst = &mut dst[written..];
            consumed = (state.total_in() - before_in) as usize;
        }
        // dbg!(&ret, written, consumed, dst.len());
        rd.consume(consumed);

        match ret {
            Ok(Status::StreamEnd) => return Ok(written),
            Ok(Status::Ok) | Ok(Status::BufError) if eof || dst.is_empty() => return Ok(written),
            Ok(Status::Ok) | Ok(Status::BufError) if written == 0 && !eof && !dst.is_empty() => continue,
            Ok(Status::Ok) | Ok(Status::BufError) => return Ok(written),
            Err(..) => return Err(io::Error::new(io::ErrorKind::InvalidInput, "corrupt deflate stream")),
        }
    }
}
