use std::cell::RefMut;

use crate::{Cache, Easy, EasyArc, Repository};

pub trait Access {
    fn repo(&self) -> &Repository;
    fn cache(&self) -> &Cache;
}

impl Access for Easy {
    fn repo(&self) -> &Repository {
        self.repo.as_ref()
    }

    fn cache(&self) -> &Cache {
        &self.cache
    }
}

impl Access for EasyArc {
    fn repo(&self) -> &Repository {
        self.repo.as_ref()
    }

    fn cache(&self) -> &Cache {
        &self.cache
    }
}
