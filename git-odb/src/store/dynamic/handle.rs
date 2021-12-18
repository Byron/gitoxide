use std::{
    cell::RefCell,
    ops::Deref,
    sync::{atomic::Ordering, Arc},
};

use git_features::threading::OwnShared;

use crate::dynamic::{handle, types};

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
    /// The index we were found at in the slot map
    pub(crate) id: types::IndexId,
}

pub struct IndexForObjectInPack {
    /// The internal identifier of the pack itself, which either is referred to by an index or a multi-pack index.
    pub(crate) pack_id: types::PackId,
    /// The offset at which the object's entry can be found
    pub(crate) pack_offset: u64,
}

pub(crate) mod index_lookup {
    use std::sync::Arc;

    use git_hash::oid;

    use crate::dynamic::{handle, types};

    pub(crate) struct Outcome<'a> {
        pub object_index: handle::IndexForObjectInPack,
        pub index_file: &'a git_pack::index::File,
        pub pack: &'a mut Option<Arc<git_pack::data::File>>,
    }

    impl handle::IndexLookup {
        /// Return an iterator over the entries of the given pack. The `pack_id` is only required to
        pub(crate) fn iter(
            &self,
            pack_id: types::PackId,
        ) -> Option<Box<dyn Iterator<Item = git_pack::index::Entry> + '_>> {
            (self.id == pack_id.index).then(|| match &self.file {
                handle::SingleOrMultiIndex::Single { index, .. } => index.iter(),
                handle::SingleOrMultiIndex::Multi { .. } => {
                    todo!("find respective pack and return it as &mut Option<>")
                }
            })
        }

        pub(crate) fn pack(&mut self, pack_id: types::PackId) -> Option<&'_ mut Option<Arc<git_pack::data::File>>> {
            (self.id == pack_id.index).then(move || match &mut self.file {
                handle::SingleOrMultiIndex::Single { data, .. } => data,
                handle::SingleOrMultiIndex::Multi { .. } => {
                    todo!("find respective pack and return it as &mut Option<>")
                }
            })
        }

        /// Return true if the given object id exists in this index
        pub(crate) fn contains(&self, object_id: &oid) -> bool {
            match &self.file {
                handle::SingleOrMultiIndex::Single { index, .. } => index.lookup(object_id).is_some(),
                handle::SingleOrMultiIndex::Multi { .. } => {
                    todo!("find respective pack and return it as &mut Option<>")
                }
            }
        }

        /// Return true if the given object id exists in this index
        pub(crate) fn oid_at_index(&self, entry_index: u32) -> &git_hash::oid {
            match &self.file {
                handle::SingleOrMultiIndex::Single { index, .. } => index.oid_at_index(entry_index),
                handle::SingleOrMultiIndex::Multi { .. } => {
                    todo!("find respective pack and return it as &mut Option<>")
                }
            }
        }

        /// Return the amount of objects contained in the index, essentially the number of object ids.
        pub(crate) fn num_objects(&self) -> u32 {
            match &self.file {
                handle::SingleOrMultiIndex::Single { index, .. } => index.num_objects(),
                handle::SingleOrMultiIndex::Multi { .. } => {
                    todo!("num_objects() on multi-index")
                }
            }
        }

        /// See if the oid is contained in this index, and return its full id for lookup possibly alongside its data file if already
        /// loaded.
        /// If it is not loaded, ask it to be loaded and put it into the returned mutable option for safe-keeping.
        pub(crate) fn lookup(&mut self, object_id: &oid) -> Option<Outcome<'_>> {
            let id = self.id;
            match &mut self.file {
                handle::SingleOrMultiIndex::Single { index, data } => index.lookup(object_id).map(move |idx| Outcome {
                    object_index: handle::IndexForObjectInPack {
                        pack_id: types::PackId {
                            index: id,
                            multipack_index: None,
                        },
                        pack_offset: index.pack_offset_at_index(idx),
                    },
                    index_file: &**index,
                    pack: data,
                }),
                handle::SingleOrMultiIndex::Multi { index: _, data: _ } => {
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
                let _lock = self.write.lock();
                self.num_handles_stable.fetch_sub(1, Ordering::SeqCst)
            }
            Mode::DeletedPacksAreInaccessible => self.num_handles_unstable.fetch_sub(1, Ordering::Relaxed),
        };
    }
    pub(crate) fn upgrade_handle(&self, mode: Mode) -> Mode {
        if let Mode::DeletedPacksAreInaccessible = mode {
            let _lock = self.write.lock();
            self.num_handles_stable.fetch_add(1, Ordering::SeqCst);
            self.num_handles_unstable.fetch_sub(1, Ordering::SeqCst);
        }
        Mode::KeepDeletedPacksAvailable
    }
}

/// Handle creation
impl super::Store {
    pub fn to_cache(self: &OwnShared<Self>) -> crate::Cache<super::Handle<OwnShared<super::Store>>> {
        self.to_handle(crate::RefreshMode::AfterAllIndicesLoaded).into()
    }

    pub fn to_cache_arc(self: &Arc<Self>) -> crate::Cache<super::Handle<Arc<super::Store>>> {
        self.to_handle_arc(crate::RefreshMode::AfterAllIndicesLoaded).into()
    }

    pub fn to_handle(
        self: &OwnShared<Self>,
        refresh_mode: crate::RefreshMode,
    ) -> super::Handle<OwnShared<super::Store>> {
        let token = self.register_handle();
        super::Handle {
            store: self.clone(),
            refresh_mode,
            token: Some(token),
            snapshot: RefCell::new(self.collect_snapshot()),
        }
    }

    pub fn to_handle_arc(self: &Arc<Self>, refresh_mode: crate::RefreshMode) -> super::Handle<Arc<super::Store>> {
        let token = self.register_handle();
        super::Handle {
            store: self.clone(),
            refresh_mode,
            token: Some(token),
            snapshot: RefCell::new(self.collect_snapshot()),
        }
    }

    /// Transform the only instance into an `Arc<Self>` or panic if this is not the only Rc handle
    /// to the contained store.
    ///
    /// This is meant to be used when the `git_features::threading::OwnShared` refers to an `Rc` as it was compiled without the
    /// `parallel` feature toggle.
    pub fn into_shared_arc(self: OwnShared<Self>) -> Arc<Self> {
        match OwnShared::try_unwrap(self) {
            Ok(this) => Arc::new(this),
            Err(_) => panic!("BUG: Must be called when there is only one owner for this RC"),
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

    pub fn store_owned(&self) -> S {
        self.store.clone()
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
            token: {
                let token = self.store.register_handle();
                match self.token.as_ref().expect("token is always set here ") {
                    handle::Mode::DeletedPacksAreInaccessible => token,
                    handle::Mode::KeepDeletedPacksAvailable => self.store.upgrade_handle(token),
                }
                .into()
            },
            snapshot: RefCell::new(self.store.collect_snapshot()),
        }
    }
}
