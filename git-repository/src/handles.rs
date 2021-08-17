use std::{cell::RefCell, rc::Rc, sync::Arc};

use crate::{Cache, Repository};

pub struct Shared {
    pub repo: Rc<Repository>,
    pub cache: Cache,
}

/// A handle is what threaded programs would use to have thread-local but otherwise shared versions the same `Repository`.
///
/// Mutable data present in the `Handle` itself while keeping the parent `Repository` (which has its own cache) shared.
/// Otherwise handles reflect the API of a `Repository`.
pub struct SharedArc {
    pub repo: Arc<Repository>,
    pub cache: Cache,
}

impl Clone for Shared {
    fn clone(&self) -> Self {
        Shared {
            repo: Rc::clone(&self.repo),
            cache: Default::default(),
        }
    }
}

impl Clone for SharedArc {
    fn clone(&self) -> Self {
        SharedArc {
            repo: Arc::clone(&self.repo),
            cache: Default::default(),
        }
    }
}

impl From<Repository> for Shared {
    fn from(repo: Repository) -> Self {
        Shared {
            repo: Rc::new(repo),
            cache: Default::default(),
        }
    }
}

impl From<Repository> for SharedArc {
    fn from(repo: Repository) -> Self {
        SharedArc {
            repo: Arc::new(repo),
            cache: Default::default(),
        }
    }
}

impl Repository {
    pub fn into_shared(self) -> Shared {
        self.into()
    }

    pub fn into_shared_arc(self) -> SharedArc {
        self.into()
    }
}
