use std::path::PathBuf;

use crate::store::{file, packed};

impl file::Store {
    /// Return a packed transaction ready to receive updates. Use this to create or update `packed-refs`.
    /// Note that if you already have a [`packed::Buffer`] then use its [`packed::Buffer::into_transaction()`] method instead.
    #[allow(unused)] // TODO: use this in related purge functionality.
    pub(crate) fn packed_transaction(
        &self,
        lock_mode: git_lock::acquire::Fail,
    ) -> Result<packed::Transaction, transaction::Error> {
        let lock = git_lock::File::acquire_to_update_resource(self.packed_refs_path(), lock_mode, None)?;
        Ok(match self.packed()? {
            Some(packed) => packed::Transaction::new_from_pack_and_lock(packed, lock),
            None => packed::Transaction::new_empty(lock),
        })
    }

    /// Return a buffer for the packed file
    pub fn packed(&self) -> Result<Option<packed::Buffer>, packed::buffer::open::Error> {
        match packed::Buffer::open(self.packed_refs_path(), 32 * 1024) {
            Ok(buf) => Ok(Some(buf)),
            Err(packed::buffer::open::Error::Io(err)) if err.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(err),
        }
    }

    /// Return the path at which packed-refs would usually be stored
    pub fn packed_refs_path(&self) -> PathBuf {
        self.base.join("packed-refs")
    }
}

///
pub mod transaction {
    use crate::store::packed;
    use quick_error::quick_error;

    quick_error! {
        /// The error returned by [`file::Store::packed_transaction`][crate::file::Store::packed_transaction()].
        #[derive(Debug)]
        #[allow(missing_docs)]
        pub enum Error {
            BufferOpen(err: packed::buffer::open::Error) {
                display("An existing pack couldn't be opened or read when preparing a transaction")
                source(err)
                from()
            }
            TransactionLock(err: git_lock::acquire::Error) {
                display("The lock for a packed transaction could not be obtained")
                source(err)
                from()
            }
        }
    }
}
