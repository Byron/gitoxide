#![forbid(unsafe_code)]
#![deny(rust_2018_idioms)]

/// The version of the way client and server communicate.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
#[allow(missing_docs)]
pub enum Protocol {
    V1 = 1,
    V2 = 2,
}

/// The kind of service to invoke on the client or the server side.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum Service {
    /// The service sending packs from a server to the client. Used for fetching pack data.
    UploadPack,
    /// The service receiving packs produced by the client, who sends a pack to the server.
    ReceivePack,
}

impl Service {
    /// Render this instance as string recognized by the git transport layer.
    pub fn as_str(&self) -> &'static str {
        match self {
            Service::ReceivePack => "git-receive-pack",
            Service::UploadPack => "git-upload-pack",
        }
    }
}

///
pub mod client;

#[doc(inline)]
pub use client::connect;
