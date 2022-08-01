use std::path::PathBuf;

use crate::store_impl::{file, packed};

impl file::Store {
    /// Return a packed transaction ready to receive updates. Use this to create or update `packed-refs`.
    /// Note that if you already have a [`packed::Buffer`] then use its [`packed::Buffer::into_transaction()`] method instead.
    pub(crate) fn packed_transaction(
        &self,
        lock_mode: git_lock::acquire::Fail,
    ) -> Result<packed::Transaction, transaction::Error> {
        let lock = git_lock::File::acquire_to_update_resource(self.packed_refs_path(), lock_mode, None)?;
        // We 'steal' the possibly existing packed buffer which may safe time if it's already there and fresh.
        // If nothing else is happening, nobody will get to see the soon stale buffer either, but if so, they will pay
        // for reloading it. That seems preferred over always loading up a new one.
        Ok(packed::Transaction::new_from_pack_and_lock(
            self.assure_packed_refs_uptodate()?,
            lock,
        ))
    }

    /// Try to open a new packed buffer. It's not an error if it doesn't exist, but yields `Ok(None)`.
    pub fn open_packed_buffer(&self) -> Result<Option<packed::Buffer>, packed::buffer::open::Error> {
        let need_more_than_this_many_bytes_to_use_mmap = 32 * 1024;
        match packed::Buffer::open(self.packed_refs_path(), need_more_than_this_many_bytes_to_use_mmap) {
            Ok(buf) => Ok(Some(buf)),
            Err(packed::buffer::open::Error::Io(err)) if err.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(err),
        }
    }

    /// Return a possibly cached packed buffer with shared ownership. At retrieval it will assure it's up to date, but
    /// after that it can be considered a snapshot as it cannot change anymore.
    ///
    /// Use this to make successive calls to [`file::Store::try_find_packed()`]
    /// or obtain iterators using [`file::Store::iter_packed()`] in a way that assures the packed-refs content won't change.
    pub fn cached_packed_buffer(&self) -> Result<Option<file::packed::SharedBuffer>, packed::buffer::open::Error> {
        self.assure_packed_refs_uptodate()
    }

    /// Return the path at which packed-refs would usually be stored
    pub fn packed_refs_path(&self) -> PathBuf {
        self.common_dir_resolved().join("packed-refs")
    }
}

///
pub mod transaction {
    use quick_error::quick_error;

    use crate::store_impl::packed;

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

#[allow(missing_docs)]
pub type SharedBuffer = git_features::fs::SharedSnapshot<packed::Buffer>;

pub(crate) mod modifiable {
    use git_features::threading::OwnShared;

    use crate::{file, packed};

    pub(crate) type MutableSharedBuffer = OwnShared<State>;
    type State = git_features::fs::MutableSnapshot<packed::Buffer>;

    impl file::Store {
        pub(crate) fn force_refresh_packed_buffer(&self) -> Result<(), packed::buffer::open::Error> {
            git_features::fs::Snapshot::force_refresh(&self.packed, || {
                let modified = self.packed_refs_path().metadata()?.modified()?;
                self.open_packed_buffer().map(|packed| Some(modified).zip(packed))
            })
        }
        pub(crate) fn assure_packed_refs_uptodate(
            &self,
        ) -> Result<Option<super::SharedBuffer>, packed::buffer::open::Error> {
            git_features::fs::Snapshot::recent_snapshot(
                &self.packed,
                || self.packed_refs_path().metadata().and_then(|m| m.modified()).ok(),
                || self.open_packed_buffer(),
            )
        }
    }
}
