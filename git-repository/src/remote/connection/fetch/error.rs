/// The error returned by [`receive()`](super::Prepare::receive()).
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error("{message}{}", desired.map(|n| format!(" (got {})", n)).unwrap_or_default())]
    Configuration {
        message: &'static str,
        desired: Option<i64>,
        source: Option<git_config::value::Error>,
    },
    #[error("Could not decode server reply")]
    FetchResponse(#[from] git_protocol::fetch::response::Error),
    #[error("Cannot fetch from a remote that uses {remote} while local repository uses {local} for object hashes")]
    IncompatibleObjectHash {
        local: git_hash::Kind,
        remote: git_hash::Kind,
    },
    #[error(transparent)]
    Negotiate(#[from] super::negotiate::Error),
    #[error(transparent)]
    Client(#[from] git_protocol::transport::client::Error),
    #[error(transparent)]
    WritePack(#[from] git_pack::bundle::write::Error),
    #[error(transparent)]
    UpdateRefs(#[from] super::refs::update::Error),
    #[error("Failed to remove .keep file at \"{}\"", path.display())]
    RemovePackKeepFile {
        path: std::path::PathBuf,
        source: std::io::Error,
    },
}
