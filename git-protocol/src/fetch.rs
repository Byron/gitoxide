use crate::credentials;
use git_transport::client;

pub trait Delegate {}

pub fn fetch(
    _transport: impl client::Transport,
    _delegate: impl Delegate,
    _authenticate: impl FnMut(credentials::Action, &str) -> credentials::Result,
) {
    unimplemented!("fetch")
}
