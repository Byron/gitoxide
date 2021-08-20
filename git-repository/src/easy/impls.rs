use std::{rc::Rc, sync::Arc};

use parking_lot::lock_api::{ArcRwLockReadGuard, ArcRwLockWriteGuard};

use crate::{easy, Easy, EasyArc, EasyArcExclusive, EasyShared, Repository};

impl Clone for Easy {
    fn clone(&self) -> Self {
        Easy {
            repo: Rc::clone(&self.repo),
            state: Default::default(),
        }
    }
}

impl Clone for EasyArc {
    fn clone(&self) -> Self {
        EasyArc {
            repo: Arc::clone(&self.repo),
            state: Default::default(),
        }
    }
}

impl<'repo> Clone for EasyShared<'repo> {
    fn clone(&self) -> Self {
        EasyShared {
            repo: self.repo,
            state: Default::default(),
        }
    }
}

impl From<Repository> for Easy {
    fn from(repo: Repository) -> Self {
        Easy {
            repo: Rc::new(repo),
            state: Default::default(),
        }
    }
}

impl From<Repository> for EasyArc {
    fn from(repo: Repository) -> Self {
        EasyArc {
            repo: Arc::new(repo),
            state: Default::default(),
        }
    }
}

impl From<Repository> for EasyArcExclusive {
    fn from(repo: Repository) -> Self {
        EasyArcExclusive {
            repo: Arc::new(parking_lot::RwLock::new(repo)),
            state: Default::default(),
        }
    }
}

impl Repository {
    pub fn to_easy(&self) -> EasyShared<'_> {
        EasyShared {
            repo: self,
            state: Default::default(),
        }
    }
    pub fn into_easy(self) -> Easy {
        self.into()
    }

    pub fn into_easy_arc(self) -> EasyArc {
        self.into()
    }

    pub fn into_easy_arc_exclusive(self) -> EasyArcExclusive {
        self.into()
    }
}

impl<'repo> easy::Access for EasyShared<'repo> {
    type RepoRef = &'repo Repository;
    type RepoRefMut = &'repo mut Repository;

    fn repo(&self) -> Result<Self::RepoRef, easy::borrow::repo::Error> {
        Ok(self.repo)
    }

    fn repo_mut(&self) -> Result<Self::RepoRefMut, easy::borrow::repo::Error> {
        Err(easy::borrow::repo::Error)
    }

    fn state(&self) -> &easy::State {
        &self.state
    }
}

impl easy::Access for Easy {
    type RepoRef = Rc<Repository>;
    type RepoRefMut = ArcRwLockWriteGuard<parking_lot::RawRwLock, Repository>; // this is a lie

    fn repo(&self) -> Result<Self::RepoRef, easy::borrow::repo::Error> {
        Ok(self.repo.clone())
    }

    fn repo_mut(&self) -> Result<Self::RepoRefMut, easy::borrow::repo::Error> {
        Err(easy::borrow::repo::Error)
    }

    fn state(&self) -> &easy::State {
        &self.state
    }
}

impl easy::Access for EasyArc {
    type RepoRef = Arc<Repository>;
    type RepoRefMut = ArcRwLockWriteGuard<parking_lot::RawRwLock, Repository>; // this is a lie

    fn repo(&self) -> Result<Self::RepoRef, easy::borrow::repo::Error> {
        Ok(self.repo.clone())
    }
    fn repo_mut(&self) -> Result<Self::RepoRefMut, easy::borrow::repo::Error> {
        Err(easy::borrow::repo::Error)
    }
    fn state(&self) -> &easy::State {
        &self.state
    }
}

impl easy::Access for EasyArcExclusive {
    type RepoRef = ArcRwLockReadGuard<parking_lot::RawRwLock, Repository>;
    type RepoRefMut = ArcRwLockWriteGuard<parking_lot::RawRwLock, Repository>;

    fn repo(&self) -> Result<Self::RepoRef, easy::borrow::repo::Error> {
        Ok(self.repo.read_arc())
    }
    fn repo_mut(&self) -> Result<Self::RepoRefMut, easy::borrow::repo::Error> {
        Ok(self.repo.write_arc())
    }
    fn state(&self) -> &easy::State {
        &self.state
    }
}
