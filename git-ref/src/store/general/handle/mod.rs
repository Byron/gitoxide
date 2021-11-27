use crate::{store, Namespace};
use git_features::threading::OwnShared;

#[derive(Clone)]
pub(crate) enum State {
    Loose {
        store: crate::file::Store,
        packed_buffer: OwnShared<store::packed::ModifiableBuffer>,
    },
}

impl crate::Store {
    /// Return a new handle which sees all references if `namespace` is `None` or all read and write operations are limited
    /// to the given `namespace` if `Some`.
    pub fn to_handle(&self, namespace: Option<Namespace>) -> store::Handle {
        match &self.state {
            store::State::Loose {
                path,
                reflog_mode,
                packed_buffer,
            } => store::Handle {
                state: store::handle::State::Loose {
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

mod iter {
    // impl store::Handle {
    //     pub fn iter<'p, 's>(&'s self, packed: Option<&'p packed::Buffer>) -> std::io::Result<LooseThenPacked<'p, 's>> {
    // }
}
