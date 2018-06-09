use failure::Error;
use miniz_oxide::inflate::core::DecompressorOxide;
use std::io::Cursor;
use miniz_oxide::inflate::core::{decompress,
                                 inflate_flags::{TINFL_FLAG_PARSE_ZLIB_HEADER,
                                                 TINFL_FLAG_USING_NON_WRAPPING_OUTPUT_BUF}};

pub struct State {
    inner: DecompressorOxide,
}

impl Default for State {
    fn default() -> Self {
        State {
            inner: DecompressorOxide::default(),
        }
    }
}

impl State {
    pub fn once(
        &mut self,
        rbuf: &[u8],
        out: &mut Cursor<&mut [u8]>,
    ) -> Result<(usize, usize), Error> {
        let (status, read_in, read_out) = decompress(
            &mut self.inner,
            rbuf,
            out,
            TINFL_FLAG_PARSE_ZLIB_HEADER | TINFL_FLAG_USING_NON_WRAPPING_OUTPUT_BUF,
        );

        use miniz_oxide::inflate::TINFLStatus::*;
        match status {
            Failed | FailedCannotMakeProgress | BadParam | Adler32Mismatch | NeedsMoreInput => {
                bail!(
                    "Could not decode zip stream for reading header, status was '{:?}'",
                    status
                )
            }
            Done | HasMoreOutput => {}
        };
        Ok((read_in, read_out))
    }
}
