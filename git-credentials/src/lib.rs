//! Interact with git credentials in various ways and launch helper programs.
//!
//! ## Feature Flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![deny(missing_docs, rust_2018_idioms)]
#![forbid(unsafe_code)]

/// A utility trait to launch a credentials helper, as well as stop them gracefully.
pub trait Helper {
    /// A way to send data to the helper.
    type Send: std::io::Write;
    /// A way to receive data from the helper.
    type Receive: std::io::Read;

    /// Start the helper and provide handles to send and receive from it.
    fn start(&mut self, action: &helper::invoke::Action) -> std::io::Result<(Self::Send, Option<Self::Receive>)>;
    /// Stop the helper and provide a way to determine it's successful.
    fn finish(self) -> std::io::Result<()>;
}

/// A program/executable implementing the credential helper protocol.
pub enum Program {
    /// The kind of program, ready for launch
    Ready(helper::Kind),
    /// The process is running.
    Started(std::process::Child),
}

mod program;

///
pub mod helper;

/// Call the `git` credentials helper program performing the given `action`, without any context from git configuration.
///
/// See [`invoke()`][helper::invoke()] for a more flexible implementation.
pub fn helper(action: helper::invoke::Action) -> helper::invoke::Result {
    helper::invoke(Program::from_kind(helper::Kind::GitCredential), action)
}
