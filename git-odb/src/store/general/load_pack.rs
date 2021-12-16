use std::sync::Arc;

use crate::general::store;

impl super::Store {
    /// If Ok(None) is returned, the pack-id was stale and referred to an unloaded pack or a pack which couldn't be
    /// loaded as its file didn't exist on disk anymore.
    /// If the oid is known, just load indices again to continue
    /// (objects rarely ever removed so should be present, maybe in another pack though),
    /// and redo the entire lookup for a valid pack id whose pack can probably be loaded next time.
    pub(crate) fn load_pack(
        &self,
        id: store::PackId,
        marker: store::SlotIndexMarker,
    ) -> std::io::Result<Option<Arc<git_pack::data::File>>> {
        let index = self.index.load();
        if index.generation != marker.generation {
            return Ok(None);
        }
        match id.multipack_index {
            None => {
                let f = &self.files[id.index];
                match &**f.files.load() {
                    Some(store::IndexAndPacks::Index(bundle)) => {
                        match bundle.data.loaded() {
                            Some(pack) => Ok(Some(pack.clone())),
                            None => {
                                let _lock = f.write.lock();
                                let mut files = f.files.load_full();
                                let files_mut = Arc::make_mut(&mut files);
                                let pack = match files_mut {
                                    Some(store::IndexAndPacks::Index(bundle)) => {
                                        bundle.data.load_with_recovery(|path| {
                                            git_pack::data::File::at(path).map(Arc::new).map_err(|err| match err {
                                                git_pack::data::header::decode::Error::Io { source, .. } => source,
                                                other => std::io::Error::new(std::io::ErrorKind::Other, other),
                                            })
                                        })?
                                    }
                                    Some(store::IndexAndPacks::MultiIndex(_)) => {
                                        // something changed between us getting the lock, trigger a complete index refresh.
                                        None
                                    }
                                    None => {
                                        unreachable!("BUG: must set this handle to be stable to avoid slots to be cleared/changed")
                                    }
                                };
                                f.files.store(files);
                                Ok(pack)
                            }
                        }
                    }
                    // This can also happen if they use an old index into our new and refreshed data which might have a multi-index
                    // here.
                    Some(store::IndexAndPacks::MultiIndex(_)) => Ok(None),
                    None => {
                        unreachable!("BUG: must set this handle to be stable to avoid slots to be cleared/changed")
                    }
                }
            }
            Some(_multipack_id) => todo!("load from given multipack which needs additional lookup"),
        }
    }
}
