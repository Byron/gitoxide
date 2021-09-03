use crate::{
    packed,
    store::{
        file,
        file::{
            loose,
            transaction::{Edit, PackedRefs},
            Transaction,
        },
    },
    transaction::{Change, LogChange, RefEdit, RefEditsExt, RefLog},
    Target,
};

impl<'s> Transaction<'s> {
    fn lock_ref_and_apply_change(
        store: &file::Store,
        lock_fail_mode: git_lock::acquire::Fail,
        packed: Option<&packed::Buffer>,
        change: &mut Edit,
    ) -> Result<(), Error> {
        use std::io::Write;
        assert!(
            change.lock.is_none(),
            "locks can only be acquired once and it's all or nothing"
        );

        let relative_path = change.update.name.to_path();
        let existing_ref = store
            .ref_contents(relative_path.as_ref())
            .map_err(Error::from)
            .and_then(|maybe_loose| {
                maybe_loose
                    .map(|buf| {
                        loose::Reference::try_from_path(change.update.name.clone(), &buf)
                            .map(file::Reference::Loose)
                            .map_err(Error::from)
                    })
                    .transpose()
            })
            .or_else(|err| match err {
                Error::ReferenceDecode(_) => Ok(None),
                other => Err(other),
            })
            .and_then(|maybe_loose| match (maybe_loose, packed) {
                (None, Some(packed)) => packed
                    .try_find(change.update.name.to_ref())
                    .map(|opt| opt.map(file::Reference::Packed))
                    .map_err(Error::from),
                (None, None) => Ok(None),
                (maybe_loose, _) => Ok(maybe_loose),
            });
        let lock = match &mut change.update.change {
            Change::Delete { previous, .. } => {
                let lock = git_lock::Marker::acquire_to_hold_resource(
                    store.reference_path(&relative_path),
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
                        let actual = existing.target();
                        if !previous.is_null() && *previous != actual {
                            let expected = previous.clone();
                            return Err(Error::ReferenceOutOfDate {
                                full_name: change.name(),
                                expected,
                                actual,
                            });
                        }
                    }
                }

                // Keep the previous value for the caller and ourselves. Maybe they want to keep a log of sorts.
                if let Some(existing) = existing_ref {
                    *previous = Some(existing.target());
                }

                lock
            }
            Change::Update { expected, new, .. } => {
                let mut lock = git_lock::File::acquire_to_update_resource(
                    store.reference_path(&relative_path),
                    lock_fail_mode,
                    Some(store.base.to_owned()),
                )
                .map_err(|err| Error::LockAcquire {
                    err,
                    full_name: "borrowchk wont allow change.name() and this will be corrected by caller".into(),
                })?;

                let existing_ref = existing_ref?;
                match (&expected, &existing_ref) {
                    (PreviousValue::Any, _)
                    | (PreviousValue::MustExist, Some(_))
                    | (PreviousValue::MustNotExist | PreviousValue::ExistingMustMatch(_), None) => {}
                    (PreviousValue::MustExist, None) => {
                        let expected = Target::Peeled(git_hash::ObjectId::null_sha1());
                        let full_name = change.name();
                        return Err(Error::MustExist { full_name, expected });
                    }
                    (PreviousValue::MustNotExist, Some(existing)) => {
                        if existing.target() != new.to_ref() {
                            let new = new.clone();
                            return Err(Error::MustNotExist {
                                full_name: change.name(),
                                actual: existing.target(),
                                new,
                            });
                        }
                    }
                    (
                        PreviousValue::MustExistAndMatch(previous) | PreviousValue::ExistingMustMatch(previous),
                        Some(existing),
                    ) => {
                        if *previous != existing.target() {
                            let actual = existing.target();
                            let expected = previous.to_owned();
                            let full_name = change.name();
                            return Err(Error::ReferenceOutOfDate {
                                full_name,
                                actual,
                                expected,
                            });
                        }
                    }

                    (PreviousValue::MustExistAndMatch(previous), None) => {
                        let expected = previous.to_owned();
                        let full_name = change.name();
                        return Err(Error::MustExist { full_name, expected });
                    }
                };

                if let Some(existing) = existing_ref {
                    *expected = PreviousValue::MustExistAndMatch(existing.target());
                };

                lock.with_mut(|file| match new {
                    Target::Peeled(oid) => write!(file, "{}", oid),
                    Target::Symbolic(name) => write!(file, "ref: {}", name.0),
                })?;

                lock.close()?
            }
        };
        change.lock = Some(lock);
        Ok(())
    }
}

