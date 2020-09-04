#![deny(unsafe_code)]

pub use git_transport;

mod remote_progress;
pub use remote_progress::RemoteProgress;

pub mod credentials;
pub mod fetch;

#[doc(inline)]
pub use fetch::fetch;
