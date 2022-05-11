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
#![deny(unsafe_code, missing_docs, rust_2018_idioms)]

use std::borrow::Cow;

use git_hash::{oid, ObjectId};
pub use git_object::bstr;
use git_object::bstr::{BStr, BString};

#[path = "store/mod.rs"]
mod store_impl;
pub use store_impl::{file, packed};

mod fullname;
///
pub mod name;
///
pub mod namespace;
///
pub mod transaction;

mod parse;
mod raw;

pub use raw::Reference;

mod target;

///
pub mod log;

///
pub mod peel;

///
pub mod store {
    /// The way a file store handles the reflog
    #[derive(Debug, PartialOrd, PartialEq, Ord, Eq, Hash, Clone, Copy)]
    pub enum WriteReflog {
        /// Write a ref log for ref edits according to the standard rules.
        Normal,
        /// Never write a ref log.
        Disable,
    }

    /// A thread-local handle for interacting with a [`Store`][crate::Store] to find and iterate references.
    #[derive(Clone)]
    #[allow(dead_code)]
    pub(crate) struct Handle {
        /// A way to access shared state with the requirement that interior mutability doesn't leak or is incorporated into error types
        /// if it could. The latter can't happen if references to said internal aren't ever returned.
        state: handle::State,
    }

    #[allow(dead_code)]
    pub(crate) enum State {
        Loose { store: file::Store },
    }

    pub(crate) mod general;

    ///
    #[path = "general/handle/mod.rs"]
    mod handle;
    pub use handle::find;

    use crate::file;
}

/// The git reference store.
/// TODO: Figure out if handles are needed at all, which depends on the ref-table implementation.
#[allow(dead_code)]
pub(crate) struct Store {
    inner: store::State,
}

/// Indicate that the given BString is a validate reference name or path that can be used as path on disk or written as target
/// of a symbolic reference
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
pub struct FullName(pub(crate) BString);

/// A validated and potentially partial reference name - it can safely be used for common operations.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
pub struct FullNameRef<'a>(&'a BStr);
/// A validated complete and fully qualified reference name, safe to use for all operations.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
pub struct PartialNameRef<'a>(Cow<'a, BStr>);

/// A _validated_ prefix for references to act as a namespace.
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
    /// It can be resolved to an id using the [`peel_in_place_to_id()`][`crate::file::ReferenceExt::peel_to_id_in_place()`] method.
    Symbolic,
}

/// The various known categories of references.
///
/// This translates into a prefix containing all references of a given category.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
pub enum Category {
    /// A tag in `refs/tags`
    Tag,
    /// A branch in `refs/heads`
    LocalBranch,
    /// A branch in `refs/remotes`
    RemoteBranch,
    /// A tag in `refs/notes`
    Note,
    /// Something outside of `ref/` in the current worktree.
    PseudoRef,
    // REF_TYPE_PER_WORKTREE,	  /* refs inside refs/ but not shared       */
    // REF_TYPE_MAIN_PSEUDOREF,  /* pseudo refs from the main worktree     */
    // REF_TYPE_OTHER_PSEUDOREF, /* pseudo refs from other worktrees       */
    // REF_TYPE_NORMAL,	  /* normal/shared refs inside refs/        */
    // NOTE: when adding something here, add it to `kind()` and `strip_prefix()` too.
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
    Symbolic(FullNameRef<'a>),
}
