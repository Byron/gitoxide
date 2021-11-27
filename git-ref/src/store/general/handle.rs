use crate::{store, Namespace};
use git_features::threading::OwnShared;

impl crate::Store {
    /// Return a new handle which sees all references.
    pub fn to_handle(self: OwnShared<Self>) -> store::Handle {
        store::Handle {
            store: self.clone(),
            namespace: None,
        }
    }

    /// Return a new handle which is confined to a namespace, and creates all references in a namespace.
    pub fn to_handle_in_namespace(self: OwnShared<Self>, namespace: Namespace) -> store::Handle {
        store::Handle {
            store: self.clone(),
            namespace: namespace.into(),
        }
    }
}
