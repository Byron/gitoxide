use crate::{store, Namespace};
use git_features::threading::OwnShared;

#[derive(Clone)]
pub(crate) enum State {
    Loose {
        store: crate::file::Store,
        packed_buffer: OwnShared<store::packed::ModifiableBuffer>,
    },
}

impl From<(&store::State, Option<Namespace>)> for State {
    fn from(s: (&store::State, Option<Namespace>)) -> Self {
        match s.0 {
            store::State::Loose {
                path,
                reflog_mode,
                packed_buffer,
            } => store::handle::State::Loose {
                store: {
                    let mut store = crate::file::Store::at(path, *reflog_mode);
                    store.namespace = s.1;
                    store
                },
                packed_buffer: packed_buffer.clone(),
            },
        }
    }
}

impl crate::Store {
    /// Return a new handle which sees all references if `namespace` is `None` or all read and write operations are limited
    /// to the given `namespace` if `Some`.
    pub fn to_handle(&self) -> store::Handle {
        Self::new_handle_inner(&self.state, None)
    }

    /// As above, but supports a namespace to be set
    pub fn to_handle_namespaced(&self, namespace: Option<Namespace>) -> store::Handle {
        Self::new_handle_inner(&self.state, namespace)
    }

    fn new_handle_inner(state: &store::State, namespace: Option<Namespace>) -> store::Handle {
        store::Handle {
            state: match state {
                store::State::Loose {
                    path,
                    reflog_mode,
                    packed_buffer,
                } => store::handle::State::Loose {
                    store: {
                        let mut store = crate::file::Store::at(path, *reflog_mode);
                        store.namespace = namespace;
                        store
                    },
                    packed_buffer: packed_buffer.clone(),
                },
            },
        }
    }
}

///
pub mod find;

mod iter {
    // impl store::Handle {
    //     pub fn iter<'p, 's>(&'s self, packed: Option<&'p packed::Buffer>) -> std::io::Result<LooseThenPacked<'p, 's>> {
    // }
}
