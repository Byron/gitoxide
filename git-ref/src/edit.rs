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

use crate::mutable::{Target, ValidName};
use bstr::BString;

/// Update an existing or a new reference.
pub struct Update {
    /// How to treat the reference log.
    pub mode: Reflog,
    /// The previous value of the ref, which will be used to assure the ref is still in the known `previous` state before
    /// updating it.
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
    pub name: ValidName,
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
