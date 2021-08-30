#![allow(missing_docs)]
pub mod state {
    #[derive(Debug, thiserror::Error)]
    pub enum Error {
        #[error("A state member could not be borrowed")]
        Borrow(#[from] std::cell::BorrowError),
        #[error("A state member could not be mutably borrowed")]
        BorrowMut(#[from] std::cell::BorrowMutError),
    }

    pub type Result<T> = std::result::Result<T, Error>;
}

pub mod repo {
    use std::{
        cell::{BorrowError, BorrowMutError},
        fmt::{Display, Formatter},
    };

    #[derive(Debug)]
    pub struct Error;

    impl Display for Error {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            "Could not borrow the repository as it's already borrowed mutably".fmt(f)
        }
    }

    impl std::error::Error for Error {}

    impl From<BorrowError> for Error {
        fn from(_: BorrowError) -> Self {
            Error
        }
    }

    impl From<BorrowMutError> for Error {
        fn from(_: BorrowMutError) -> Self {
            Error
        }
    }

    pub type Result<T> = std::result::Result<T, Error>;
}
