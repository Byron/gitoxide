//! Various functionality related to git references
#![forbid(unsafe_code)]
#![deny(rust_2018_idioms)]
#![allow(missing_docs)]

pub mod refname {
    use bstr::BStr;
    use quick_error::quick_error;

    quick_error! {
        #[derive(Debug)]
        pub enum Error {
            Todo
        }
    }

    pub fn refname(path: &BStr) -> Result<&BStr, Error> {
        Ok(path)
    }
}
pub use refname::refname;
