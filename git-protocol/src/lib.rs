//! An abstraction over [fetching][fetch()] a pack from the server.
//!
//! This implementation hides the transport layer, statefulness and the protocol version to the [fetch delegate][fetch::Delegate],
//! the actual client implementation.
#![deny(unsafe_code)]
#![deny(rust_2018_idioms, missing_docs)]

/// A convenience export allowing users of git-protocol to use the transport layer without their own cargo dependency.
pub use git_transport as transport;

///
pub mod credentials;
///
#[cfg(any(feature = "blocking-client", feature = "async-client"))]
pub mod fetch;

#[cfg(any(feature = "blocking-client", feature = "async-client"))]
mod fetch_fn;
#[cfg(any(feature = "blocking-client", feature = "async-client"))]
pub use fetch_fn::fetch;

mod remote_progress;
pub use remote_progress::RemoteProgress;

#[cfg(all(feature = "blocking-client", feature = "async-client"))]
compile_error!("Cannot set both 'blocking-client' and 'async-client' features as they are mutually exclusive");
