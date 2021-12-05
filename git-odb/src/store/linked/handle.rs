use std::sync::Arc;

use git_features::threading::OwnShared;

use crate::{linked, Handle};

impl linked::Store {
    /// Create a store handle with a shared reference to this instance.
    pub fn to_handle(&self) -> Handle<&Self> {
        self.into()
    }

    /// Create a store handle with a shared reference to this instance, if this instance is kept in an Arc.
    pub fn to_handle_arc(self: &Arc<Self>) -> Handle<Arc<Self>> {
        Arc::clone(self).into()
    }

    /// Create a store handle with a shared reference to this instance, if this instance is kept in an Rc/Arc.
    ///
    /// The latter depends on the `git-features/parallel` feature toggle.
    pub fn to_handle_shared(self: &OwnShared<Self>) -> Handle<OwnShared<Self>> {
        OwnShared::clone(self).into()
    }
}
