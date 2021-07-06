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

use crate::RefStore;
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
    pub fn to_shared(&self) -> crate::FullName<'_> {
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

/// A description of an edit to perform.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
pub enum Change {
    /// If previous is not `None`, the ref must exist and its `oid` must agree with the `previous`, and
    /// we function like `update`.
    /// Otherwise it functions as `create-or-update`.
    Update {
        /// How to treat the reference log.
        mode: UpdateMode,
        /// The previous value of the ref, which will be used to assure the ref is still in the known `previous` state before
        /// updating it. It will also be filled in automatically for use in the reflog, if applicable.
        previous: Option<Target>,
        /// The new state of the reference, either for updating an existing one or creating a new one.
        new: Target,
    },
    /// Delete a reference and optionally check if `previous` is its content.
    Delete {
        /// The previous state of the reference. If set, the reference is expected to exist and match the given value.
        /// If the value is a peeled null-id the reference is expected to exist but the value doesn't matter, neither peeled nor symbolic.
        ///
        /// If a previous ref existed, this value will be filled in automatically and can be accessed if the transaction was committed successfully.
        previous: Option<Target>,
        /// How to thread the reference log during deletion.
        mode: DeleteMode,
    },
}

/// A reference that is to be changed
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
pub struct RefEdit {
    /// The change itself
    pub change: Change,
    /// The name of the reference to apply the change to
    pub name: FullName,
}

/// An extension trait to perform commonly used operations on edits across different ref stores.
pub trait RefEditsExt<T> {
    /// Return true if each ref `name` has exactly one `edit` across multiple ref edits
    fn assure_one_name_has_one_edit(&self) -> Result<(), BString>;

    /// Split all symbolic refs into updates for the symbolic ref as well as all their referents if the update or delete mode allows
    /// dereferencing them.
    ///
    /// Note no action is performed if deref isn't specified.
    /// If deref is specified and the reference isn't a symbolic reference, the deref mode will be changed
    /// to match.
    fn extend_with_splits_of_symbolic_refs(
        &mut self,
        store: &impl RefStore,
        make_entry: impl FnMut(RefEdit) -> T,
    ) -> Result<(), std::io::Error>;
}

impl<E> RefEditsExt<E> for Vec<E>
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
    fn extend_with_splits_of_symbolic_refs(
        &mut self,
        _store: &impl RefStore,
        _make_entry: impl FnMut(RefEdit) -> E,
    ) -> Result<(), std::io::Error> {
        let new_edits = Vec::new();
        for _edit in self.iter() {}
        self.extend(new_edits.into_iter());
        Ok(())
    }
}

/// The way to deal with the Reflog in deletions.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
pub enum DeleteMode {
    /// As symbolic references only ever see this when you want to detach them, we won't try to dereference them
    /// in this case and apply the change to it and its reflog directly.
    RefAndRefLogNoDeref,
    /// As above, but delete the ref log only without dereferencing symbolic refs
    RefLogOnlyNoDeref,
    /// Delete the reference and the log, but also apply this to the referent, recursively.
    RefAndRefLogDeref,
    /// Delete only the reflog, also deleting the referents reflog, recursively.
    RefLogOnlyDeref,
}

/// The way to deal with the Reflog in a particular edit
///
/// If the `create_unconditionally` field is set, create a reflog even if it otherwise wouldn't be created,
/// as is the case for tags.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
pub enum UpdateMode {
    /// As symbolic references only ever see this when you want to detach them, we won't try to dereference them
    /// in this case and apply the change to it directly.
    RefAndRefLogAndNoDeref {
        /// If set, update the reflog even if it otherwise wouldn't.
        create_unconditionally: bool,
    },
    /// Only update the reflog but require this to be a symbolic ref so the actual update can be performed on the
    /// referent.
    RefLogOnlyAndDeref {
        /// If set, update the reflog even if it otherwise wouldn't.
        create_unconditionally: bool,
    },
}
