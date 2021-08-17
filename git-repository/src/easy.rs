use std::{cell::RefCell, rc::Rc, sync::Arc};

use crate::{Cache, Repository};

pub struct Easy {
    pub repo: Rc<Repository>,
    pub cache: Cache,
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
