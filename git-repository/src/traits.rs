use std::cell::RefMut;

use crate::{Cache, Repository, Shared, SharedArc};

pub trait Access {
    fn repo(&self) -> &Repository;
    fn cache(&self) -> &Cache;
}

impl Access for Shared {
    fn repo(&self) -> &Repository {
        self.repo.as_ref()
    }

    fn cache(&self) -> &Cache {
        &self.cache
    }
}

impl Access for SharedArc {
    fn repo(&self) -> &Repository {
        self.repo.as_ref()
    }

    fn cache(&self) -> &Cache {
        &self.cache
    }
}
