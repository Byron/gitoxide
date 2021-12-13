use std::{
    path::PathBuf,
    sync::{atomic::Ordering, Arc},
};

use crate::{
    general::{handle, store, store::StateId},
    RefreshMode,
};

pub(crate) enum Outcome {
    /// Drop all data and fully replace it with `indices`.
    /// This happens if we have witnessed a generational change invalidating all of our ids and causing currently loaded
    /// indices and maps to be dropped.
    Replace(Snapshot),
    /// Despite all values being full copies, indices are still compatible to what was before. This also means
    /// the caller can continue searching the added indices and loose-dbs.
    /// Or in other words, new indices were only added to the known list, and what was seen before is known not to have changed.
    /// Besides that, the full internal state can be replaced as with `Replace`.
    ReplaceStable(Snapshot),
}

pub(crate) struct Snapshot {
    /// Indices ready for object lookup or contains checks, ordered usually by modification data, recent ones first.
    pub(crate) indices: Vec<handle::IndexLookup>,
    /// A set of loose objects dbs to search once packed objects weren't found.
    pub(crate) loose_dbs: Arc<Vec<crate::loose::Store>>,
    /// remember what this state represents and to compare to other states.
    pub(crate) marker: store::SlotIndexMarker,
}

impl super::Store {
    /// If `None` is returned, there is new indices and the caller should give up. This is a possibility even if it's allowed to refresh
    /// as here might be no change to pick up.
    pub(crate) fn load_next_indices(
        &self,
        refresh_mode: RefreshMode,
        marker: &store::SlotIndexMarker,
    ) -> std::io::Result<Option<Outcome>> {
        let index = self.index.load();
        let state_id = index.state_id();
        if !index.is_initialized() {
            // TODO: figure out what kind of refreshes we need. This one loads in the initial slot map, but I think this cost is paid
            //       in full during instantiation.
            return self.consolidate_with_disk_state();
        }

        let outcome = {
            if marker.generation != index.generation {
                self.collect_replace_outcome(false /*stable*/)
            } else if marker.state_id == index.state_id() {
                // always compare to the latest state
                // Nothing changed in the mean time, try to load another index…

                // …and if that didn't yield anything new consider refreshing our disk state.
                match refresh_mode {
                    RefreshMode::Never => return Ok(None),
                    RefreshMode::AfterAllIndicesLoaded => return self.consolidate_with_disk_state(),
                }
            } else {
                self.collect_replace_outcome(true /*stable*/)
            }
        };
        Ok(Some(outcome))
    }

    /// refresh and possibly clear out our existing data structures, causing all pack ids to be invalidated.
    fn consolidate_with_disk_state(&self) -> std::io::Result<Option<Outcome>> {
        let index = self.index.load();
        let index_state = Arc::as_ptr(&index) as usize;
        let previous_generation = index.generation;

        // IMPORTANT: get a lock after we recorded the previous state.
        let objects_directory = self.path.lock();

        // Now we know the index isn't going to change anymore, even though threads might still load indices in the meantime.
        let index = self.index.load();
        if index_state != Arc::as_ptr(&index) as usize {
            // Someone else took the look before and changed the index. Return it without doing any additional work.
            return Ok(Some(
                self.collect_replace_outcome(index.generation == previous_generation),
            ));
        }

        let was_uninitialized = index.is_initialized();
        let needs_stable_indices = self.maintain_stable_indices(&objects_directory);

        self.num_disk_state_consolidation.fetch_add(1, Ordering::Relaxed);
        let mut db_paths = crate::alternate::resolve(&*objects_directory)
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?;
        // These are in addition to our objects directory
        db_paths.insert(0, objects_directory.clone());
        todo!("consolidate")
    }

    /// Stability means that indices returned by this API will remain valid.
    /// Without that constraint, we may unload unused packs and indices, and may rebuild the slotmap index.
    ///
    /// Note that this must be called with a lock to the relevant state held to assure these values don't change while
    /// we are working on said index.
    fn maintain_stable_indices(&self, _guard: &parking_lot::MutexGuard<'_, PathBuf>) -> bool {
        self.num_handles_stable.load(Ordering::SeqCst) == 0
    }

    pub(crate) fn collect_snapshot(&self) -> Snapshot {
        let index = self.index.load();
        let indices = if index.is_initialized() {
            index
                .slot_indices
                .iter()
                .map(|idx| (*idx, &self.files[*idx]))
                .filter_map(|(id, file)| {
                    let lookup = match (&**file.files.load()).as_ref()? {
                        store::IndexAndPacks::Index(bundle) => handle::SingleOrMultiIndex::Single {
                            index: bundle.index.loaded()?.clone(),
                            data: bundle.data.loaded().cloned(),
                        },
                        store::IndexAndPacks::MultiIndex(multi) => handle::SingleOrMultiIndex::Multi {
                            index: multi.multi_index.loaded()?.clone(),
                            data: multi.data.iter().map(|f| f.loaded().cloned()).collect(),
                        },
                    };
                    handle::IndexLookup { file: lookup, id }.into()
                })
                .collect()
        } else {
            Vec::new()
        };

        Snapshot {
            indices,
            loose_dbs: Arc::clone(&index.loose_dbs),
            marker: index.marker(),
        }
    }

    fn collect_replace_outcome(&self, is_stable: bool) -> Outcome {
        let snapshot = self.collect_snapshot();
        if is_stable {
            Outcome::ReplaceStable(snapshot)
        } else {
            Outcome::Replace(snapshot)
        }
    }
}
