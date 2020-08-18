#![forbid(unsafe_code)]
use bstr::BStr;

pub mod packet_line;

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct RemoteProgress<'a> {
    #[cfg_attr(feature = "serde1", serde(borrow))]
    pub action: &'a BStr,
    pub percent: Option<u32>,
    pub step: Option<usize>,
    pub max: Option<usize>,
}

#[doc(inline)]
pub use packet_line::Borrowed as PacketLine;

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum Protocol {
    V1,
    V2,
}

pub mod client;

#[doc(inline)]
pub use client::connect;
