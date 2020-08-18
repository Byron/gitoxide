#![forbid(unsafe_code)]

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum Protocol {
    V1,
    V2,
}

pub mod client;

#[doc(inline)]
pub use client::connect;
