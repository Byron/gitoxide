use crate::credentials;
use git_transport::{client, Service};
use quick_error::quick_error;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Credentials(err: credentials::Error) {
            display("Failed to obtain, approve or reject credentials")
            from()
            source(err)
        }
        Transport(err: client::Error) {
            display("An error occurred on the transport layer while fetching data")
            from()
            source(err)
        }
    }
}

pub trait Delegate {}

pub fn fetch(
    mut transport: impl client::Transport,
    _delegate: impl Delegate,
    _authenticate: impl FnMut(credentials::Action) -> credentials::Result,
) -> Result<(), Error> {
    let client::SetServiceResponse {
        actual_protocol,
        capabilities,
        refs,
    } = transport.handshake(Service::UploadPack)?;
    unimplemented!("fetch")
}
