use crate::{
    mutable::Target,
    store::file,
    transaction::{Change, DeleteMode, RefEdit, RefEditsExt},
};
use bstr::BString;
use std::io::Write;

struct Edit {
    update: RefEdit,
    lock: Option<git_lock::Marker>,
    /// Set if this update is coming from a symbolic reference and used to make it appear like it is the one that is handled,
    /// instead of the referent reference.
    #[allow(dead_code)]
    parent_index: Option<usize>,
}

impl Edit {
    fn name(&self) -> BString {
        self.update.name.0.clone()
    }
}

impl std::borrow::Borrow<RefEdit> for Edit {
    fn borrow(&self) -> &RefEdit {
        &self.update
    }
}

impl std::borrow::BorrowMut<RefEdit> for Edit {
    fn borrow_mut(&mut self) -> &mut RefEdit {
        &mut self.update
    }
}
/// A transaction
pub struct Transaction<'a> {
    store: &'a file::Store,
    updates: Vec<Edit>,
    state: State,
    lock_fail_mode: git_lock::acquire::Fail,
}

impl<'a> Transaction<'a> {
    fn lock_ref_and_apply_change(
        store: &file::Store,
        lock_fail_mode: git_lock::acquire::Fail,
        change: &mut Edit,
    ) -> Result<(), Error> {
        assert!(
            change.lock.is_none(),
            "locks can only be acquired once and it's all or nothing"
        );

        let relative_path = change.update.name.to_path();
        let existing_ref = store
            .ref_contents(relative_path.as_ref())
            .map_err(Error::from)
            .and_then(|opt| {
                opt.map(|buf| file::Reference::try_from_path(store, relative_path.as_ref(), &buf).map_err(Error::from))
                    .transpose()
            });
        let lock = match &mut change.update.change {
            Change::Delete { previous, .. } => {
                let lock = git_lock::Marker::acquire_to_hold_resource(
                    store.ref_path(&relative_path),
                    lock_fail_mode,
                    Some(store.base.to_owned()),
                )?;
                let existing_ref = existing_ref?;
                match (&previous, &existing_ref) {
                    (None, None | Some(_)) => {}
                    (Some(_previous), None) => {
                        return Err(Error::DeleteReferenceMustExist {
                            full_name: change.name(),
                        })
                    }
                    (Some(previous), Some(existing)) => {
                        if !previous.is_null() && *previous != existing.target() {
                            let expected = previous.clone();
                            return Err(Error::DeleteReferenceOutOfDate {
                                full_name: change.name(),
                                expected,
                                actual: existing.target().to_owned(),
                            });
                        }
                    }
                }

                // Keep the previous value for the caller only. Maybe they want to keep a log of sorts.
                if let Some(existing) = existing_ref {
                    *previous = Some(existing.target().into());
                }

                lock
            }
            Change::Update { previous, new, .. } => {
                let mut lock = git_lock::File::acquire_to_update_resource(
                    store.ref_path(&relative_path),
                    lock_fail_mode,
                    Some(store.base.to_owned()),
                )?;

                match previous {
                    Some(_expected_target) => todo!("check previous value, if object id is not null"),
                    None => {
                        if let Some(reference) = existing_ref? {
                            *previous = Some(reference.target().into());
                        }
                    }
                }

                lock.with_mut(|file| match new {
                    Target::Peeled(oid) => file.write_all(oid.as_bytes()),
                    Target::Symbolic(name) => file.write_all(b"ref: ").and_then(|_| file.write_all(name.as_ref())),
                })?;

                lock.close()?
            }
        };
        change.lock = Some(lock);
        Ok(())
    }
}

impl<'a> Transaction<'a> {
    /// Discard the transaction and re-obtain the initial edits
    pub fn into_edits(self) -> Vec<RefEdit> {
        self.updates.into_iter().map(|e| e.update).collect()
    }

    /// Prepare for calling [`commit(…)`][Transaction::commit()] in a way that can be rolled back perfectly.
    ///
    /// If the operation succeeds, the transaction can be committed or dropped to cause a rollback automatically.
    /// Rollbacks happen automatically on failure and they tend to be perfect.
    /// This method is idempotent.
    pub fn prepare(mut self) -> Result<Self, Error> {
        Ok(match self.state {
            State::Prepared => self,
            State::Open => {
                self.updates
                    .assure_one_name_has_one_edit()
                    .map_err(|first_name| Error::DuplicateRefEdits { first_name })?;
                self.updates
                    .extend_with_splits_of_symbolic_refs(self.store, |update| Edit {
                        update,
                        lock: None,
                        parent_index: None,
                    })?;

                for change in self.updates.iter_mut() {
                    Self::lock_ref_and_apply_change(self.store, self.lock_fail_mode, change)?;
                }
                self.state = State::Prepared;
                self
            }
        })
    }

