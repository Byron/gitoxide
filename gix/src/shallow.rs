pub(crate) type CommitsStorage =
    gix_features::threading::OwnShared<gix_features::fs::MutableSnapshot<Vec<gix_hash::ObjectId>>>;
/// A lazily loaded and auto-updated list of commits which are at the shallow boundary (behind which there are no commits available),
/// sorted to allow bisecting.
pub type Commits = gix_features::fs::SharedSnapshot<Vec<gix_hash::ObjectId>>;

///
pub mod open {
    /// The error returned by [`Repository::shallow_commits()`][crate::Repository::shallow_commits()].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error("Could not open shallow file for reading")]
        Io(#[from] std::io::Error),
        #[error("Could not decode a line in shallow file as hex-encoded object hash")]
        DecodeHash(#[from] gix_hash::decode::Error),
    }
}
