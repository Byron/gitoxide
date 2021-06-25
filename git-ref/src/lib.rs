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
pub use store::*;

pub mod transaction {
    #![allow(missing_docs, dead_code)]
    //! Research
    //!
    //!   * `RefLogOnly`
    //!      - symbolic references don't actually change but one might still want to record the HEAD changes for convenience.
    //!      - Judging by how `reflog-transaction` hooks one transaction is scheduled for each ref, so the deref part is
    //!         actually not done automatically.
    //!   * `REF_FORCE_CREATE_REFLOG`
    //!      - required only for tags which otherwise wouldn't have a reflog. Otherwise it's up to the implementation
    //!                       which seems to have an automation on where to create a reflog or not.
    //!                       Reflogs are basic features of all stores.
    //!   * REF_NO_DEREF - Apparently dereffing is the default so detaching HEAD would need this flag to write HEAD directly
    //!                  opposedly this might mean that a change to HEAD will change the branch it points to and affect two reflogs
    //!                  automaticaly.
    //!   * Be able to delete broken refs (those with invalid content) - this is part of the ref iteration
    //!   * How to handle initial_ref_transaction_commit (to be a version that assumes no other writers)? It uses packed-refs essentially
    //!     and it does validate certain invariants, too, but doesn't have to check for file refs.
    //!     - **it's probably a standard transaction passed to `store.apply_exclusive(…)` as opposed to `store.apply(…)`.**
    //!
    //! |                         |Update        |Kind    |Data       |Reflog Mode |Deref|ref itself|reflog|referent|referent reflog|
    //! |-------------------------|--------------|--------|-----------|------------|-----|----------|------|--------|---------------|
    //! |HEAD                     |CreateOrUpdate|symbolic|oid        |only-reflog |✔    |          |✔     |✔       |✔              |
    //! |HEAD to detached HEAD    |CreateOrUpdate|symbolic|oid        |auto        |     |✔         |✔     |        |               |
    //! |detached HEAD to HEAD|CreateOrUpdate|peeled      |refpath    |auto        |     |✔         |✔     |        |               |
    //! |HEAD                     |Delete        |any     |oid        |only-reflog |✔    |          |      |        |               |
    //! |HEAD                     |Delete        |any     |oid        |auto        |✔    |✔         |✔     |        |               |
    //! |refs/heads/main          |CreateOrUpdate|peeled  |oid        |auto        |     |✔         |✔     |        |               |
    //! |refs/tags/0.1.0          |CreateOrUpdate|peeled  |oid        |auto        |     |✔         |      |        |               |
    //! |refs/tags/0.1.0          |CreateOrUpdate|peeled  |oid        |force-reflog|     |✔         |✔     |        |               |

    use crate::mutable;
    use std::marker::PhantomData;

    pub enum Update {
        /// If previous is not `None`, the ref must exist and its `oid` must agree with the `previous`, and
        /// we function like `update`.
        /// Otherwise it functions as `create-or-update`.
        Update {
            mode: Reflog,
            previous: Option<mutable::Target>,
            new: mutable::Target,
            /// Set if this update is coming from a symbolic reference and used to make it appear like it is the one that is handled,
            /// instead of the referent reference.
            parent_index: Option<usize>,
        },
        Delete {
            previous: Option<mutable::Target>,
        },
    }

    pub enum Reflog {
        /// As symbolic references only ever see this when you want to detach them, we won't try to dereference them
        /// in this case and apply the change to it directly.
        AutoAndNoDeref,
        /// Only update the reflog but require this to be a symbolic ref so the actual update can be performed on the
        /// referent.
        OnlyAndDeref,
        /// Create a reflog even if it otherwise wouldn't be created, as is the case for tags. Otherwise it acts like `AutoNoDeref`.
        CreateUnconditionally,
    }

    pub struct Transaction<T> {
        updates: Vec<usize>,
        _state: PhantomData<T>,
    }
}

/// Indicate that the given BString is a validate reference name that can be used as path on disk or written as target
/// of a symbolic reference
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
pub struct ValidatedRefname(bstr::BString);

pub mod mutable {
    //!
    #![allow(dead_code)]
    use crate::ValidatedRefname;
    use git_hash::ObjectId;

    /// Denotes a ref target, equivalent to [`Kind`][super::Kind], but with mutable data.
    #[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
    #[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
    pub enum Target {
        /// A ref that points to an object id
        Peeled(ObjectId),
        /// A ref that points to another reference by its validated name, adding a level of indirection.
        ///
        /// Note that this is an extension of gitoxide which will be helpful in logging all reference changes.
        Symbolic(ValidatedRefname),
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
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
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
