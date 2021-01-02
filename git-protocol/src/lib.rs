#![deny(unsafe_code)]
#![deny(rust_2018_idioms)]

/// A convenience export allowing users of git-protocol to use the transport layer without their own cargo dependency.
pub use git_transport;

mod remote_progress;
pub use remote_progress::RemoteProgress;

pub mod credentials;
pub mod fetch;

#[doc(inline)]
pub use fetch::fetch;
