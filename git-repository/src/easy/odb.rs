//!
///
pub mod refresh {
    use crate::easy;

    /// The error returned by [`RepositoryAccessExt::refresh_object_database(…)`][easy::ext::RepositoryAccessExt::refresh_object_database()].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error(transparent)]
        Refresh(#[from] git_odb::linked::init::Error),
        #[error(transparent)]
        BorrowRepoMut(#[from] easy::borrow::repo::Error),
    }
}
