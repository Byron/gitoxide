use std::sync::atomic::Ordering;
use std::{path::Path, sync::Arc};

use crate::store::{handle, types};

impl super::Store {
    /// If Ok(None) is returned, the pack-id was stale and referred to an unloaded pack or a pack which couldn't be
    /// loaded as its file didn't exist on disk anymore.
    /// If the oid is known, just load indices again to continue
    /// (objects rarely ever removed so should be present, maybe in another pack though),
    /// and redo the entire lookup for a valid pack id whose pack can probably be loaded next time.
    pub(crate) fn load_pack(
        &self,
        id: types::PackId,
        marker: types::SlotIndexMarker,
    ) -> std::io::Result<Option<Arc<git_pack::data::File>>> {
        let index = self.index.load();
        if index.generation != marker.generation {
            return Ok(None);
        }
        fn load_pack(
            path: &Path,
            id: types::PackId,
            hash_kind: git_hash::Kind,
        ) -> std::io::Result<Arc<git_pack::data::File>> {
            git_pack::data::File::at(path, hash_kind)
                .map(|mut pack| {
                    pack.id = id.to_intrinsic_pack_id();
                    Arc::new(pack)
                })
                .map_err(|err| match err {
                    git_pack::data::header::decode::Error::Io { source, .. } => source,
                    other => std::io::Error::new(std::io::ErrorKind::Other, other),
                })
        }

        let slot = &self.files[id.index];
        if slot.generation.load(Ordering::SeqCst) > marker.generation {
            // There is a disk consolidation in progress which just overwrote a slot that could be disposed with some other
            // pack, one we didn't intend to load.
            // Hope that when the caller returns/retries the new index is set so they can fetch it and retry.
            return Ok(None);
        }
        match id.multipack_index {
            None => {
                match &**slot.files.load() {
                    Some(types::IndexAndPacks::Index(bundle)) => {
                        match bundle.data.loaded() {
                            Some(pack) => Ok(Some(pack.clone())),
                            None => {
                                let _lock = slot.write.lock();
                                let mut files = slot.files.load_full();
                                let files_mut = Arc::make_mut(&mut files);
                                let pack = match files_mut {
                                    Some(types::IndexAndPacks::Index(bundle)) => bundle
                                        .data
                                        .load_with_recovery(|path| load_pack(path, id, self.hash_kind))?,
                                    Some(types::IndexAndPacks::MultiIndex(_)) => {
                                        // something changed between us getting the lock, trigger a complete index refresh.
                                        None
                                    }
                                    None => {
                                        unreachable!("BUG: must set this handle to be stable to avoid slots to be cleared/changed")
                                    }
                                };
                                slot.files.store(files);
                                Ok(pack)
                            }
                        }
                    }
                    // This can also happen if they use an old index into our new and refreshed data which might have a multi-index
                    // here.
                    Some(types::IndexAndPacks::MultiIndex(_)) => Ok(None),
                    None => {
                        unreachable!("BUG: must set this handle to be stable to avoid slots to be cleared/changed")
                    }
                }
            }
            Some(_multipack_id) => todo!("load from given multipack which needs additional lookup"),
        }
    }

    /// Similar to `.load_pack()`, but for entire indices, bypassing the index entirely and going solely by marker and id.
    /// Returns `None` if the index wasn't available anymore or could otherwise not be loaded, which can be considered a bug
    /// as we should always keep needed indices available.
    pub(crate) fn index_by_id(&self, id: types::PackId, marker: types::SlotIndexMarker) -> Option<handle::IndexLookup> {
        let slot = self.files.get(id.index)?;
        if slot.generation.load(Ordering::SeqCst) > marker.generation {
            // This means somebody just overwrote our trashed slot with a new (or about to be stored) index, which means the slot isn't
            // what we need it to be.
            // This shouldn't as we won't overwrite slots while handles need stable indices.
            return None;
        }
        let lookup = match (&**slot.files.load()).as_ref()? {
            types::IndexAndPacks::Index(bundle) => handle::SingleOrMultiIndex::Single {
                index: bundle.index.loaded()?.clone(),
                data: bundle.data.loaded().cloned(),
            },
            types::IndexAndPacks::MultiIndex(multi) => handle::SingleOrMultiIndex::Multi {
                index: multi.multi_index.loaded()?.clone(),
                data: multi.data.iter().map(|f| f.loaded().cloned()).collect(),
            },
        };
        handle::IndexLookup {
            id: id.index,
            file: lookup,
        }
        .into()
    }
}
