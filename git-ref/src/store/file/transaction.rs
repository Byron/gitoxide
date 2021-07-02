#![allow(unused)]

use crate::{edit, store::file::Store};

mod error {
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
        }
    }
}

use crate::edit::{RefEdit, RefEditsExt};
pub use error::Error;

struct Edit {
    update: RefEdit,
    lock: Option<git_lock::Marker>,
    /// Set if this update is coming from a symbolic reference and used to make it appear like it is the one that is handled,
    /// instead of the referent reference.
    parent_index: Option<usize>,
}

impl std::borrow::Borrow<RefEdit> for Edit {
    fn borrow(&self) -> &RefEdit {
        &self.update
    }
}

/// A transaction
pub struct Transaction {
    updates: Vec<Edit>,
    state: State,
}

impl Transaction {
    fn lock_ref_and_write_change(edit: &mut Edit) -> Result<(), Error> {
        todo!("lock and write")
    }
}

impl Transaction {
    /// Discard the transaction and re-obtain the initial edits
    pub fn into_edits(self) -> Vec<RefEdit> {
        self.updates.into_iter().map(|e| e.update).collect()
    }

    /// Prepare for calling [`commit(…)`][Transaction::commit()] in a way that can be rolled back perfectly.
    ///
    /// If the operation succeeds, the transaction can be committed or dropped to cause a rollback automatically.
    /// Rollbacks happen automatically on failure.
    /// This method is idempotent.
    pub fn prepare(mut self) -> Result<Self, Error> {
        Ok(match self.state {
            State::Prepared => self,
            State::Open => {
                self.updates
                    .assure_one_name_has_one_edit()
                    .map_err(|first_name| Error::DuplicateRefEdits { first_name })?;

                for edit in self.updates.iter_mut() {
                    Self::lock_ref_and_write_change(edit)?;
                }
                self.state = State::Prepared;
                todo!("transaction prep")
            }
        })
    }

    /// Make all [prepared][Transaction::prepare()] permanent and return the performed edits which represent the current
    /// state of the affected refs in the ref store in that instant. Please note that the obtained edits may have been
    /// adjusted to contain more dependent edits or additional information.
    ///
    /// On error the transaction may have been performed partially and can be retried, depending on the nature of the error.
    ///
    /// Note that transactions will be prepared automatically as needed.
    pub fn commit(mut self) -> Result<Vec<RefEdit>, Error> {
        match self.state {
            State::Open => self.prepare()?.commit(),
            State::Prepared => {
                todo!("transaction commit")
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
impl Store {
    /// Open a transaction with the given `edits`, and determine how to fail if a `lock` cannot be obtained.
    pub fn transaction(&self, edits: impl IntoIterator<Item = RefEdit>, lock: git_lock::acquire::Fail) -> Transaction {
        Transaction {
            updates: edits
                .into_iter()
                .map(|update| Edit {
                    update,
                    lock: None,
                    parent_index: None,
                })
                .collect(),
            state: State::Open,
        }
    }
}
