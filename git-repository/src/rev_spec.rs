///
pub mod parse {
    /// The error returned by [`crate::Repository::rev_parse()`].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error(transparent)]
        IdFromHex(#[from] git_hash::decode::Error),
        #[error(transparent)]
        Find(#[from] crate::object::find::existing::OdbError),
    }
}
