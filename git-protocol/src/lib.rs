#![forbid(unsafe_code)]

mod remote_progress;
pub use remote_progress::RemoteProgress;

pub mod fetch {
    use git_transport::client;

    pub trait Delegate {}

    pub fn fetch(
        _transport: impl client::Transport,
        _delegate: impl Delegate,
        _authenticate: impl FnMut() -> client::Identity,
    ) {
        unimplemented!("fetch")
    }
}

#[doc(inline)]
pub use fetch::fetch;
