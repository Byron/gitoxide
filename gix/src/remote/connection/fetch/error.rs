use crate::config;

/// The error returned by [`receive()`](super::Prepare::receive()).
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error("The value to configure pack threads should be 0 to auto-configure or the amount of threads to use")]
    PackThreads(#[from] config::unsigned_integer::Error),
    #[error("The value to configure the pack index version should be 1 or 2")]
    PackIndexVersion(#[from] config::key::GenericError),
    #[error("Could not decode server reply")]
    FetchResponse(#[from] gix_protocol::fetch::response::Error),
    #[error("Cannot fetch from a remote that uses {remote} while local repository uses {local} for object hashes")]
    IncompatibleObjectHash {
        local: gix_hash::Kind,
        remote: gix_hash::Kind,
    },
    #[error(transparent)]
    Negotiate(#[from] super::negotiate::Error),
    #[error(transparent)]
    Client(#[from] gix_protocol::transport::client::Error),
    #[error(transparent)]
    WritePack(#[from] gix_pack::bundle::write::Error),
    #[error(transparent)]
    UpdateRefs(#[from] super::refs::update::Error),
    #[error("Failed to remove .keep file at \"{}\"", path.display())]
    RemovePackKeepFile {
        path: std::path::PathBuf,
        source: std::io::Error,
    },
    #[error(transparent)]
    ShallowOpen(#[from] crate::shallow::open::Error),
    #[error("Server lack feature {feature:?}: {description}")]
    MissingServerFeature {
        feature: &'static str,
        description: &'static str,
    },
    #[error("Could not write 'shallow' file to incorporate remote updates after fetching")]
    WriteShallowFile(#[from] crate::shallow::write::Error),
    #[error("'shallow' file could not be locked in preparation for writing changes")]
    LockShallowFile(#[from] gix_lock::acquire::Error),
    #[error("Could not obtain configuration to learn if shallow remotes should be rejected")]
    RejectShallowRemoteConfig(#[from] config::boolean::Error),
    #[error("Receiving objects from shallow remotes is prohibited due to the value of `clone.rejectShallow`")]
    RejectShallowRemote,
    #[error(transparent)]
    NegotiationAlgorithmConfig(#[from] config::key::GenericErrorWithValue),
}

impl gix_protocol::transport::IsSpuriousError for Error {
    fn is_spurious(&self) -> bool {
        match self {
            Error::FetchResponse(err) => err.is_spurious(),
            Error::Client(err) => err.is_spurious(),
            _ => false,
        }
    }
}
