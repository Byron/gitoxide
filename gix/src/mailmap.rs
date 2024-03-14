pub use gix_mailmap::*;

///
#[allow(clippy::empty_docs)]
pub mod load {
    /// The error returned by [`crate::Repository::open_mailmap_into()`].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error("The mailmap file declared in `mailmap.file` could not be read")]
        Io(#[from] std::io::Error),
        #[error("The configured mailmap.blob could not be parsed")]
        BlobSpec(#[from] crate::revision::spec::parse::single::Error),
        #[error(transparent)]
        PathInterpolate(#[from] gix_config::path::interpolate::Error),
        #[error("Could not find object configured in `mailmap.blob`")]
        FindExisting(#[from] crate::object::find::existing::Error),
    }
}
