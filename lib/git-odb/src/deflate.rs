use failure::Error;
use miniz_oxide::inflate::core::DecompressorOxide;
use std::io::Cursor;
use miniz_oxide::inflate::{TINFLStatus,
                           core::{decompress,
                                  inflate_flags::{TINFL_FLAG_HAS_MORE_INPUT,
                                                  TINFL_FLAG_USING_NON_WRAPPING_OUTPUT_BUF}}};
use std::io;

pub struct State {
    inner: DecompressorOxide,
    pub is_done: bool,
}

impl Default for State {
    fn default() -> Self {
        State {
            inner: DecompressorOxide::default(),
            is_done: false,
        }
    }
}

impl State {
    pub fn to_end(
        &mut self,
        input: &[u8],
        mut out: impl io::Write,
        flags: u32,
    ) -> Result<(usize, usize), Error> {
        let mut buf = [0; 1024];
        let mut in_pos = 0;
        let mut out_pos = 0;
        loop {
            //            let (status, in_consumed, out_consumed) = {
            //                let mut c = Cursor::new(&mut buf[..]);
            //                decompress(
            //                    &mut self.inner,
            //                    &input[in_pos..],
            //                    &mut c,
            //                    flags | TINFL_FLAG_USING_NON_WRAPPING_OUTPUT_BUF
            //
            //                )
            //            };
            let (status, in_consumed, out_consumed) = {
                let mut c = Cursor::new(&mut buf[..]);
                self.once(&input[in_pos..], &mut c, flags)?
            };
            println!(
                "in {} + {} of {}\nout {} + {}",
                in_pos,
                in_consumed,
                input.len(),
                out_pos,
                out_consumed
            );
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
                TINFLStatus::NeedsMoreInput | _ => unreachable!(
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
        flags: u32,
    ) -> Result<(TINFLStatus, usize, usize), Error> {
        let (status, in_consumed, out_consumed) = decompress(
            &mut self.inner,
            input,
            out,
            flags | TINFL_FLAG_HAS_MORE_INPUT | TINFL_FLAG_USING_NON_WRAPPING_OUTPUT_BUF,
        );

        use miniz_oxide::inflate::TINFLStatus::*;
        match status {
            Failed | FailedCannotMakeProgress | BadParam | Adler32Mismatch => {
                bail!("Could not decode zip stream, status was '{:?}'", status)
            }
            HasMoreOutput | NeedsMoreInput => {}
            Done => {
                self.is_done = true;
            }
        };
        Ok((status, in_consumed, out_consumed))
    }
}
