#![allow(missing_docs)]
///
pub mod find {

    use crate::easy;

    pub(crate) type OdbError = git_odb::compound::find::Error;

    #[derive(Debug, thiserror::Error)]
    pub enum Error {
        #[error(transparent)]
        Find(#[from] OdbError),
        #[error("BUG: Part of interior state could not be borrowed.")]
        BorrowState(#[from] easy::borrow::state::Error),
        #[error("BUG: The repository could not be borrowed")]
        BorrowRepo(#[from] easy::borrow::repo::Error),
    }

    pub mod existing {
        use crate::easy;

        pub(crate) type OdbError = git_odb::pack::find::existing::Error<git_odb::compound::find::Error>;

        #[derive(Debug, thiserror::Error)]
        pub enum Error {
            #[error(transparent)]
            FindExisting(#[from] OdbError),
            #[error("BUG: Part of interior state could not be borrowed.")]
            BorrowState(#[from] easy::borrow::state::Error),
            #[error("BUG: The repository could not be borrowed")]
            BorrowRepo(#[from] easy::borrow::repo::Error),
        }
    }
}

pub mod write {
    use crate::easy;

    #[derive(Debug, thiserror::Error)]
    pub enum Error {
        #[error(transparent)]
        OdbWrite(#[from] git_odb::loose::write::Error),
        #[error("BUG: The repository could not be borrowed")]
        BorrowRepo(#[from] easy::borrow::repo::Error),
    }
}
