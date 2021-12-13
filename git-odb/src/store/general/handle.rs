use std::{
    ops::Deref,
    sync::{atomic::Ordering, Arc},
};

use git_features::threading::OwnShared;

use crate::general::store;

pub(crate) mod multi_index {
    // TODO: replace this one with an actual implementation of a multi-pack index.
    pub type File = ();
}

pub enum SingleOrMultiIndex {
    Single {
        index: Arc<git_pack::index::File>,
        data: Option<Arc<git_pack::data::File>>,
    },
    Multi {
        index: Arc<multi_index::File>,
        data: Vec<Option<Arc<git_pack::data::File>>>,
    },
}

pub struct IndexLookup {
    pub(crate) file: SingleOrMultiIndex,
    pub(crate) id: store::IndexId,
}

pub struct IndexForObjectInPack {
    /// The internal identifier of the pack itself, which either is referred to by an index or a multi-pack index.
    pack_id: store::PackId,
    /// The index of the object within the pack
    object_index_in_pack: u32,
}

pub(crate) mod index_lookup {
    use std::sync::Arc;

    use git_hash::oid;

    use crate::general::{handle, store};

    impl handle::IndexLookup {
        /// See if the oid is contained in this index, and return its full id for lookup possibly alongside its data file if already
        /// loaded.
        /// If it is not loaded, ask it to be loaded and put it into the returned mutable option for safe-keeping.
        fn lookup(
            &mut self,
            object_id: &oid,
        ) -> Option<(handle::IndexForObjectInPack, &mut Option<Arc<git_pack::data::File>>)> {
            let id = self.id;
            match &mut self.file {
                handle::SingleOrMultiIndex::Single { index, data } => {
                    index.lookup(object_id).map(|object_index_in_pack| {
                        (
                            handle::IndexForObjectInPack {
                                pack_id: store::PackId {
                                    index: id,
                                    multipack_index: None,
                                },
                                object_index_in_pack,
                            },
                            data,
                        )
                    })
                }
                handle::SingleOrMultiIndex::Multi { index, data } => {
                    todo!("find respective pack and return it as &mut Option<>")
                }
            }
        }
    }
}

pub(crate) enum Mode {
    DeletedPacksAreInaccessible,
    /// This mode signals that we should not unload packs even after they went missing.
    KeepDeletedPacksAvailable,
}

/// Handle registration
impl super::Store {
    pub(crate) fn register_handle(&self) -> Mode {
        self.num_handles_unstable.fetch_add(1, Ordering::Relaxed);
        Mode::DeletedPacksAreInaccessible
    }
    pub(crate) fn remove_handle(&self, mode: Mode) {
        match mode {
            Mode::KeepDeletedPacksAvailable => {
                let _lock = self.path.lock();
                self.num_handles_stable.fetch_sub(1, Ordering::SeqCst)
            }
            Mode::DeletedPacksAreInaccessible => self.num_handles_unstable.fetch_sub(1, Ordering::Relaxed),
        };
    }
    pub(crate) fn upgrade_handle(&self, mode: Mode) -> Mode {
        if let Mode::DeletedPacksAreInaccessible = mode {
            let _lock = self.path.lock();
            self.num_handles_stable.fetch_add(1, Ordering::SeqCst);
            self.num_handles_unstable.fetch_sub(1, Ordering::SeqCst);
        }
        Mode::KeepDeletedPacksAvailable
    }
}

/// Handle creation
impl super::Store {
    pub fn to_handle(
        self: &OwnShared<Self>,
        refresh_mode: crate::RefreshMode,
    ) -> super::Handle<OwnShared<super::Store>> {
        let token = self.register_handle();
        super::Handle {
            store: self.clone(),
            refresh_mode,
            token: Some(token),
            snapshot: self.collect_snapshot(),
        }
    }
}

impl<S> super::Handle<S>
where
    S: Deref<Target = super::Store> + Clone,
{
    /// Call once if pack ids are stored and later used for lookup, meaning they should always remain mapped and not be unloaded
    /// even if they disappear from disk.
    /// This must be called if there is a chance that git maintenance is happening while a pack is created.
    pub fn prevent_pack_unload(&mut self) {
        self.token = self.token.take().map(|token| self.store.upgrade_handle(token));
    }

    pub fn store(&self) -> &S::Target {
        &*self.store
    }
}

impl<S> Drop for super::Handle<S>
where
    S: Deref<Target = super::Store> + Clone,
{
    fn drop(&mut self) {
        if let Some(token) = self.token.take() {
            self.store.remove_handle(token)
        }
    }
}

impl<S> Clone for super::Handle<S>
where
    S: Deref<Target = super::Store> + Clone,
{
    fn clone(&self) -> Self {
        super::Handle {
            store: self.store.clone(),
            refresh_mode: self.refresh_mode,
            token: self.store.register_handle().into(),
            snapshot: self.store.collect_snapshot(),
        }
    }
}
