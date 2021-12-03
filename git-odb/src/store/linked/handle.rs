#![allow(missing_docs)]

use crate::{linked, Handle};
use git_features::threading::OwnShared;
use std::sync::Arc;

impl linked::Store {
    pub fn to_handle(&self) -> Handle<&Self> {
        self.into()
    }

    pub fn to_handle_arc(self: &Arc<Self>) -> Handle<Arc<Self>> {
        Arc::clone(self).into()
    }

    pub fn to_handle_shared(self: &OwnShared<Self>) -> Handle<OwnShared<Self>> {
        OwnShared::clone(self).into()
    }
}
