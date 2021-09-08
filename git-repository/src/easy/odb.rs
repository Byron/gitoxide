#![allow(missing_docs)]
pub mod refresh {
    use crate::easy;

    #[derive(Debug, thiserror::Error)]
    pub enum Error {
        #[error(transparent)]
        Refresh(#[from] git_odb::linked::init::Error),
        #[error(transparent)]
        BorrowRepoMut(#[from] easy::borrow::repo::Error),
    }
}
