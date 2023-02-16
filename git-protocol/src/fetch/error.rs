use std::io;

use gix_transport::client;

use crate::{fetch::response, handshake, ls_refs};

/// The error used in [`fetch()`][crate::fetch()].
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error(transparent)]
    Handshake(#[from] handshake::Error),
    #[error("Could not access repository or failed to read streaming pack file")]
    Io(#[from] io::Error),
    #[error(transparent)]
    Transport(#[from] client::Error),
    #[error(transparent)]
    LsRefs(#[from] ls_refs::Error),
    #[error(transparent)]
    Response(#[from] response::Error),
}
