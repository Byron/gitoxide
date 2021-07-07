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
use crate::mutable::{FullName, Target};

/// A description of an edit to perform.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
pub enum Change {
    /// If previous is not `None`, the ref must exist and its `oid` must agree with the `previous`, and
    /// we function like `update`.
    /// Otherwise it functions as `create-or-update`.
    Update {
        /// How to treat the reference log.
        mode: RefLog,
        /// The previous value of the ref, which will be used to assure the ref is still in the known `previous` state before
        /// updating it. It will also be filled in automatically for use in the reflog, if applicable.
        previous: Option<Target>,
        /// The new state of the reference, either for updating an existing one or creating a new one.
        new: Target,
        /// If set, create a reflog even though it would otherwise not be the case as prohibited by general rules.
        /// Note that ref-log writing might be prohibited in the entire repository which is when this flag has no effect either.
        force_create_reflog: bool,
    },
    /// Delete a reference and optionally check if `previous` is its content.
    Delete {
        /// The previous state of the reference. If set, the reference is expected to exist and match the given value.
        /// If the value is a peeled null-id the reference is expected to exist but the value doesn't matter, neither peeled nor symbolic.
        ///
        /// If a previous ref existed, this value will be filled in automatically and can be accessed if the transaction was committed successfully.
        previous: Option<Target>,
        /// How to thread the reference log during deletion.
        mode: RefLog,
    },
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

mod ext {
    use crate::{
        transaction::{Change, RefEdit, RefLog, Target},
        RefStore,
    };
    use bstr::BString;

    /// An extension trait to perform commonly used operations on edits across different ref stores.
    pub trait RefEditsExt<T>
    where
        T: std::borrow::Borrow<RefEdit> + std::borrow::BorrowMut<RefEdit>,
    {
        /// Return true if each ref `name` has exactly one `edit` across multiple ref edits
        fn assure_one_name_has_one_edit(&self) -> Result<(), BString>;

        /// Split all symbolic refs into updates for the symbolic ref as well as all their referents if the `deref` flag is enabled.
        ///
        /// Note no action is performed if deref isn't specified.
        fn extend_with_splits_of_symbolic_refs(
            &mut self,
            store: &impl RefStore,
            make_entry: impl FnMut(usize, RefEdit) -> T,
        ) -> Result<(), std::io::Error>;

        /// All processing steps in one and in the correct order.
        ///
        /// Users call this to assure derefs are honored and duplicate checks are done.
        fn pre_process(
            &mut self,
            store: &impl RefStore,
            make_entry: impl FnMut(usize, RefEdit) -> T,
        ) -> Result<(), std::io::Error> {
            self.extend_with_splits_of_symbolic_refs(store, make_entry)?;
            self.assure_one_name_has_one_edit().map_err(|name| {
                std::io::Error::new(
                    std::io::ErrorKind::AlreadyExists,
                    format!("A reference named '{}' has multiple edits", name),
                )
            })
        }
    }

    impl<E> RefEditsExt<E> for Vec<E>
    where
        E: std::borrow::Borrow<RefEdit> + std::borrow::BorrowMut<RefEdit>,
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
            store: &impl RefStore,
            mut make_entry: impl FnMut(usize, RefEdit) -> E,
        ) -> Result<(), std::io::Error> {
            let mut new_edits = Vec::new();
            let mut first = 0;
            let mut round = 1;
            loop {
                for (eid, edit) in self[first..].iter_mut().enumerate().map(|(eid, v)| (eid + first, v)) {
                    let edit = edit.borrow_mut();
                    if !edit.deref {
                        continue;
                    };

                    match store.find_one_existing(edit.name.to_partial()).ok() {
                        Some(Target::Symbolic(referent)) => {
                            match &mut edit.change {
                                Change::Delete { previous, mode } => {
                                    new_edits.push(make_entry(
                                        eid,
                                        RefEdit {
                                            change: Change::Delete {
                                                previous: previous.clone(),
                                                mode: *mode,
                                            },
                                            name: referent,
                                            deref: true,
                                        },
                                    ));
                                    *mode = RefLog::Only;
                                }
                                Change::Update {
                                    mode,
                                    previous,
                                    new,
                                    force_create_reflog,
                                } => {
                                    new_edits.push(make_entry(
                                        eid,
                                        RefEdit {
                                            change: Change::Update {
                                                previous: previous.clone(),
                                                new: new.clone(),
                                                mode: *mode,
                                                force_create_reflog: *force_create_reflog,
                                            },
                                            name: referent,
                                            deref: true,
                                        },
                                    ));
                                    *mode = RefLog::Only;
                                }
                            };
                            edit.deref = false;
                        }
                        Some(Target::Peeled(_)) => {
                            edit.deref = false;
                        }
                        None => {}
                    }
                }
                if new_edits.is_empty() {
                    break Ok(());
                }
                if round == 5 {
                    break Err(std::io::Error::new(
                        std::io::ErrorKind::WouldBlock,
                        format!(
                            "Could not follow all splits after {} rounds, assuming reference cycle",
                            round
                        ),
                    ));
                }
                round += 1;
                first = self.len();

                self.extend(new_edits.drain(..));
            }
        }
    }
}
pub use ext::RefEditsExt;
