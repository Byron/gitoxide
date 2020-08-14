use bstr::BStr;
use std::io;

pub(crate) const U16_HEX_BYTES: usize = 4;
pub(crate) const MAX_DATA_LEN: usize = 65516;
pub(crate) const MAX_LINE_LEN: usize = MAX_DATA_LEN + U16_HEX_BYTES;
pub(crate) const FLUSH_LINE: &[u8] = b"0000";
pub(crate) const ERR_PREFIX: &[u8] = b"ERR ";

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum Borrowed<'a> {
    Data(&'a [u8]),
    Flush,
}

impl<'a> Borrowed<'a> {
    pub fn to_write(&self, out: impl io::Write) -> Result<usize, encode::Error> {
        match self {
            Borrowed::Flush => encode::flush_to_write(out).map_err(Into::into),
            Borrowed::Data(d) => encode::data_to_write(d, out),
        }
    }

    pub fn as_slice(&self) -> &[u8] {
        match self {
            Borrowed::Data(d) => d,
            Borrowed::Flush => &[],
        }
    }
    pub fn as_bstr(&self) -> &BStr {
        self.as_slice().into()
    }
}

pub mod decode;
pub mod encode;
