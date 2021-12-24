#![allow(missing_docs)]
use std::{
    cell::RefCell,
    ops::Deref,
    sync::{atomic::Ordering, Arc},
};

use git_features::threading::OwnShared;

use crate::store::{handle, types, RefreshMode};

pub(crate) mod multi_index {
    // TODO: remove this declaration and replace it with the actual type where it's used
    pub type File = git_pack::multi_index::File;
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

// pub enum SingleOrMultiIndexRef<'a> {}

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

    use crate::store::{handle, types};

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
                handle::SingleOrMultiIndex::Multi { index, .. } => {
                    // TODO: figure out if this could actually not be true all the time.
                    let pack_index = pack_id
                        .multipack_index
                        .expect("multi-pack index must be set if this is a multi-pack");
                    Box::new(index.iter().filter_map(move |e| {
                        (e.pack_index == pack_index as u32).then(|| git_pack::index::Entry {
                            oid: e.oid,
                            pack_offset: e.pack_offset,
                            crc32: None,
                        })
                    }))
                }
            })
        }

        pub(crate) fn pack(&mut self, pack_id: types::PackId) -> Option<&'_ mut Option<Arc<git_pack::data::File>>> {
            (self.id == pack_id.index).then(move || match &mut self.file {
                handle::SingleOrMultiIndex::Single { data, .. } => data,
                handle::SingleOrMultiIndex::Multi { data, .. } => {
                    // TODO: figure out if this could actually not be true all the time.
                    let pack_index = pack_id
                        .multipack_index
                        .expect("multi-pack index must be set if this is a multi-pack");
                    &mut data[pack_index]
                }
            })
        }

        /// Return true if the given object id exists in this index
        pub(crate) fn contains(&self, object_id: &oid) -> bool {
            match &self.file {
                handle::SingleOrMultiIndex::Single { index, .. } => index.lookup(object_id).is_some(),
                handle::SingleOrMultiIndex::Multi { index, .. } => index.lookup(object_id).is_some(),
            }
        }

        /// Return true if the given object id exists in this index
        pub(crate) fn oid_at_index(&self, entry_index: u32) -> &git_hash::oid {
            match &self.file {
                handle::SingleOrMultiIndex::Single { index, .. } => index.oid_at_index(entry_index),
                handle::SingleOrMultiIndex::Multi { index, .. } => index.oid_at_index(entry_index),
            }
        }

        /// Return the amount of objects contained in the index, essentially the number of object ids.
        pub(crate) fn num_objects(&self) -> u32 {
            match &self.file {
                handle::SingleOrMultiIndex::Single { index, .. } => index.num_objects(),
                handle::SingleOrMultiIndex::Multi { index, .. } => index.num_objects(),
            }
        }

        /// See if the oid is contained in this index, and return its full id for lookup possibly alongside its data file if already
        /// loaded.
        /// Also return the index itself as it's needed to resolve intra-pack ref-delta objects. They are a possibility even though
        /// they won't be used in practice as it's more efficient to store their offsets.
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
                handle::SingleOrMultiIndex::Multi { index, data } => index.lookup(object_id).map(move |idx| {
                    let (pack_index, pack_offset) = index.pack_offset_and_pack_id_at_index(idx);
                    let pack_index = pack_index as usize;
                    Outcome {
                        object_index: handle::IndexForObjectInPack {
                            pack_id: types::PackId {
                                index: id,
                                multipack_index: Some(pack_index),
                            },
                            pack_offset,
                        },
                        // index_file: &**index,
                        index_file: todo!("figure out how to pass different kinds of indices"),
                        pack: &mut data[pack_index],
                    }
                }),
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
        self.to_handle().into()
    }

    pub fn to_cache_arc(self: &Arc<Self>) -> crate::Cache<super::Handle<Arc<super::Store>>> {
        self.to_handle_arc().into()
    }

    pub fn to_handle(self: &OwnShared<Self>) -> super::Handle<OwnShared<super::Store>> {
        let token = self.register_handle();
        super::Handle {
            store: self.clone(),
            refresh_mode: RefreshMode::default(),
            token: Some(token),
            snapshot: RefCell::new(self.collect_snapshot()),
        }
    }

    pub fn to_handle_arc(self: &Arc<Self>) -> super::Handle<Arc<super::Store>> {
        let token = self.register_handle();
        super::Handle {
            store: self.clone(),
            refresh_mode: Default::default(),
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

    pub fn refresh_never(&mut self) {
        self.refresh_mode = RefreshMode::Never;
    }

    /// Return the current refresh mode.
    pub fn refresh_mode(&mut self) -> RefreshMode {
        self.refresh_mode
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
