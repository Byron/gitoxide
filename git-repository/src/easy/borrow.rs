pub mod state {
    use quick_error::quick_error;
    quick_error! {
        #[derive(Debug)]
        pub enum Error {
            Borrow(err: std::cell::BorrowError) {
                display("A state member could not be borrowed")
                from()
            }
            BorrowMut(err: std::cell::BorrowMutError) {
                display("A state member could not be mutably borrowed")
                from()
            }
        }
    }
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
}
