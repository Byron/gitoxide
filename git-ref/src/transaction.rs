#![allow(dead_code)]
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

use bstr::{BStr, BString, ByteSlice};
use git_hash::ObjectId;
use std::{borrow::Cow, convert::TryFrom, path::Path};

/// Indicate that the given BString is a validate reference name or path that can be used as path on disk or written as target
/// of a symbolic reference
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
pub struct FullName(BString);

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

impl<'a> From<crate::Target<'a>> for Target {
    fn from(src: crate::Target<'a>) -> Self {
        match src {
            crate::Target::Peeled(oid) => Target::Peeled(oid.to_owned()),
            crate::Target::Symbolic(name) => Target::Symbolic(FullName(name.to_owned())),
        }
    }
}

/// Update an existing or a new reference.
pub struct Update {
    /// How to treat the reference log.
    pub mode: Reflog,
    /// The previous value of the ref, which will be used to assure the ref is still in the known `previous` state before
    /// updating it. It will also be filled in automatically for use in the reflog, if applicable.
    pub previous: Option<Target>,
    /// The new state of the reference, either for updating an existing one or creating a new one.
    pub new: Target,
}

/// A description of an edit to perform.
pub enum Change {
    /// If previous is not `None`, the ref must exist and its `oid` must agree with the `previous`, and
    /// we function like `update`.
    /// Otherwise it functions as `create-or-update`.
    Update(Update),
    /// Delete a reference and optionally check if `previous` is its content.
    Delete {
        /// The previous state of the reference
        previous: Option<Target>,
    },
}

/// A reference that is to be changed
pub struct RefEdit {
    /// The change itself
    pub edit: Change,
    /// The name of the reference to apply the change to
    pub name: FullName,
}

/// An extension trait to perform commonly used operations on edits across different ref stores.
pub trait RefEditsExt {
    /// Return true if each ref `name` has exactly one `edit` across multiple ref edits
    fn assure_one_name_has_one_edit(&self) -> Result<(), BString>;
}

impl<E> RefEditsExt for Vec<E>
where
    E: std::borrow::Borrow<RefEdit>,
{
    fn assure_one_name_has_one_edit(&self) -> Result<(), BString> {
        let mut names: Vec<_> = self.iter().map(|e| &e.borrow().name).collect();
        names.sort();
        match names.windows(2).find(|v| v[0] == v[1]) {
            Some(name) => Err(name[0].as_ref().to_owned()),
            None => Ok(()),
        }
    }
}

/// The way to deal with the Reflog in a particular edit
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
