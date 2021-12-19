use std::sync::atomic::Ordering;

use crate::store::{types, types::IndexAndPacks};

impl super::Store {
    /// Return metrics collected in a racy fashion, giving an idea of what's currently going on in the store.
    ///
    /// Use this to decide whether a new instance should be created to get a chance at dropping all open handles.
    pub fn metrics(&self) -> types::Metrics {
        let mut open_packs = 0;
        let mut open_indices = 0;
        let mut known_packs = 0;
        let mut known_indices = 0;
        let mut unused_slots = 0;

        let index = self.index.load();
        for f in index.slot_indices.iter().map(|idx| &self.files[*idx]) {
            match &**f.files.load() {
                Some(IndexAndPacks::Index(bundle)) => {
                    if bundle.index.is_loaded() {
                        open_indices += 1;
                    }
                    known_indices += 1;
                    if bundle.data.is_loaded() {
                        open_packs += 1;
                    }
                    known_packs += 1;
                }
                Some(IndexAndPacks::MultiIndex(multi)) => {
                    if multi.multi_index.is_loaded() {
                        open_indices += 1;
                    }
                    known_indices += 1;
                    for pack in multi.data.iter() {
                        if pack.is_loaded() {
                            open_packs += 1;
                        }
                        known_packs += 1;
                    }
                }
                None => {}
            }
        }

        for slot in &self.files {
            if slot.files.load().is_none() {
                unused_slots += 1;
            }
        }

        types::Metrics {
            num_handles: self.num_handles_unstable.load(Ordering::Relaxed)
                + self.num_handles_stable.load(Ordering::Relaxed),
            num_refreshes: self.num_disk_state_consolidation.load(Ordering::Relaxed),
            open_packs,
            open_indices,
            known_indices,
            known_packs,
            unused_slots,
            loose_dbs: index.loose_dbs.len(),
        }
    }
}
