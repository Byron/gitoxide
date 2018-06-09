use failure::Error;
use miniz_oxide::inflate::core::DecompressorOxide;
use std::io::Cursor;
use miniz_oxide::inflate::{TINFLStatus,
                           core::{decompress,
                                  inflate_flags::{TINFL_FLAG_PARSE_ZLIB_HEADER,
                                                  TINFL_FLAG_USING_NON_WRAPPING_OUTPUT_BUF}}};
use miniz_oxide::inflate::core::inflate_flags;

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
    pub fn once(
        &mut self,
        rbuf: &[u8],
        out: &mut Cursor<&mut [u8]>,
    ) -> Result<(usize, usize), Error> {
        let (status, in_consumed, out_consumed) = decompress(
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
            HasMoreOutput => {}
            Done => {
                self.is_done = true;
            }
        };
        Ok((in_consumed, out_consumed))
    }
}

fn decompress_to_vec_inner(input: &[u8], flags: u32) -> Result<Vec<u8>, TINFLStatus> {
    let flags = flags | inflate_flags::TINFL_FLAG_USING_NON_WRAPPING_OUTPUT_BUF;
    let mut ret = Vec::with_capacity(input.len() * 2);

    // # Unsafe
    // We trust decompress to not read the unitialized bytes as it's wrapped
    // in a cursor that's position is set to the end of the initialized data.
    unsafe {
        let cap = ret.capacity();
        ret.set_len(cap);
    };
    let mut decomp = unsafe { DecompressorOxide::with_init_state_only() };

    let mut in_pos = 0;
    let mut out_pos = 0;
    loop {
        let (status, in_consumed, out_consumed) = {
            // Wrap the whole output slice so we know we have enough of the
            // decompressed data for matches.
            let mut c = Cursor::new(ret.as_mut_slice());
            c.set_position(out_pos as u64);
            decompress(&mut decomp, &input[in_pos..], &mut c, flags)
        };
        in_pos += in_consumed;
        out_pos += out_consumed;

        match status {
            TINFLStatus::Done => {
                ret.truncate(out_pos);
                return Ok(ret);
            }

            TINFLStatus::HasMoreOutput => {
                // We need more space so extend the buffer.
                ret.reserve(out_pos);
                // # Unsafe
                // We trust decompress to not read the unitialized bytes as it's wrapped
                // in a cursor that's position is set to the end of the initialized data.
                unsafe {
                    let cap = ret.capacity();
                    ret.set_len(cap);
                }
            }

            _ => return Err(status),
        }
    }
}
