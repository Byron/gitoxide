//! Various functionality related to git references
#![forbid(unsafe_code)]
#![deny(rust_2018_idioms)]
#![allow(missing_docs)]
use bstr::BStr;
use git_hash::oid;

mod store;
pub use store::*;

/// Denotes the kind of function to produce a `Id`
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum Kind {
    /// A reference that points to an object id
    Peeled,
    /// A reference that points to another reference
    Symbolic,
}

/// Denotes a ref target, equivalent to [`Kind`], but with data.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum Target<'a> {
    /// A reference that points to an object id
    Peeled(&'a oid),
    /// A reference that points to another reference
    Symbolic(&'a BStr),
}

impl<'a> Target<'a> {
    pub fn kind(&self) -> Kind {
        match self {
            Target::Symbolic(_) => Kind::Symbolic,
            Target::Peeled(_) => Kind::Peeled,
        }
    }
    pub fn as_id(&self) -> Option<&oid> {
        match self {
            Target::Symbolic(_) => None,
            Target::Peeled(oid) => Some(oid),
        }
    }
    pub fn as_ref(&self) -> Option<&BStr> {
        match self {
            Target::Symbolic(path) => Some(path),
            Target::Peeled(_) => None,
        }
    }
}

mod validate {
    use bstr::BStr;

    pub mod refname {
        use quick_error::quick_error;
        quick_error! {
            #[derive(Debug)]
            pub enum Error {
                Todo
            }
        }
    }

    pub fn refname(path: &BStr) -> Result<&BStr, refname::Error> {
        Ok(path)
    }
}
