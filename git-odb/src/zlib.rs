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
