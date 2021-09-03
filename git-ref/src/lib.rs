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
use bstr::{BStr, BString};
use git_hash::{oid, ObjectId};

mod store;
pub use store::{file, packed};

mod fullname;
///
pub mod name;
///
pub mod namespace;
///
pub mod transaction;

/// Indicate that the given BString is a validate reference name or path that can be used as path on disk or written as target
/// of a symbolic reference
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
pub struct FullName(pub(crate) BString);

/// A validated and potentially partial reference name - it can safely be used for common operations.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
pub struct FullNameRef<'a>(&'a BStr);
/// A validated complete and fully qualified reference name, safe to use for all operations.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
pub struct PartialNameRef<'a>(&'a BStr);

/// A validated prefix for references to act as a namespace.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
pub struct Namespace(BString);

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

/// Denotes a ref target, equivalent to [`Kind`], but with mutable data.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
pub enum Target {
    /// A ref that points to an object id
    Peeled(ObjectId),
    /// A ref that points to another reference by its validated name, adding a level of indirection.
    ///
    /// Note that this is an extension of gitoxide which will be helpful in logging all reference changes.
    Symbolic(FullName),
}

/// Denotes a ref target, equivalent to [`Kind`], but with immutable data.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
pub enum TargetRef<'a> {
    /// A ref that points to an object id
    Peeled(&'a oid),
    /// A ref that points to another reference by its validated name, adding a level of indirection.
    Symbolic(&'a BStr),
}

mod parse;
mod raw;
mod target;
