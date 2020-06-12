use miniz_oxide::inflate::core::DecompressorOxide;
use miniz_oxide::inflate::{
    core::{
        decompress,
        inflate_flags::{
            TINFL_FLAG_HAS_MORE_INPUT, TINFL_FLAG_PARSE_ZLIB_HEADER,
            TINFL_FLAG_USING_NON_WRAPPING_OUTPUT_BUF,
        },
    },
    TINFLStatus,
};
use std::io::{self, Cursor};

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
    pub fn all_till_done(
        &mut self,
        input: &[u8],
        mut out: impl io::Write,
    ) -> Result<(usize, usize), Error> {
        let mut buf = [0; 8192]; // as per git itself
        let mut in_pos = 0;
        let mut out_pos = 0;
        loop {
            let (status, in_consumed, out_consumed) = {
                let mut c = Cursor::new(&mut buf[..]);
                self.once(&input[in_pos..], &mut c)?
            };
            out.write_all(&buf[..out_consumed])?;
            in_pos += in_consumed;
            out_pos += out_consumed;

            match status {
                TINFLStatus::Done => {
                    return Ok((in_pos, out_pos));
                }

                TINFLStatus::HasMoreOutput => {
                    // just try again with fresh cursor
                }
                _ => unreachable!(
                    "This should all be covered by once, we expect a complete input buffer: {:?}",
                    status
                ),
            }
        }
    }

    pub fn once(
        &mut self,
        input: &[u8],
        out: &mut Cursor<&mut [u8]>,
    ) -> Result<(TINFLStatus, usize, usize), Error> {
        let (status, in_consumed, out_consumed) = decompress(
            &mut self.inner,
            input,
            out,
            TINFL_FLAG_PARSE_ZLIB_HEADER
                | TINFL_FLAG_HAS_MORE_INPUT
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
