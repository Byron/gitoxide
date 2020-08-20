#![forbid(unsafe_code)]
pub(crate) const U16_HEX_BYTES: usize = 4;
pub(crate) const MAX_DATA_LEN: usize = 65516;
pub(crate) const MAX_LINE_LEN: usize = MAX_DATA_LEN + U16_HEX_BYTES;
pub(crate) const FLUSH_LINE: &[u8] = b"0000";
pub(crate) const DELIMITER_LINE: &[u8] = b"0001";
pub(crate) const RESPONSE_END_LINE: &[u8] = b"0002";
pub(crate) const ERR_PREFIX: &[u8] = b"ERR ";

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum Channel {
    Data = 1,
    Progress = 2,
    Error = 3,
}

/// The information usually found in remote progress messages as sent by a git server during
/// fetch, clone and push.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct RemoteProgress<'a> {
    #[cfg_attr(feature = "serde1", serde(borrow))]
    pub action: &'a bstr::BStr,
    pub percent: Option<u32>,
    pub step: Option<usize>,
    pub max: Option<usize>,
}

pub mod borrowed;
pub use borrowed::Borrowed as PacketLine;

pub mod read;
#[doc(inline)]
pub use read::Reader;

pub mod write;
pub use write::Writer;

pub mod decode;
pub use decode::all_at_once as decode;
pub mod encode;
