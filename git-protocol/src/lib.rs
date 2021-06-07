//! An abstraction over [fetching][fetch()] a pack from the server.
//!
//! This implementation hides the transport layer, statefulness and the protocol version to the [fetch delegate][fetch::Delegate],
//! the actual client implementation.
#![deny(unsafe_code)]
#![deny(rust_2018_idioms, missing_docs)]

/// A convenience export allowing users of git-protocol to use the transport layer without their own cargo dependency.
pub use git_transport as transport;

mod remote_progress;
pub use remote_progress::RemoteProgress;

///
pub mod credentials;
///
pub mod fetch;

#[doc(inline)]
#[cfg(any(feature = "blocking-client", feature = "async-client"))]
pub use fetch::fetch;

#[cfg(all(feature = "blocking-client", feature = "async-client"))]
compile_error!("Cannot set both 'blocking-client' and 'async-client' features as they are mutually exclusive");
