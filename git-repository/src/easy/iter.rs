#![allow(missing_docs)]
pub mod references {
    /// An iterator over references
    pub struct State<'r, A> {
        // pub(crate) inner: git_ref::file::iter::LooseThenPacked<'r, 'r>,
        pub(crate) access: &'r A,
    }

    mod error {
        use crate::easy;

        #[derive(Debug, thiserror::Error)]
        pub enum Error {
            #[error(transparent)]
            Io(#[from] std::io::Error),
            #[error("BUG: The repository could not be borrowed")]
            BorrowRepo(#[from] easy::borrow::repo::Error),
        }
    }
    pub use error::Error;
}