impl<'s> Transaction<'s> {
    /// Prepare for calling [`commit(…)`][Transaction::commit()] in a way that can be rolled back perfectly.
    ///
    /// If the operation succeeds, the transaction can be committed or dropped to cause a rollback automatically.
    /// Rollbacks happen automatically on failure and they tend to be perfect.
    /// This method is idempotent.
    pub fn prepare(
        mut self,
        edits: impl IntoIterator<Item = RefEdit>,
        lock_fail_mode: git_lock::acquire::Fail,
    ) -> Result<Self, Error> {
        assert!(self.updates.is_none(), "BUG: Must not call prepare(…) multiple times");
        let store = self.store;
        let mut updates: Vec<_> = edits
            .into_iter()
            .map(|update| Edit {
                update,
                lock: None,
                parent_index: None,
                leaf_referent_previous_oid: None,
            })
            .collect();
        updates
            .pre_process(
                |name| {
                    let symbolic_refs_are_never_packed = None;
                    store
                        .find(name, symbolic_refs_are_never_packed)
                        .map(|r| r.into_target())
                        .ok()
                },
                |idx, update| Edit {
                    update,
                    lock: None,
                    parent_index: Some(idx),
                    leaf_referent_previous_oid: None,
                },
                self.namespace.take(),
            )
            .map_err(Error::PreprocessingFailed)?;

        let mut maybe_updates_for_packed_refs = match self.packed_refs {
            PackedRefs::DeletionsAndNonSymbolicUpdates(_)
            | PackedRefs::DeletionsAndNonSymbolicUpdatesRemoveLooseSourceReference(_) => Some(0_usize),
            PackedRefs::DeletionsOnly => None,
        };
        if maybe_updates_for_packed_refs.is_some() || self.store.packed_refs_path().is_file() {
            let mut edits_for_packed_transaction = Vec::<RefEdit>::new();
            let mut needs_packed_refs_lookups = false;
            for edit in updates.iter() {
                let log_mode = match edit.update.change {
                    Change::Update {
                        log: LogChange { mode, .. },
                        ..
                    } => mode,
                    Change::Delete { log, .. } => log,
                };
                if log_mode == RefLog::Only {
                    continue;
                }
                if let Some(ref mut num_updates) = maybe_updates_for_packed_refs {
                    if let Change::Update {
                        new: Target::Peeled(_), ..
                    } = edit.update.change
                    {
                        edits_for_packed_transaction.push(edit.update.clone());
                        *num_updates += 1;
                    }
                    continue;
                }
                match edit.update.change {
                    Change::Update {
                        expected: PreviousValue::ExistingMustMatch(_) | PreviousValue::MustExistAndMatch(_),
                        ..
                    } => needs_packed_refs_lookups = true,
                    Change::Delete { .. } => {
                        edits_for_packed_transaction.push(edit.update.clone());
                    }
                    _ => {
                        needs_packed_refs_lookups = true;
                    }
                }
            }

            if !edits_for_packed_transaction.is_empty() || needs_packed_refs_lookups {
                // What follows means that we will only create a transaction if we have to access packed refs for looking
                // up current ref values, or that we definitely have a transaction if we need to make updates. Otherwise
                // we may have no transaction at all which isn't required if we had none and would only try making deletions.
                let packed_transaction: Option<_> = if maybe_updates_for_packed_refs.unwrap_or(0) > 0 {
                    // We have to create a packed-ref even if it doesn't exist
                    self.store
                        .packed_transaction(lock_fail_mode)
                        .map_err(|err| match err {
                            file::packed::transaction::Error::BufferOpen(err) => Error::from(err),
                            file::packed::transaction::Error::TransactionLock(err) => {
                                Error::PackedTransactionAcquire(err)
                            }
                        })?
                        .into()
                } else {
                    // A packed transaction is optional - we only have deletions that can't be made if
                    // no packed-ref file exists anyway
                    self.store
                        .packed_buffer()?
                        .map(|p| {
                            p.into_transaction(lock_fail_mode)
                                .map_err(Error::PackedTransactionAcquire)
                        })
                        .transpose()?
                };
                if let Some(transaction) = packed_transaction {
                    self.packed_transaction = Some(match &mut self.packed_refs {
                        PackedRefs::DeletionsAndNonSymbolicUpdatesRemoveLooseSourceReference(f)
                        | PackedRefs::DeletionsAndNonSymbolicUpdates(f) => {
                            transaction.prepare(edits_for_packed_transaction, f)?
                        }
                        PackedRefs::DeletionsOnly => transaction
                            .prepare(edits_for_packed_transaction, &mut |_, _| {
                                unreachable!("BUG: deletions never trigger object lookups")
                            })?,
                    });
                }
            }
        }

        for cid in 0..updates.len() {
            let change = &mut updates[cid];
            if let Err(err) = Self::lock_ref_and_apply_change(
                self.store,
                lock_fail_mode,
                self.packed_transaction.as_ref().and_then(|t| t.buffer()),
                change,
            ) {
                let err = match err {
                    Error::LockAcquire { err, full_name: _bogus } => Error::LockAcquire {
                        err,
                        full_name: {
                            let mut cursor = change.parent_index;
                            let mut ref_name = change.name();
                            while let Some(parent_idx) = cursor {
                                let parent = &updates[parent_idx];
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
            if let (Some(crate::TargetRef::Peeled(oid)), Some(parent_idx)) =
                (change.update.change.previous_value(), change.parent_index)
            {
                let oid = oid.to_owned();
                let mut parent_idx_cursor = Some(parent_idx);
                while let Some(parent) = parent_idx_cursor.take().map(|idx| &mut updates[idx]) {
                    parent_idx_cursor = parent.parent_index;
                    parent.leaf_referent_previous_oid = Some(oid);
                }
            }
        }
        self.updates = Some(updates);
        Ok(self)
    }
}

mod error {
    use bstr::BString;
    use quick_error::quick_error;

    use crate::{
        store::{file, packed},
        Target,
    };

    quick_error! {
        /// The error returned by various [`Transaction`][super::Transaction] methods.
        #[derive(Debug)]
        #[allow(missing_docs)]
        pub enum Error {
            Packed(err: packed::buffer::open::Error) {
                display("The packed ref buffer could not be loaded")
                from()
                source(err)
            }
            PackedTransactionAcquire(err: git_lock::acquire::Error) {
                display("The lock for the packed-ref file could not be obtained")
                source(err)
            }
            PackedTransactionPrepare(err: packed::transaction::prepare::Error) {
                display("The packed transaction could not be prepared")
                from()
                source(err)
            }
            PackedFind(err: packed::find::Error) {
                display("The packed ref file could not be parsed")
                source(err)
                from()
            }
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
            MustNotExist { full_name: BString, actual: Target, new: Target } {
                display("Reference '{}' was not supposed to exist when writing it with value {}, but actual content was {}", full_name, new, actual)
            }
            MustExist { full_name: BString, expected: Target } {
                display("Reference '{}' was supposed to exist with value {}, but didn't.", full_name, expected)
            }
            ReferenceOutOfDate { full_name: BString, expected: Target, actual: Target } {
                display("The reference '{}' should have content {}, actual content was {}", full_name, expected, actual)
            }
            ReferenceDecode(err: file::loose::reference::decode::Error) {
                display("Could not read reference")
                from()
                source(err)
            }
        }
    }
}

pub use error::Error;

use crate::transaction::PreviousValue;
