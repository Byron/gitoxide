use crate::credentials;
use git_transport::client;
use quick_error::quick_error;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Credentials(err: credentials::Error) {
            display("Failed to obtain, approve or reject credentials")
            from()
            source(err)
        }
    }
}

pub trait Delegate {}

pub fn fetch(
    _transport: impl client::Transport,
    _delegate: impl Delegate,
    _authenticate: impl FnMut(credentials::Action) -> credentials::Result,
) -> Result<(), Error> {
    unimplemented!("fetch")
}
