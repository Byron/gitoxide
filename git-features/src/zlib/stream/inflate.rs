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

/// Read bytes from `read` and decompress them using `state` into a pre-allocated fitting buffer `dst`.
pub fn read(rd: &mut impl BufRead, state: &mut Decompress, dst: &mut [u8]) -> io::Result<usize> {
    loop {
        let (read, consumed, ret, eof);
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
            read = (state.total_out() - before_out) as usize;
            consumed = (state.total_in() - before_in) as usize;
        }
        rd.consume(consumed);

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
