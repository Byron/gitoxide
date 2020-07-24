use miniz_oxide::{
    inflate::core::DecompressorOxide,
    inflate::{
        core::inflate_flags::{
            TINFL_FLAG_HAS_MORE_INPUT, TINFL_FLAG_PARSE_ZLIB_HEADER, TINFL_FLAG_USING_NON_WRAPPING_OUTPUT_BUF,
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
            source(err)
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
        let (status, in_consumed, out_consumed) = miniz_oxide::inflate::core::decompress(
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

pub mod stream;
