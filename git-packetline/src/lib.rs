//! Read and write the git packet line wire format without copying it.
//!
//! For reading the packet line format use the [`StreamingPeekableIter`], and for writing the `Writer`.
#![deny(unsafe_code, rust_2018_idioms, missing_docs)]

const U16_HEX_BYTES: usize = 4;
const MAX_DATA_LEN: usize = 65516;
const MAX_LINE_LEN: usize = MAX_DATA_LEN + U16_HEX_BYTES;
const FLUSH_LINE: &[u8] = b"0000";
const DELIMITER_LINE: &[u8] = b"0001";
const RESPONSE_END_LINE: &[u8] = b"0002";
const ERR_PREFIX: &[u8] = b"ERR ";

/// One of three side-band types allowing to multiplex information over a single connection.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum Channel {
    /// The usable data itself in any format.
    Data = 1,
    /// Progress information in a user-readable format.
    Progress = 2,
    /// Error information in a user readable format. Receiving it usually terminates the connection.
    Error = 3,
}

///
pub mod line;

/// A borrowed packet line as it refers to a slice of data by reference.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum PacketLineRef<'a> {
    /// A chunk of raw data.
    Data(&'a [u8]),
    /// A flush packet.
    Flush,
    /// A delimiter packet.
    Delimiter,
    /// The end of the response.
    ResponseEnd,
}

/// A packet line representing an Error in a side-band channel.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct ErrorRef<'a>(pub &'a [u8]);

/// A packet line representing text, which may include a trailing newline.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct TextRef<'a>(pub &'a [u8]);

/// A band in a side-band channel.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum BandRef<'a> {
    /// A band carrying data.
    Data(&'a [u8]),
    /// A band carrying user readable progress information.
    Progress(&'a [u8]),
    /// A band carrying user readable errors.
    Error(&'a [u8]),
}

///
pub mod read;
#[doc(inline)]
pub use read::StreamingPeekableIter;

///
#[cfg(any(feature = "async-io", feature = "blocking-io"))]
pub mod write;
#[cfg(any(feature = "async-io", feature = "blocking-io"))]
#[doc(inline)]
pub use write::Writer;

/// Utilities to help decoding packet lines
pub mod decode;
#[doc(inline)]
pub use decode::all_at_once as decode;
/// Utilities to encode different kinds of packet lines
pub mod encode;

#[cfg(all(feature = "async-io", feature = "blocking-io"))]
compile_error!("Cannot set both 'blocking-io' and 'async-io' features as they are mutually exclusive");