    /// Make all [prepared][Transaction::prepare()] permanent and return the performed edits which represent the current
    /// state of the affected refs in the ref store in that instant. Please note that the obtained edits may have been
    /// adjusted to contain more dependent edits or additional information.
    ///
    /// On error the transaction may have been performed partially, depending on the nature of the error, and no attempt to roll back
    /// partial changes is made.
    ///
    /// In this stage, we perform the following operations:
    ///
    /// * write the ref log
    /// * move updated refs into place
    /// * delete reflogs
    /// * delete their corresponding reference (if applicable)
    ///   along with empty parent directories
    ///
    /// Note that transactions will be prepared automatically as needed.
    pub fn commit(mut self) -> Result<Vec<RefEdit>, Error> {
        match self.state {
            State::Open => self.prepare()?.commit(),
            State::Prepared => {
                // Perform updates first so live commits remain referenced
                for change in self.updates.iter_mut() {
                    match &change.update.change {
                        Change::Update { mode, new, .. } => {
                            let lock = change.lock.take().expect("each ref is locked");
                            match (new, mode) {
                                (Target::Symbolic(_), _reflog_mode) => {} // skip any log for symbolic refs
                                _ => todo!("commit other reflog write cases"),
                            }
                            lock.commit()?
                        }
                        Change::Delete { .. } => {}
                    }
                }

                for change in self.updates.iter_mut() {
                    assert!(!change.update.deref, "Deref mode is turned into splits and turned off");
                    match &change.update.change {
                        Change::Update { .. } => {}
                        Change::Delete { mode, .. } => {
                            let lock = change.lock.take().expect("each ref is locked, even deletions");
                            let (rm_reflog, rm_ref) = match mode {
                                DeleteMode::RefAndRefLog => (true, true),
                                DeleteMode::RefLogOnly => (true, false),
                            };

                            // Reflog deletion happens first in case it fails a ref without log is less terrible than
                            // a log without a reference.
                            if rm_reflog {
                                let reflog_path = self.store.reflog_path(change.update.name.borrow());
                                if let Err(err) = std::fs::remove_file(reflog_path) {
                                    if err.kind() != std::io::ErrorKind::NotFound {
                                        return Err(Error::DeleteReflog {
                                            err,
                                            full_name: change.name(),
                                        });
                                    }
                                }
                            }
                            if rm_ref {
                                let reference_path = self.store.ref_path(change.update.name.to_path().as_ref());
                                if let Err(err) = std::fs::remove_file(reference_path) {
                                    if err.kind() != std::io::ErrorKind::NotFound {
                                        return Err(Error::DeleteReference {
                                            err,
                                            full_name: change.name(),
                                        });
                                    }
                                }
                            }
                            drop(lock); // allow deletion of empty leading directories
                        }
                    }
                }
                Ok(self.updates.into_iter().map(|edit| edit.update).collect())
            }
        }
    }
}

/// The state of a [`Transaction`]
#[allow(missing_docs)]
pub enum State {
    Open,
    Prepared,
}

/// Edits
impl file::Store {
    /// Open a transaction with the given `edits`, and determine how to fail if a `lock` cannot be obtained.
    pub fn transaction(
        &self,
        edits: impl IntoIterator<Item = RefEdit>,
        lock: git_lock::acquire::Fail,
    ) -> Transaction<'_> {
        Transaction {
            store: self,
            updates: edits
                .into_iter()
                .map(|update| Edit {
                    update,
                    lock: None,
                    parent_index: None,
                })
                .collect(),
            state: State::Open,
            lock_fail_mode: lock,
        }
    }
}

mod error {
    use crate::{mutable::Target, store::file};
    use bstr::BString;
    use quick_error::quick_error;

    quick_error! {
        /// The error returned by various [`Transaction`][super::Transaction] methods.
        #[derive(Debug)]
        #[allow(missing_docs)]
        pub enum Error {
            DuplicateRefEdits{ first_name: BString } {
                display("Only one edit per reference must be provided, the first duplicate was {:?}", first_name)
            }
            LockAcquire(err: git_lock::acquire::Error) {
                display("A lock could not be obtained for a resource")
                from()
                source(err)
            }
            Io(err: std::io::Error) {
                display("An IO error occurred while applying an edit")
                from()
                source(err)
            }
            DeleteReferenceMustExist { full_name: BString } {
                display("The reference '{}' for deletion did not exist", full_name)
            }
            DeleteReferenceOutOfDate { full_name: BString, expected: Target, actual: Target } {
                display("The reference '{}' should have content {}, actual content was {}", full_name, expected, actual)
            }
            DeleteReference{ full_name: BString, err: std::io::Error } {
                display("The reference '{}' could not be deleted", full_name)
                source(err)
            }
            DeleteReflog{ full_name: BString, err: std::io::Error } {
                display("The reflog of reference '{}' could not be deleted", full_name)
                source(err)
            }
            ReferenceDecode(err: file::reference::decode::Error) {
                display("Could not read reference")
                from()
                source(err)
            }
        }
    }
}
pub use error::Error;
