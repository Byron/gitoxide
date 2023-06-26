//! Worktree encodings are powered by the `encoding_rs` crate, which has a narrower focus than the `iconv` library. Thus this implementation
//! is inherently more limited but will handle the common cases.
//!  
//! Note that for encoding to legacy formats, [additional normalization steps](https://docs.rs/encoding_rs/0.8.32/encoding_rs/#preparing-text-for-the-encoders)
//! can be taken, which we do not yet take unless there is specific examples or problems to solve.

use crate::clear_and_set_capacity;
use crate::worktree::encode_to_git::RoundTrip;
use encoding_rs::DecoderResult;

///
pub mod encoding {
    use bstr::BStr;
    use encoding_rs::Encoding;

    ///
    pub mod for_label {
        use bstr::BString;

        /// The error returned by [for_label()][super::for_label()].
        #[derive(Debug, thiserror::Error)]
        #[allow(missing_docs)]
        pub enum Error {
            #[error("An encoding named '{name}' is not known")]
            Unknown { name: BString },
        }
    }
    /// Try to produce a new `Encoding` for `label` or report an error if it is not known.
    ///
    /// ### Deviation
    ///
    /// * There is no special handling of UTF-16LE/BE with checks if data contains a BOM or not, like `git` as we don't expect to have
    ///   data available here.
    /// * Special `-BOM` suffixed versions of `UTF-16` encodings are not supported.
    pub fn for_label<'a>(label: impl Into<&'a BStr>) -> Result<&'static Encoding, for_label::Error> {
        let mut label = label.into();
        if label == "latin-1" {
            label = "ISO-8859-1".into();
        }
        let enc =
            Encoding::for_label(label.as_ref()).ok_or_else(|| for_label::Error::Unknown { name: label.into() })?;
        Ok(enc)
    }
}

///
pub mod encode_to_git {
    /// Whether or not to perform round-trip checks.
    #[derive(Debug, Copy, Clone)]
    pub enum RoundTrip {
        /// Assure that we can losslessly convert the UTF-8 result back to the original encoding.
        Validate,
        /// Do not check if the encoding is round-trippable.
        Ignore,
    }

    /// The error returned by [`encode_to_git()][super::encode_to_git()].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error("Cannot convert input of {input_len} bytes to UTF-8 without overflowing")]
        Overflow { input_len: usize },
        #[error("The input was malformed and could not be decoded as '{encoding}'")]
        Malformed { encoding: &'static str },
        #[error("Encoding from '{src_encoding}' to '{dest_encoding}' and back is not the same")]
        RoundTrip {
            src_encoding: &'static str,
            dest_encoding: &'static str,
        },
    }
}

/// Decode `src` according to `src_encoding` to `UTF-8` for storage in git.
/// Note that the encoding is always applied, there is no conditional even if `src_encoding` already is `UTF-8`.
pub fn encode_to_git(
    src: &[u8],
    src_encoding: &'static encoding_rs::Encoding,
    buf: &mut Vec<u8>,
    round_trip: encode_to_git::RoundTrip,
) -> Result<(), encode_to_git::Error> {
    let mut decoder = src_encoding.new_decoder_with_bom_removal();
    let buf_len = decoder
        .max_utf8_buffer_length_without_replacement(src.len())
        .ok_or_else(|| encode_to_git::Error::Overflow { input_len: src.len() })?;
    clear_and_set_capacity(buf, buf_len);
    // SAFETY: `clear_and_set_capacity` assure that we have the given `buf_len` allocated, so setting its length is only making available
    //          what is allocated. Later we will truncate to the amount of actually written bytes.
    #[allow(unsafe_code)]
    unsafe {
        buf.set_len(buf_len);
    }
    let (res, read, written) = decoder.decode_to_utf8_without_replacement(src, buf, true);
    match res {
        DecoderResult::InputEmpty => {
            assert!(
                buf_len >= written,
                "encoding_rs estimates the maximum amount of bytes written correctly"
            );
            assert_eq!(read, src.len(), "input buffer should be fully consumed");
            // SAFETY: we trust that `encoding_rs` reports this number correctly, and truncate everything else.
            #[allow(unsafe_code)]
            unsafe {
                buf.set_len(written);
            }
        }
        DecoderResult::OutputFull => {
            unreachable!("we assure that the output buffer is big enough as per the encoder's estimate")
        }
        DecoderResult::Malformed(_, _) => {
            return Err(encode_to_git::Error::Malformed {
                encoding: src_encoding.name(),
            })
        }
    }

    match round_trip {
        RoundTrip::Validate => {
            // SAFETY: we trust `encoding_rs` to output valid UTF-8 only if we ask it to.
            #[allow(unsafe_code)]
            let str = unsafe { std::str::from_utf8_unchecked(&buf) };
            let (should_equal_src, _actual_encoding, _had_errors) = src_encoding.encode(str);
            if should_equal_src != src {
                return Err(encode_to_git::Error::RoundTrip {
                    src_encoding: src_encoding.name(),
                    dest_encoding: "UTF-8",
                });
            }
        }
        RoundTrip::Ignore => {}
    }
    Ok(())
}
