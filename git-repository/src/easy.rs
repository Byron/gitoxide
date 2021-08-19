use std::{rc::Rc, sync::Arc};

use crate::{odb, refs, Repository};
use std::cell::RefCell;

pub struct Easy {
    pub repo: Rc<Repository>,
    pub cache: Cache,
}

#[derive(Default)]
pub struct Cache {
    pub(crate) packed_refs: RefCell<Option<refs::packed::Buffer>>,
    pub(crate) pack: RefCell<odb::pack::cache::Never>, // TODO: choose great all-round cache
    pub(crate) buf: RefCell<Vec<u8>>,
}

mod cache {
    use std::ops::DerefMut;

    use crate::{
        refs::{file, packed},
        Cache,
    };

    impl Cache {
        // TODO: this method should be on the Store itself, as one day there will be reftable support which lacks packed-refs
        pub(crate) fn assure_packed_refs_present(&self, file: &file::Store) -> Result<(), packed::buffer::open::Error> {
            if self.packed_refs.borrow().is_none() {
                *self.packed_refs.borrow_mut().deref_mut() = file.packed()?;
            }
            Ok(())
        }
    }
}

/// A handle is what threaded programs would use to have thread-local but otherwise shared versions the same `Repository`.
///
/// Mutable data present in the `Handle` itself while keeping the parent `Repository` (which has its own cache) shared.
/// Otherwise handles reflect the API of a `Repository`.
pub struct EasyArc {
    pub repo: Arc<Repository>,
    pub cache: Cache,
}

impl Clone for Easy {
    fn clone(&self) -> Self {
        Easy {
            repo: Rc::clone(&self.repo),
            cache: Default::default(),
        }
    }
}

impl Clone for EasyArc {
    fn clone(&self) -> Self {
        EasyArc {
            repo: Arc::clone(&self.repo),
            cache: Default::default(),
        }
    }
}

impl From<Repository> for Easy {
    fn from(repo: Repository) -> Self {
        Easy {
            repo: Rc::new(repo),
            cache: Default::default(),
        }
    }
}

impl From<Repository> for EasyArc {
    fn from(repo: Repository) -> Self {
        EasyArc {
            repo: Arc::new(repo),
            cache: Default::default(),
        }
    }
}

impl Repository {
    pub fn into_easy(self) -> Easy {
        self.into()
    }

    pub fn into_easy_arc(self) -> EasyArc {
        self.into()
    }
}
