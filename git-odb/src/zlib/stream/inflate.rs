use flate2::{Decompress, FlushDecompress, Status};
use std::{io, io::BufRead};

/// The boxed variant is faster for what we do (moving the decompressor in and out a lot)
pub struct ReadBoxed<R> {
    pub(crate) inner: R,
    pub(crate) decompressor: Box<Decompress>,
}

impl<R> io::Read for ReadBoxed<R>
where
    R: BufRead,
{
    fn read(&mut self, into: &mut [u8]) -> io::Result<usize> {
        read(&mut self.inner, &mut self.decompressor, into)
    }
}

pub(crate) fn read<R: BufRead>(obj: &mut R, data: &mut Decompress, dst: &mut [u8]) -> io::Result<usize> {
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
