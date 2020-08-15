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

pub mod borrowed;
pub use borrowed::Borrowed;

pub mod read;
#[doc(inline)]
pub use read::Reader;

pub mod decode;
pub mod encode;
