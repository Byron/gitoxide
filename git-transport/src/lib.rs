#![forbid(unsafe_code)]

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum Protocol {
    V1,
    V2,
}

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum Service {
    /// The service sending packs from a server to the client. Used for fetching pack data.
    UploadPack,
    /// The service receiving packs produced by the client, who sends a pack to the server.
    ReceivePack,
}

pub mod client;

#[doc(inline)]
pub use client::connect;
