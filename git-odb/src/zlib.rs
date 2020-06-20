use miniz_oxide::{
    inflate::core::DecompressorOxide,
    inflate::{
        core::{
            decompress,
            inflate_flags::{
                TINFL_FLAG_HAS_MORE_INPUT, TINFL_FLAG_PARSE_ZLIB_HEADER,
                TINFL_FLAG_USING_NON_WRAPPING_OUTPUT_BUF,
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

pub struct Inflate {
    inner: DecompressorOxide,
    pub is_done: bool,
}

impl Default for Inflate {
    fn default() -> Self {
        Inflate {
            inner: DecompressorOxide::default(),
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
            &mut self.inner,
            input,
            out,
            if parse_header {
                TINFL_FLAG_PARSE_ZLIB_HEADER
            } else {
                0
            } | TINFL_FLAG_HAS_MORE_INPUT
                | TINFL_FLAG_USING_NON_WRAPPING_OUTPUT_BUF,
        );

        use miniz_oxide::inflate::TINFLStatus::*;
        match status {
            Failed | FailedCannotMakeProgress | BadParam | Adler32Mismatch => {
                return Err(Error::Inflate(status))
            }
            HasMoreOutput | NeedsMoreInput => {}
            Done => {
                self.is_done = true;
            }
        };
        Ok((status, in_consumed, out_consumed))
    }
}

pub mod stream {
    use miniz_oxide::{inflate, inflate::stream::InflateState, MZError, MZFlush, MZStatus};
    use quick_error::quick_error;

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

    pub struct Inflate {
        inner: InflateState,
        total_in: u64,
        total_out: u64,
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
        fn decompress(
            &mut self,
            input: &[u8],
            output: &mut [u8],
            flush: MZFlush,
        ) -> Result<Status, Error> {
            let res = inflate::stream::inflate(&mut self.inner, input, output, flush);
            self.total_in += res.bytes_consumed as u64;
            self.total_out += res.bytes_written as u64;

            match res.status {
                Ok(status) => match status {
                    MZStatus::Ok => Ok(Status::Ok),
                    MZStatus::StreamEnd => Ok(Status::StreamEnd),
                    MZStatus::NeedDict => Err(Error::ZLibNeedDict(
                        self.inner.decompressor().adler32().unwrap_or(0),
                    )),
                },
                Err(status) => match status {
                    MZError::Buf => Ok(Status::BufError),
                    _ => Err(Error::Decompression),
                },
            }
        }
    }
}
