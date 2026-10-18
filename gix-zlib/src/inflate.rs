use gix_error::Result;
use gix_error::{ResultExt, message};

#[cfg(not(feature = "sirocco"))]
use crate::FlushDecompress;
use crate::{Inflate, Status};

impl Inflate {
    /// Decompress the zlib stream at the start of `input` into `out`, returning `(status, consumed_in, written_out)`.
    ///
    /// If `out` can hold the whole decompressed stream, the status is [`Status::StreamEnd`]. Otherwise this behaves
    /// like [`Self::prefix()`].
    /// Cannot be run multiple times without [resetting](Self::reset()) in between.
    pub fn once(&mut self, input: &[u8], out: &mut [u8]) -> Result<(Status, usize, usize)> {
        self.prefix(input, out)
    }

    /// Decompress as much of the zlib stream at the start of `input` as fits into `out` in a single step,
    /// returning `(status, consumed_in, written_out)`.
    ///
    /// The decompression can be continued with [`Self::rest()`].
    pub fn prefix(&mut self, input: &[u8], out: &mut [u8]) -> Result<(Status, usize, usize)> {
        #[cfg(feature = "sirocco")]
        {
            let decoder = self.decoder();
            decoder.reset();
            let p = decoder
                .decompress(input, out, true)
                .map_err(sirocco_error)
                .or_raise(|| message("Could not decode zip stream"))?;
            let status = if p.done { Status::StreamEnd } else { Status::Ok };
            Ok((status, p.consumed, p.output.len()))
        }
        #[cfg(not(feature = "sirocco"))]
        {
            let before_in = self.state.total_in();
            let before_out = self.state.total_out();
            let status = self
                .state
                .decompress(input, out, FlushDecompress::None)
                .or_raise(|| message("Could not decode zip stream"))?;
            Ok((
                status,
                (self.state.total_in() - before_in) as usize,
                (self.state.total_out() - before_out) as usize,
            ))
        }
    }

    /// Decompress the rest of the stream that [`Self::prefix()`] started, from `input` (what follows the input it
    /// consumed) until the stream ends or `len` bytes are written.
    ///
    /// `prefix` is what `prefix()` wrote. `out` receives the output after its first `skip` bytes, which includes
    /// `prefix[skip..]`, and is cleared first. Returns how many bytes were written to `out`.
    pub fn rest(&mut self, input: &[u8], prefix: &[u8], skip: usize, out: &mut Vec<u8>, len: usize) -> Result<usize> {
        out.clear();
        #[cfg(feature = "sirocco")]
        {
            // The decoder needs the output so far in front of where it writes.
            out.resize(skip + len, 0);
            out[..prefix.len()].copy_from_slice(prefix);
            let decoder = self.decoder();
            let written = decoder
                .decompress(input, out, true)
                .map_err(sirocco_error)
                .or_raise(|| message("Could not decode zip stream"))?
                .output
                .len();
            let n = prefix.len() + written - skip;
            out.drain(..skip);
            out.truncate(n);
            Ok(n)
        }
        #[cfg(not(feature = "sirocco"))]
        {
            out.resize(len, 0);
            let held = prefix.len() - skip;
            out[..held].copy_from_slice(&prefix[skip..]);
            let mut input = input;
            let n = crate::stream::inflate::read(&mut input, &mut self.state, &mut out[held..])
                .or_raise(|| message("Could not decode zip stream"))?;
            out.truncate(held + n);
            Ok(held + n)
        }
    }

    /// Ready this instance for decoding another data stream.
    pub fn reset(&mut self) {
        self.state.reset();
        #[cfg(feature = "sirocco")]
        if let Some(decoder) = &mut self.sirocco {
            decoder.reset();
        }
    }

    #[cfg(feature = "sirocco")]
    fn decoder(&mut self) -> &mut sirocco::inflate::Decoder {
        self.sirocco
            .get_or_insert_with(|| Box::new(sirocco::inflate::Decoder::new(sirocco::Format::Zlib)))
    }
}

/// The error `zlib-rs` would report for a `sirocco` one.
#[cfg(feature = "sirocco")]
fn sirocco_error(err: sirocco::inflate::Error) -> gix_error::Error {
    use gix_error::ErrorExt;
    match err {
        sirocco::inflate::Error::PresetDictionary => {
            gix_error::validation("Decompressing this input requires a dictionary")
                .raise_erased()
                .into()
        }
        sirocco::inflate::Error::BadData | sirocco::inflate::Error::ChecksumMismatch => {
            gix_error::corruption("Invalid input data").raise_erased().into()
        }
        sirocco::inflate::Error::Truncated => gix_error::corruption("Truncated input data").raise_erased().into(),
        _ => message("stream error").raise_erased().into(),
    }
}
