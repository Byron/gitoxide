#![forbid(unsafe_code)]

mod remote_progress;
pub use remote_progress::RemoteProgress;

pub mod credentials;

pub mod fetch {
    use crate::credentials;
    use git_transport::client;

    pub trait Delegate {}

    pub fn fetch(
        _transport: impl client::Transport,
        _delegate: impl Delegate,
        _authenticate: impl FnMut(credentials::Action, &str) -> Result<client::Identity, crate::credentials::Error>,
    ) {
        unimplemented!("fetch")
    }
}

#[doc(inline)]
pub use fetch::fetch;
