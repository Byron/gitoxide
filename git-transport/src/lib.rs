//! An implementation of the `git` transport layer, abstracting over all of its [versions][Protocol], providing
//! [`connect()`] to establish a connection given a repository URL.
//!
//! All git transports are supported, including `ssh`, `git`, `http` and `https`, as well as local repository paths.
#![forbid(unsafe_code)]
#![deny(rust_2018_idioms, missing_docs)]

pub use git_packetline as packetline;

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
#[cfg(feature = "blocking-client")]
pub use client::connect;

#[cfg(all(feature = "async-client", feature = "blocking-client"))]
compile_error!("Cannot set both 'blocking-client' and 'async-client' features as they are mutually exclusive");
