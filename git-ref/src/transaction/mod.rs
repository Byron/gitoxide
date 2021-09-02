//! **Transactions** are the only way make changes to the ref store in order to increase the chance of consistency in a multi-threaded
//! environment.
//!
//! Transactions currently allow to…
//!
//! * create or update reference
//! * delete references
//!
//! The following guarantees are made:
//!
//! * transactions are prepared which is when other writers are prevented from changing them
//!   - errors during preparations will cause a perfect rollback
//! * prepared transactions are committed to finalize the change
//!   - errors when committing while leave the ref store in an inconsistent, but operational state.
use bstr::BString;
use git_hash::ObjectId;

use crate::{FullName, Target};

/// A change to the reflog.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
pub struct LogChange {
    /// How to treat the reference log.
    pub mode: RefLog,
    /// If set, create a reflog even though it would otherwise not be the case as prohibited by general rules.
    /// Note that ref-log writing might be prohibited in the entire repository which is when this flag has no effect either.
    pub force_create_reflog: bool,
    /// The message to put into the reference log. It must be a single line, hence newlines are forbidden.
    /// The string can be empty to indicate there should be no message at all.
    pub message: BString,
}

impl Default for LogChange {
    fn default() -> Self {
        LogChange {
            mode: RefLog::AndReference,
            force_create_reflog: false,
            message: Default::default(),
        }
    }
}

/// A way to determine if a value should be created or created or updated. In the latter case the previous
/// value can be specified to indicate to what extend the previous value matters.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
pub enum Create {
    /// Create a ref only. This fails if the ref exists and does not match the desired new value.
    Only,
    /// Create or update the reference with the `previous` value being controlling how to deal with existing ref values.
    ///
    OrUpdate {
        /// Interpret…
        /// * `None` so that existing values do not matter at all. This is the mode that always creates or updates a reference to the
        ///   desired new value.
        /// * `Some(Target::Peeled(ObjectId::null_sha1())` so that the reference is required to exist even though its value doesn't matter.
        /// * `Some(value)` so that the reference is required to exist and have the given `value`.
        previous: Option<Target>,
    },
}

impl Create {
    pub(crate) fn previous_oid(&self) -> Option<ObjectId> {
        match self {
            Create::OrUpdate {
                previous: Some(Target::Peeled(oid)),
            } => Some(*oid),
            Create::Only | Create::OrUpdate { .. } => None,
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
        /// The desired change to the reference log.
        log: LogChange,
        /// The create mode.
        /// If a ref was existing previously it will be updated to reflect the previous value for bookkeeping purposes
        /// and for use in the reflog.
        mode: Create,
        /// The new state of the reference, either for updating an existing one or creating a new one.
        new: Target,
    },
    /// Delete a reference and optionally check if `previous` is its content.
    Delete {
        /// The previous state of the reference. If set, the reference is expected to exist and match the given value.
        /// If the value is a peeled null-id the reference is expected to exist but the value doesn't matter, neither peeled nor symbolic.
        /// If `None`, the actual value does not matter.
        ///
        /// If a previous ref existed, this value will be filled in automatically and can be accessed
        /// if the transaction was committed successfully.
        previous: Option<Target>,
        /// How to thread the reference log during deletion.
        log: RefLog,
    },
}

impl Change {
    /// Return references to values that are in common between all variants.
    pub fn previous_value(&self) -> Option<crate::TargetRef<'_>> {
        match self {
            Change::Update { mode: Create::Only, .. } => None,
            Change::Update {
                mode: Create::OrUpdate { previous },
                ..
            }
            | Change::Delete { previous, .. } => previous.as_ref().map(|t| t.to_ref()),
        }
    }
}

/// A reference that is to be changed
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
pub struct RefEdit {
    /// The change itself
    pub change: Change,
    /// The name of the reference to apply the change to
    pub name: FullName,
    /// If set, symbolic references  identified by `name`  will be dereferenced to have the `change` applied to their target.
    /// This flag has no effect if the reference isn't symbolic.
    pub deref: bool,
}

/// The way to deal with the Reflog in deletions.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
pub enum RefLog {
    /// Delete or update the reference and the log
    AndReference,
    /// Delete or update only the reflog
    Only,
}

mod ext;
pub use ext::RefEditsExt;
