use crate::{
    mutable::Target,
    store::file::Transaction,
    transaction::{Change, RefEdit, RefLog},
};

impl<'s> Transaction<'s> {
    /// Make all [prepared][Transaction::prepare()] permanent and return the performed edits which represent the current
    /// state of the affected refs in the ref store in that instant. Please note that the obtained edits may have been
    /// adjusted to contain more dependent edits or additional information.
    /// `committer` is used in the reflog.
    ///
    /// On error the transaction may have been performed partially, depending on the nature of the error, and no attempt to roll back
    /// partial changes is made.
    ///
    /// In this stage, we perform the following operations:
    ///
    /// * update the ref log
    /// * move updated refs into place
    /// * delete reflogs and empty parent directories
    /// * delete packed refs
    /// * delete their corresponding reference (if applicable)
    ///   along with empty parent directories
    ///
    /// Note that transactions will be prepared automatically as needed.
    pub fn commit(self, committer: &git_actor::Signature) -> Result<Vec<RefEdit>, Error> {
        match self.updates {
            Some(mut updates) => {
                // Perform updates first so live commits remain referenced
                for change in updates.iter_mut() {
                    assert!(!change.update.deref, "Deref mode is turned into splits and turned off");
                    match &change.update.change {
                        // reflog first, then reference
                        Change::Update { log, new, mode } => {
                            let lock = change.lock.take().expect("each ref is locked");
                            let (update_ref, update_reflog) = match log.mode {
                                RefLog::Only => (false, true),
                                RefLog::AndReference => (true, true),
                            };
                            if update_reflog {
                                match new {
                                    Target::Symbolic(_) => {} // no reflog for symref changes
                                    Target::Peeled(new_oid) => {
                                        let previous = mode.previous_oid().or(change.leaf_referent_previous_oid);
                                        let do_update = previous.as_ref().map_or(true, |previous| previous != new_oid);
                                        if do_update {
                                            self.store.reflog_create_or_append(
                                                &lock,
                                                previous,
                                                new_oid,
                                                committer,
                                                log.message.as_ref(),
                                                log.force_create_reflog,
                                            )?;
                                        }
                                    }
                                }
                            }
                            if update_ref {
                                if let Err(err) = lock.commit() {
                                    #[cfg(not(target_os = "windows"))]
                                    let special_kind = std::io::ErrorKind::Other;
                                    #[cfg(target_os = "windows")]
                                    let special_kind = std::io::ErrorKind::PermissionDenied;
                                    let err = if err.error.kind() == special_kind {
                                        git_tempfile::remove_dir::empty_depth_first(err.instance.resource_path())
                                            .map_err(|io_err| std::io::Error::new(std::io::ErrorKind::Other, io_err))
                                            .and_then(|_| err.instance.commit().map_err(|err| err.error))
                                            .err()
                                    } else {
                                        Some(err.error)
                                    };

                                    if let Some(err) = err {
                                        return Err(Error::LockCommit {
                                            err,
                                            full_name: change.name(),
                                        });
                                    }
                                };
                            }
                        }
                        Change::Delete { .. } => {}
                    }
                }

                let reflog_root = self.store.reflog_root();
                for change in updates.iter_mut() {
                    match &change.update.change {
                        Change::Update { .. } => {}
                        Change::Delete { .. } => {
                            // Reflog deletion happens first in case it fails a ref without log is less terrible than
                            // a log without a reference.
                            let reflog_path = self.store.reflog_path(change.update.name.borrow());
                            if let Err(err) = std::fs::remove_file(&reflog_path) {
                                if err.kind() != std::io::ErrorKind::NotFound {
                                    return Err(Error::DeleteReflog {
                                        err,
                                        full_name: change.name(),
                                    });
                                }
                            } else {
                                git_tempfile::remove_dir::empty_upward_until_boundary(
                                    reflog_path.parent().expect("never without parent"),
                                    &reflog_root,
                                )
                                .ok();
                            }
                        }
                    }
                }

                if let Some(t) = self.packed_transaction {
                    t.commit().map_err(Error::PackedTransactionCommit)?;
                }

                for change in updates.iter_mut() {
                    match &change.update.change {
                        Change::Update { .. } => {}
                        Change::Delete { log: mode, .. } => {
                            let lock = change.lock.take().expect("each ref is locked, even deletions");
                            if *mode == RefLog::AndReference {
                                let reference_path = self.store.reference_path(change.update.name.to_path().as_ref());
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
                Ok(updates.into_iter().map(|edit| edit.update).collect())
            }
            None => panic!("BUG: must call prepare before commit"),
        }
    }
}
mod error {
    use crate::store::{file, packed};
    use bstr::BString;
    use quick_error::quick_error;

    quick_error! {
        /// The error returned by various [`Transaction`][super::Transaction] methods.
        #[derive(Debug)]
        #[allow(missing_docs)]
        pub enum Error {
            PackedTransactionCommit(err: packed::transaction::commit::Error) {
                display("The packed-ref transaction could not be committed")
                source(err)
            }
            PreprocessingFailed(err: std::io::Error) {
                display("Edit preprocessing failed with error: {}", err.to_string())
                source(err)
            }
            LockCommit{err: std::io::Error, full_name: BString} {
                display("THe change for reference {} could not be committed", full_name)
                source(err)
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
        }
    }
}
pub use error::Error;
