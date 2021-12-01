use git_features::threading::OwnShared;
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
    pub fn cached_packed_buffer(&self) -> Result<Option<OwnShared<packed::Buffer>>, packed::buffer::open::Error> {
        self.assure_packed_refs_uptodate()
    }

    /// Return the path at which packed-refs would usually be stored
    pub fn packed_refs_path(&self) -> PathBuf {
        self.base.join("packed-refs")
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

pub(crate) mod modifiable {
    use std::time::SystemTime;

    use git_features::threading::{get_mut, get_ref, OwnShared};

    use crate::{file, packed};

    #[derive(Debug, Default)]
    pub(crate) struct State {
        buffer: Option<OwnShared<packed::Buffer>>,
        modified: Option<SystemTime>,
    }

    impl file::Store {
        /// Always reload the internally cached packed buffer from disk. This can be necessary if the caller knows something changed
        /// but fears the change is not picked up due to lack of precision in fstat mtime calls.
        pub(crate) fn force_refresh_packed_buffer(&self) -> Result<(), packed::buffer::open::Error> {
            let mut state = get_mut(&self.packed);
            state.buffer = self.open_packed_buffer()?.map(OwnShared::new);
            Ok(())
        }
        pub(crate) fn assure_packed_refs_uptodate(
            &self,
        ) -> Result<Option<OwnShared<packed::Buffer>>, packed::buffer::open::Error> {
            let packed_refs_modified_time = || self.packed_refs_path().metadata().and_then(|m| m.modified()).ok();
            let state = get_ref(&self.packed);
            let buffer = if state.buffer.is_none() {
                drop(state);
                let mut state = get_mut(&self.packed);
                state.buffer = self.open_packed_buffer()?.map(OwnShared::new);
                if state.buffer.is_some() {
                    state.modified = packed_refs_modified_time();
                }
                state.buffer.clone()
            } else {
                let recent_modification = packed_refs_modified_time();
                match (&state.modified, recent_modification) {
                    (None, None) => state.buffer.clone(),
                    (Some(_), None) => {
                        drop(state);
                        let mut state = get_mut(&self.packed);
                        state.buffer = None;
                        state.modified = None;
                        state.buffer.clone()
                    }
                    (Some(cached_time), Some(modified_time)) => {
                        if *cached_time < modified_time {
                            drop(state);
                            let mut state = get_mut(&self.packed);
                            state.buffer = self.open_packed_buffer()?.map(OwnShared::new);
                            state.modified = Some(modified_time);
                            state.buffer.clone()
                        } else {
                            // Note that this relies on sub-section precision or else is a race when the packed file was just changed.
                            // It's nothing we can know though, so… up to the caller unfortunately.
                            state.buffer.clone()
                        }
                    }
                    (None, Some(modified_time)) => {
                        drop(state);
                        let mut state = get_mut(&self.packed);
                        state.buffer = self.open_packed_buffer()?.map(OwnShared::new);
                        state.modified = Some(modified_time);
                        state.buffer.clone()
                    }
                }
            };
            Ok(buffer)
        }
    }
}
