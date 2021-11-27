#![allow(dead_code)]

use git_features::threading::{get_ref_upgradeable, upgrade_ref_to_mut, MutableOnDemand};
use std::time::SystemTime;

use crate::file;

#[derive(Default)]
struct State {
    buffer: Option<crate::packed::Buffer>,
    modified: Option<SystemTime>,
}

/// A buffer with on-demand interior mutability that can update itself.
#[derive(Default)]
pub(crate) struct ModifiableBuffer {
    state: MutableOnDemand<State>,
}

impl ModifiableBuffer {
    pub fn assure_packed_refs_uptodate(&self, store: &file::Store) -> Result<(), crate::packed::buffer::open::Error> {
        let packed_refs_modified_time = || store.packed_refs_path().metadata().and_then(|m| m.modified()).ok();
        let state = get_ref_upgradeable(&self.state);
        if state.buffer.is_none() {
            let mut state = upgrade_ref_to_mut(state);
            state.buffer = store.packed_buffer()?;
            if state.buffer.is_some() {
                state.modified = packed_refs_modified_time();
            }
        } else {
            let recent_modification = packed_refs_modified_time();
            match (&state.modified, recent_modification) {
                (None, None) => {}
                (Some(_), None) => {
                    let mut state = upgrade_ref_to_mut(state);
                    state.buffer = None;
                    state.modified = None
                }
                (Some(cached_time), Some(modified_time)) => {
                    if *cached_time < modified_time {
                        let mut state = upgrade_ref_to_mut(state);
                        state.buffer = store.packed_buffer()?;
                        state.modified = Some(modified_time);
                    }
                }
                (None, Some(modified_time)) => {
                    let mut state = upgrade_ref_to_mut(state);
                    state.buffer = store.packed_buffer()?;
                    state.modified = Some(modified_time);
                }
            }
        }
        Ok(())
    }
}
