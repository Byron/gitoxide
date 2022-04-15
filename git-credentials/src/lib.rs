#![forbid(unsafe_code)]
#![deny(missing_docs, rust_2018_idioms)]
//! Interact with git credentials in various ways and launch helper programs.

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
/// An identity for use when authenticating the transport layer.
pub enum Identity {
    /// An account based identity
    Account {
        /// The user's name
        username: String,
        /// The user's password
        password: String,
    },
}

///
pub mod helper;
