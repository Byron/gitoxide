//! An abstraction over [fetching][fetch()] a pack from the server.
//!
//! This implementation hides the transport layer, statefulness and the protocol version to the [fetch delegate][fetch::Delegate],
//! the actual client implementation.
#![deny(unsafe_code)]
#![deny(rust_2018_idioms, missing_docs)]
#![cfg_attr(
    any(
        feature = "async-client",
        all(not(feature = "blocking-client"), not(feature = "async-client"))
    ),
    allow(dead_code)
)] // TODO: remove this and assure this holds at when done.

/// A convenience export allowing users of git-protocol to use the transport layer without their own cargo dependency.
pub use git_transport as transport;

mod remote_progress;
pub use remote_progress::RemoteProgress;

///
pub mod credentials;
///
pub mod fetch;

#[doc(inline)]
#[cfg(feature = "blocking-client")]
pub use fetch::fetch;
