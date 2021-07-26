#![allow(missing_docs)]

use crate::{
    store::packed,
    transaction::{Change, RefEdit},
};

impl packed::Transaction {
    /// Returns our packed buffer
    pub fn buffer(&self) -> &packed::Buffer {
        &self.buffer
    }

    /// Prepare the transaction by checking all edits for applicability.
    pub fn prepare(mut self) -> Result<Self, prepare::Error> {
        // Remove all edits which are deletions that aren't here in the first place
        let buffer = &self.buffer;
        self.edits.retain(|edit| {
            if let Change::Delete { .. } = edit.change {
                buffer.find_existing(edit.name.borrow()).is_ok()
            } else {
                true
            }
        });
        // TODO: check preconditions and requirements to some extend,
        Ok(self)
    }

    /// Commit the prepare transaction
    pub fn commit(self) -> Result<(Vec<RefEdit>, packed::Buffer), git_lock::commit::Error<git_lock::File>> {
        // TODO: change transaction layout to allow detection of prepare/commit state
        if self.edits.is_empty() {
            Ok((self.edits, self.buffer))
        } else {
            todo!("actual packed ref commit")
        }
    }
}

impl packed::Buffer {
    /// Convert this buffer to be used as the basis for a transaction.
    pub fn into_transaction(
        self,
        edits: impl IntoIterator<Item = RefEdit>,
        lock_mode: git_lock::acquire::Fail,
    ) -> Result<packed::Transaction, git_lock::acquire::Error> {
        let lock = git_lock::File::acquire_to_update_resource(&self.path, lock_mode, None)?;
        Ok(packed::Transaction {
            buffer: self,
            lock,
            edits: edits.into_iter().collect(),
        })
    }
}

///
pub mod prepare {
    use quick_error::quick_error;
    quick_error! {
        #[derive(Debug)]
        pub enum Error {
            TBD
        }
    }
}
