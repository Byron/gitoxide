//! A crate for handling the references stored in various formats in a git repository.
//!
//! References are also called _refs_ which are used interchangeably.
//!
//! Refs are the way to keep track of objects and come in two flavors.
//!
//! * symbolic refs are pointing to another reference
//! * peeled refs point to the an object by its [ObjectId][git_hash::ObjectId]
//!
//! They can be identified by a relative path and stored in various flavors.
//!
//! * **files**
//!   * **[loose][file::Store]**
//!     * one reference maps to a file on disk
//!   * **packed**
//!     * references are stored in a single human-readable file, along with their targets if they are symbolic.
//! * **ref-table**
//!   * supersedes all of the above to allow handling hundreds of thousands of references.
#![forbid(unsafe_code)]
#![deny(missing_docs, rust_2018_idioms)]
use bstr::BStr;
use git_hash::oid;

mod store;
pub use store::file;

///
pub mod edit;

/// Indicate that the given BString is a validate reference name or path that can be used as path on disk or written as target
/// of a symbolic reference
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
pub struct ValidName(bstr::BString);

pub mod mutable {
    //!
    use crate::ValidName;
    use git_hash::ObjectId;

    /// Denotes a ref target, equivalent to [`Kind`][super::Kind], but with mutable data.
    #[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
    pub enum Target {
        /// A ref that points to an object id
        Peeled(ObjectId),
        /// A ref that points to another reference by its validated name, adding a level of indirection.
        ///
        /// Note that this is an extension of gitoxide which will be helpful in logging all reference changes.
        Symbolic(ValidName),
    }
}

/// A validated and potentially partial reference name - it can safely be used for common operations.
pub struct SafePartialName<'a>(&'a BStr);
mod safe_name;

/// Denotes the kind of reference.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum Kind {
    /// A ref that points to an object id
    Peeled,
    /// A ref that points to another reference, adding a level of indirection.
    ///
    /// It can be resolved to an id using the [`peel_in_place_to_id()`][file::Reference::peel_to_id_in_place()] method.
    Symbolic,
}

/// Denotes a ref target, equivalent to [`Kind`], but with immutable data.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
pub enum Target<'a> {
    /// A ref that points to an object id
    Peeled(&'a oid),
    /// A ref that points to another reference by its validated name, adding a level of indirection.
    Symbolic(&'a BStr),
}

impl<'a> Target<'a> {
    /// Returns the kind of the target the ref is pointing to.
    pub fn kind(&self) -> Kind {
        match self {
            Target::Symbolic(_) => Kind::Symbolic,
            Target::Peeled(_) => Kind::Peeled,
        }
    }
    /// Interpret this target as object id which maybe `None` if it is symbolic.
    pub fn as_id(&self) -> Option<&oid> {
        match self {
            Target::Symbolic(_) => None,
            Target::Peeled(oid) => Some(oid),
        }
    }
    /// Interpret this target as name of the reference it points to which maybe `None` if it an object id.
    pub fn as_ref(&self) -> Option<&BStr> {
        match self {
            Target::Symbolic(path) => Some(path),
            Target::Peeled(_) => None,
        }
    }
}

mod parse {
    use nom::{
        branch::alt,
        bytes::complete::{tag, take_while_m_n},
        error::ParseError,
        IResult,
    };

    fn is_hex_digit_lc(b: u8) -> bool {
        matches!(b, b'0'..=b'9' | b'a'..=b'f')
    }

    /// Copy from https://github.com/Byron/gitoxide/blob/f270850ff92eab15258023b8e59346ec200303bd/git-object/src/immutable/parse.rs#L64
    pub fn hex_sha1<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], &'a [u8], E> {
        take_while_m_n(40usize, 40, is_hex_digit_lc)(i)
    }

    pub fn newline(i: &[u8]) -> IResult<&[u8], &[u8]> {
        alt((tag(b"\r\n"), tag(b"\n")))(i)
    }
}
