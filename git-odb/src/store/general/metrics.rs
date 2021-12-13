use crate::general::store;
use crate::general::store::IndexAndPacks;
use std::sync::atomic::Ordering;

impl super::Store {
    pub fn metrics(&self) -> store::Metrics {
        let mut open_packs = 0;
        let mut open_indices = 0;
        let mut known_packs = 0;
        let mut known_indices = 0;
        let mut unused_slots = 0;

        for f in self.index.load().slot_indices.iter().map(|idx| &self.files[*idx]) {
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
                None => unused_slots += 1,
            }
        }

        store::Metrics {
            num_handles: self.num_handles_unstable.load(Ordering::Relaxed)
                + self.num_handles_stable.load(Ordering::Relaxed),
            num_refreshes: self.num_disk_state_consolidation.load(Ordering::Relaxed),
            open_packs,
            open_indices,
            known_indices,
            known_packs,
            unused_slots,
        }
    }
}
