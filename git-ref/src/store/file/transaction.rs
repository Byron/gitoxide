use crate::{
    mutable::Target,
    store::file,
    transaction::{Change, Create, RefEdit, RefEditsExt, RefLog},
};
use bstr::BString;
use std::io::Write;

#[derive(Debug)]
struct Edit {
    update: RefEdit,
    lock: Option<git_lock::Marker>,
    /// Set if this update is coming from a symbolic reference and used to make it appear like it is the one that is handled,
    /// instead of the referent reference.
    parent_index: Option<usize>,
    /// For symbolic refs, this is the previous OID to put into the reflog instead of our own previous value. It's the
    /// peeled value of the leaf referent.
    leaf_referent_previous_oid: Option<ObjectId>,
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
            })
            .or_else(|err| match err {
                Error::ReferenceDecode(_) => Ok(None),
                other => Err(other),
            });
        let lock = match &mut change.update.change {
            Change::Delete { previous, .. } => {
                let lock = git_lock::Marker::acquire_to_hold_resource(
                    store.ref_path(&relative_path),
                    lock_fail_mode,
                    Some(store.base.to_owned()),
                )
                .map_err(|err| Error::LockAcquire {
                    err,
                    full_name: "borrowchk wont allow change.name()".into(),
                })?;
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

                // Keep the previous value for the caller and ourselves. Maybe they want to keep a log of sorts.
                if let Some(existing) = existing_ref {
                    *previous = Some(existing.target().into());
                }

                lock
            }
            Change::Update {
                mode: previous, new, ..
            } => {
                let mut lock = git_lock::File::acquire_to_update_resource(
                    store.ref_path(&relative_path),
                    lock_fail_mode,
                    Some(store.base.to_owned()),
                )
                .map_err(|err| Error::LockAcquire {
                    err,
                    full_name: "borrowchk wont allow change.name()".into(),
                })?;

                let existing_ref = existing_ref?;
                match (&previous, &existing_ref) {
                    (Create::Only, Some(existing)) if existing.target() != new.borrow() => {
                        todo!("fail as we won't create the ref and it doesn't match our expected state")
                    }
                    (
                        Create::OrUpdate {
                            previous: Some(previous),
                        },
                        Some(existing),
                    ) => match previous {
                        Target::Peeled(oid) if oid.is_null() => {}
                        any_target if any_target.borrow() == existing.target() => {}
                        _target_mismatch => todo!("abort because existing ref didn't have the correct value"),
                    },
                    (
                        Create::OrUpdate {
                            previous: Some(_previous),
                        },
                        None,
                    ) => {
                        todo!("ref was supposed to have a given value or exist, but it did not")
                    }
                    (Create::Only | Create::OrUpdate { previous: None }, None | Some(_)) => {}
                }

                *previous = match existing_ref {
                    None => Create::Only,
                    Some(existing) => Create::OrUpdate {
                        previous: Some(existing.target().into()),
                    },
                };

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
                    .pre_process(self.store, |idx, update| Edit {
                        update,
                        lock: None,
                        parent_index: Some(idx),
                        leaf_referent_previous_oid: None,
                    })
                    .map_err(Error::PreprocessingFailed)?;

                for cid in 0..self.updates.len() {
                    let change = &mut self.updates[cid];
                    if let Err(err) = Self::lock_ref_and_apply_change(self.store, self.lock_fail_mode, change) {
                        let err = match err {
                            Error::LockAcquire { err, full_name: _bogus } => Error::LockAcquire {
                                err,
                                full_name: {
                                    let mut cursor = change.parent_index;
                                    let mut ref_name = change.name();
                                    while let Some(parent_idx) = cursor {
                                        let parent = &self.updates[parent_idx];
                                        if parent.parent_index.is_none() {
                                            ref_name = parent.name();
                                        } else {
                                            cursor = parent.parent_index;
                                        }
                                    }
                                    ref_name
                                },
                            },
                            other => other,
                        };
                        return Err(err);
                    };

                    // traverse parent chain from leaf/peeled ref and set the leaf previous oid accordingly
                    // to help with their reflog entries
                    if let (Some(crate::Target::Peeled(oid)), Some(parent_idx)) =
                        (change.update.change.previous_value(), change.parent_index)
                    {
                        let oid = oid.to_owned();
                        let mut parent_idx_cursor = Some(parent_idx);
                        while let Some(parent) = parent_idx_cursor.take().map(|idx| &mut self.updates[idx]) {
                            parent_idx_cursor = parent.parent_index;
                            parent.leaf_referent_previous_oid = Some(oid);
                        }
                    }
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
                    assert!(!change.update.deref, "Deref mode is turned into splits and turned off");
                    match &change.update.change {
                        // reflog first, then reference
                        Change::Update { log, new, mode } => {
                            let lock = change.lock.take().expect("each ref is locked");
                            match new {
                                Target::Symbolic(_) => {} // no reflog for symref changes
                                Target::Peeled(oid) => {
                                    self.store.create_or_append_reflog(
                                        &lock,
                                        mode.previous_oid().or(change.leaf_referent_previous_oid),
                                        oid,
                                        log,
                                    )?;
                                }
                            }
                            lock.commit()?;
                        }
                        Change::Delete { .. } => {}
                    }
                }

                for change in self.updates.iter_mut() {
                    match &change.update.change {
                        Change::Update { .. } => {}
                        Change::Delete { log: mode, .. } => {
                            let lock = change.lock.take().expect("each ref is locked, even deletions");
                            let (rm_reflog, rm_ref) = match mode {
                                RefLog::AndReference => (true, true),
                                RefLog::Only => (true, false),
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
pub enum State {
    /// The transaction was just created but isn't prepared yet.
    Open,
    /// The transaction is ready to be committed.
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
                    leaf_referent_previous_oid: None,
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
            PreprocessingFailed(err: std::io::Error) {
                display("Edit preprocessing failed with error: {}", err.to_string())
                source(err)
            }
            LockAcquire{err: git_lock::acquire::Error, full_name: BString} {
                display("A lock could not be obtained for reference {}", full_name)
                source(err)
            }
            Io(err: std::io::Error) {
                display("An IO error occurred while applying an edit")
                from()
                source(err)
            }
            DeleteReferenceMustExist { full_name: BString } {
                display("The reference '{}' for deletion did not exist or could not be parsed", full_name)
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
            CreateOrUpdateRefLog(err: file::log::create_or_update::Error) {
                display("The reflog could not be created or updated")
                from()
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
use git_hash::ObjectId;
