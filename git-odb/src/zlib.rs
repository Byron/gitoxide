use miniz_oxide::{
    inflate::core::DecompressorOxide,
    inflate::{
        core::{
            decompress,
            inflate_flags::{
                TINFL_FLAG_HAS_MORE_INPUT, TINFL_FLAG_PARSE_ZLIB_HEADER, TINFL_FLAG_USING_NON_WRAPPING_OUTPUT_BUF,
            },
        },
        TINFLStatus,
    },
};
use quick_error::quick_error;
use std::io::Cursor;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        WriteInflated(err: std::io::Error) {
            display("Could not write all bytes when decompressing content")
            from()
            cause(err)
        }
        Inflate(status: miniz_oxide::inflate::TINFLStatus) {
            display("Could not decode zip stream, status was '{:?}'", status)
        }
    }
}

/// Decompress a few bytes of a zlib stream without allocation
pub struct Inflate {
    state: DecompressorOxide,
    pub is_done: bool,
}

impl Default for Inflate {
    fn default() -> Self {
        Inflate {
            state: DecompressorOxide::default(),
            is_done: false,
        }
    }
}

impl Inflate {
    /// Run the decompressor exactly once. Cannot be run mutliple times
    pub fn once(
        &mut self,
        input: &[u8],
        out: &mut Cursor<&mut [u8]>,
        parse_header: bool,
    ) -> Result<(TINFLStatus, usize, usize), Error> {
        let (status, in_consumed, out_consumed) = decompress(
            &mut self.state,
            input,
            out,
            if parse_header { TINFL_FLAG_PARSE_ZLIB_HEADER } else { 0 }
                | TINFL_FLAG_HAS_MORE_INPUT
                | TINFL_FLAG_USING_NON_WRAPPING_OUTPUT_BUF,
        );

        use miniz_oxide::inflate::TINFLStatus::*;
        match status {
            Failed | FailedCannotMakeProgress | BadParam | Adler32Mismatch => return Err(Error::Inflate(status)),
            HasMoreOutput | NeedsMoreInput => {}
            Done => {
                self.is_done = true;
            }
        };
        Ok((status, in_consumed, out_consumed))
    }
}

pub mod stream {
    use miniz_oxide::{inflate, inflate::stream::InflateState, DataFormat, MZError, MZFlush, MZStatus};
    use quick_error::quick_error;
    use std::io;
    use std::io::BufRead;

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

    struct Inflate {
        state: InflateState,
        total_in: u64,
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

    /// Provide streaming decompression using the `std::io::Read` trait.
    /// If `std::io::BufReader` is used, an allocation for the input buffer will be performed.
    pub struct InflateReader<R> {
        inner: R,
        decompressor: Inflate,
    }

    impl<R> InflateReader<R>
    where
        R: io::Read,
    {
        pub fn new(read: R) -> InflateReader<io::BufReader<R>> {
            // TODO: Performance opportunity - a buf reader that doesn't allocate
            InflateReader {
                decompressor: Inflate::default(),
                inner: io::BufReader::new(read),
            }
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

    /// From Flate2
    /// Possible status results of compressing some data or successfully
    /// decompressing a block of data.
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub enum Status {
        /// Indicates success.
        ///
        /// Means that more input may be needed but isn't available
        /// and/or there's more output to be written but the output buffer is full.
        Ok,

        /// Indicates that forward progress is not possible due to input or output
        /// buffers being empty.
        ///
        /// For compression it means the input buffer needs some more data or the
        /// output buffer needs to be freed up before trying again.
        ///
        /// For decompression this means that more input is needed to continue or
        /// the output buffer isn't large enough to contain the result. The function
        /// can be called again after fixing both.
        BufError,

        /// Indicates that all input has been consumed and all output bytes have
        /// been written. Decompression/compression should not be called again.
        ///
        /// For decompression with zlib streams the adler-32 of the decompressed
        /// data has also been verified.
        StreamEnd,
    }

    impl Inflate {
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
        use std::io::Read;
        use std::path::PathBuf;

        fn fixture_path(path: &str) -> PathBuf {
            PathBuf::from("tests/fixtures").join(path)
        }

        #[test]
        fn small_file_decompress() {
            let r = InflateReader::new(
                std::fs::File::open(fixture_path("objects/37/d4e6c5c48ba0d245164c4e10d5f41140cab980")).unwrap(),
            );
            let mut bytes = r.bytes();
            let content = bytes.by_ref().take(16).collect::<Result<Vec<_>, _>>().unwrap();
            assert_eq!(content.as_slice().as_bstr(), b"blob 9\0hi there\n".as_bstr());
            assert!(bytes.next().is_none());
        }
    }
}
