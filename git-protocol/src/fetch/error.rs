use std::io;

use git_transport::client;

use crate::{
    credentials,
    fetch::{refs, response},
};

/// The error used in [`fetch()`][super::fetch].
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error("Could not access repository or failed to read streaming pack file")]
    Io(#[from] io::Error),
    #[error(transparent)]
    Credentials(#[from] credentials::helper::Error),
    #[error(transparent)]
    Transport(#[from] client::Error),
    #[error("A symref 'capability' is expected to have a value")]
    SymrefWithoutValue,
    #[error("The transport didn't accept the advertised server version {actual_version:?} and closed the connection client side")]
    TransportProtocolPolicyViolation { actual_version: git_transport::Protocol },
    #[error(transparent)]
    Ref(#[from] refs::Error),
    #[error(transparent)]
    Response(#[from] response::Error),
}
