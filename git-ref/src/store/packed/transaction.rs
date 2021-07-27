#![allow(missing_docs)]

use crate::{
    store::packed,
    transaction::{Change, RefEdit},
};

/// Access and instantiation
impl packed::Transaction {
    /// Create an entirely new packfile using the given `lock` representing the resource to write.
    /// Note that it's up to the caller to assure a race cannot occur.
    pub(crate) fn new_empty(lock: git_lock::File) -> Self {
        packed::Transaction {
            buffer: None,
            edits: None,
            lock: Some(lock),
            closed_lock: None,
        }
    }

    pub(crate) fn new_from_pack_and_lock(buffer: packed::Buffer, lock: git_lock::File) -> Self {
        packed::Transaction {
            buffer: Some(buffer),
            edits: None,
            lock: Some(lock),
            closed_lock: None,
        }
    }
}

/// Access
impl packed::Transaction {
    /// Returns our packed buffer
    pub fn buffer(&self) -> Option<&packed::Buffer> {
        self.buffer.as_ref()
    }
}

/// Lifecycle
impl packed::Transaction {
    /// Prepare the transaction by checking all edits for applicability.
    pub fn prepare(mut self, edits: impl IntoIterator<Item = RefEdit>) -> Result<Self, prepare::Error> {
        match self.edits {
            None => {
                let mut edits: Vec<_> = edits.into_iter().collect();
                // Remove all edits which are deletions that aren't here in the first place
                let buffer = &self.buffer;
                edits.retain(|edit| {
                    if let Change::Delete { .. } = edit.change {
                        buffer
                            .as_ref()
                            .map_or(true, |b| b.find_existing(edit.name.borrow()).is_ok())
                    } else {
                        true
                    }
                });
                if edits.is_empty() {
                    self.closed_lock = self
                        .lock
                        .take()
                        .map(|l| l.close())
                        .transpose()
                        .map_err(prepare::Error::CloseLock)?;
                } else {
                    // TODO: check preconditions and requirements to some extend,
                }
                self.edits = Some(edits);
            }
            Some(_) => {
                panic!("BUG: cannot call prepare(…) more than once")
            }
        }
        Ok(self)
    }

    /// Commit the prepare transaction
    pub fn commit(self) -> Result<(Vec<RefEdit>, Option<packed::Buffer>), git_lock::commit::Error<git_lock::File>> {
        match self.edits {
            Some(edits) => {
                if edits.is_empty() {
                    Ok((edits, self.buffer))
                } else {
                    let _file = self.lock.expect("a write lock for applying changes");
                    todo!("actual packed ref commit")
                }
            }
            None => panic!("BUG: cannot call commit() before prepare(…)"),
        }
    }
}

impl packed::Buffer {
    /// Convert this buffer to be used as the basis for a transaction.
    pub fn into_transaction(
        self,
        lock_mode: git_lock::acquire::Fail,
    ) -> Result<packed::Transaction, git_lock::acquire::Error> {
        let lock = git_lock::File::acquire_to_update_resource(&self.path, lock_mode, None)?;
        Ok(packed::Transaction {
            buffer: Some(self),
            lock: Some(lock),
            closed_lock: None,
            edits: None,
        })
    }
}

///
pub mod prepare {
    use quick_error::quick_error;
    quick_error! {
        #[derive(Debug)]
        pub enum Error {
            CloseLock(err: std::io::Error) {
                display("Could not close a lock which won't ever be committed")
                source(err)
            }
        }
    }
}
