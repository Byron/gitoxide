#[cfg(all(feature = "unstable", feature = "git-mailmap"))]
pub use git_mailmap::*;

///
pub mod load {
    /// The error returned by [`crate::Repository::load_mailmap_into()`].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error("The mailmap file declared in `mailmap.file` could not be read")]
        Io(#[from] std::io::Error),
        #[error("The configured mailmap.blob could not be parsed")]
        BlobSpec(#[from] git_hash::decode::Error),
        #[error(transparent)]
        PathInterpolate(#[from] git_config::values::path::interpolate::Error),
        #[error("Could not find object configured in `mailmap.blob`")]
        FindExisting(#[from] crate::object::find::existing::OdbError),
    }
}
