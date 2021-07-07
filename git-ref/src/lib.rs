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
pub mod name;
///
pub mod transaction;
///
pub mod mutable {
    use bstr::{BStr, BString, ByteSlice};
    use git_hash::ObjectId;
    use std::{borrow::Cow, convert::TryFrom, fmt, path::Path};

    /// Indicate that the given BString is a validate reference name or path that can be used as path on disk or written as target
    /// of a symbolic reference
    #[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
    pub struct FullName(pub(crate) BString);

    impl TryFrom<&str> for FullName {
        type Error = git_validate::refname::Error;

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            Ok(FullName(git_validate::refname(value.as_bytes().as_bstr())?.into()))
        }
    }

    impl AsRef<BStr> for FullName {
        fn as_ref(&self) -> &BStr {
            self.0.as_bstr()
        }
    }

    impl FullName {
        /// Interpret this fully qualified reference name as partial name.
        pub fn to_partial(&self) -> crate::PartialName<'_> {
            crate::PartialName(self.0.as_bstr())
        }

        /// Interpret this fully qualified reference as shared full name
        pub fn borrow(&self) -> crate::FullName<'_> {
            crate::FullName(self.0.as_bstr())
        }

        /// Convert this name into the relative path identifying the reference location relative to a repository
        pub fn to_path(&self) -> Cow<'_, Path> {
            self.0.to_path_lossy()
        }
    }

    /// Denotes a ref target, equivalent to [`Kind`][super::Kind], but with mutable data.
    #[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
    pub enum Target {
        /// A ref that points to an object id
        Peeled(ObjectId),
        /// A ref that points to another reference by its validated name, adding a level of indirection.
        ///
        /// Note that this is an extension of gitoxide which will be helpful in logging all reference changes.
        Symbolic(FullName),
    }

    impl Target {
        /// Return true if this is a peeled target with a null hash
        pub fn is_null(&self) -> bool {
            match self {
                Target::Peeled(oid) => oid.is_null(),
                Target::Symbolic(_) => false,
            }
        }

        /// Interpret this owned Target as shared Target
        pub fn borrow(&self) -> crate::Target<'_> {
            match self {
                Target::Peeled(oid) => crate::Target::Peeled(&oid),
                Target::Symbolic(name) => crate::Target::Symbolic(name.0.as_bstr()),
            }
        }

        /// Create an instance that signals that a reference should exist if this value is used in a [`Change`][crate::transaction::Change].
        pub fn must_exist() -> Self {
            Target::Peeled(ObjectId::null_sha1())
        }
    }

    impl<'a> From<crate::Target<'a>> for Target {
        fn from(src: crate::Target<'a>) -> Self {
            match src {
                crate::Target::Peeled(oid) => Target::Peeled(oid.to_owned()),
                crate::Target::Symbolic(name) => Target::Symbolic(FullName(name.to_owned())),
            }
        }
    }

    impl<'a> PartialEq<crate::Target<'a>> for Target {
        fn eq(&self, other: &crate::Target<'a>) -> bool {
            match (self, other) {
                (Target::Peeled(lhs), crate::Target::Peeled(rhs)) => lhs == rhs,
                (Target::Symbolic(lhs), crate::Target::Symbolic(rhs)) => lhs.as_ref() == *rhs,
                _ => false,
            }
        }
    }

    impl fmt::Display for Target {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Target::Peeled(oid) => oid.fmt(f),
                Target::Symbolic(name) => write!(f, "ref: {}", name.as_ref()),
            }
        }
    }
}

mod traits {
    use crate::{mutable::Target, PartialName};

    /// A minimal trait to group useful operations for handling references across store implementations.
    pub trait RefStore {
        /// The error used in [`RefStore::find_one_existing()`].
        type FindOneExistingError;

        /// Find the reference with the given `name`. Return `Ok(None)` if the reference doesn't exist.
        fn find_one_existing(&self, name: PartialName<'_>) -> Result<Target, Self::FindOneExistingError>;
    }
}
pub use traits::RefStore;

/// A validated and potentially partial reference name - it can safely be used for common operations.
pub struct FullName<'a>(&'a BStr);
/// A validated complete and fully qualified reference name, safe to use for all operations.
pub struct PartialName<'a>(&'a BStr);

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

mod target {
    use crate::{Kind, Target};
    use bstr::BStr;
    use git_hash::oid;

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
        pub fn as_name(&self) -> Option<&BStr> {
            match self {
                Target::Symbolic(path) => Some(path),
                Target::Peeled(_) => None,
            }
        }
        /// Convert this instance into an owned version, without consuming it.
        pub fn to_owned(self) -> crate::mutable::Target {
            self.into()
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
