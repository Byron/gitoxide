//! An abstraction over [fetching][fetch()] a pack from the server.
//!
//! This implementation hides the transport layer, statefulness and the protocol version to the [fetch delegate][fetch::Delegate],
//! the actual client implementation.
//! ## Feature Flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![deny(missing_docs, rust_2018_idioms, unsafe_code)]

#[cfg(feature = "async-trait")]
pub use async_trait;
#[cfg(feature = "futures-io")]
pub use futures_io;
pub use git_credentials as credentials;
/// A convenience export allowing users of git-protocol to use the transport layer without their own cargo dependency.
pub use git_transport as transport;
pub use maybe_async;

///
#[cfg(any(feature = "blocking-client", feature = "async-client"))]
pub mod fetch;

#[cfg(any(feature = "blocking-client", feature = "async-client"))]
mod fetch_fn;
#[cfg(any(feature = "blocking-client", feature = "async-client"))]
pub use fetch_fn::{fetch, fetch_pack, FetchConnection};

mod remote_progress;
pub use remote_progress::RemoteProgress;

#[cfg(all(feature = "blocking-client", feature = "async-client"))]
compile_error!("Cannot set both 'blocking-client' and 'async-client' features as they are mutually exclusive");
