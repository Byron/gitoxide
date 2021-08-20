use std::{rc::Rc, sync::Arc};

use parking_lot::lock_api::{ArcRwLockReadGuard, ArcRwLockWriteGuard};

use crate::{easy, Easy, EasyArc, EasyArcExclusive, EasyShared, Repository};
use std::cell::{Ref, RefMut};

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

impl<'repo> easy::AccessGAT for EasyShared<'repo> {
    type RepoRef<'a> = &'a Repository;
    type RepoRefMut<'a> = &'a mut Repository;

    fn repo(&self) -> Result<Self::RepoRef<'_>, easy::borrow::repo::Error> {
        Ok(self.repo)
    }

    fn repo_mut<'a>(&'a self) -> Result<Self::RepoRefMut<'repo>, easy::borrow::repo::Error> {
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

/// A handle to a `Repository` for use when the repository needs to be shared, providing state for one `ObjectRef` at a time, , created with [`Repository::into_easy()`].
///
/// For use in one-off commands that don't have to deal with the changes they potentially incur.
pub struct EasyExclusive {
    /// The repository
    pub repo: Rc<std::cell::RefCell<Repository>>,
    /// The state with interior mutability
    pub state: easy::State,
}

impl easy::AccessGAT for EasyExclusive {
    type RepoRef<'a> = Ref<'a, Repository>; // this will be Ref<'a, Repository>
    type RepoRefMut<'a> = RefMut<'a, Repository>; // this is a lie

    fn repo(&self) -> Result<Self::RepoRef<'_>, easy::borrow::repo::Error> {
        Ok(self.repo.borrow())
    }

    fn repo_mut(&self) -> Result<Self::RepoRefMut<'_>, easy::borrow::repo::Error> {
        self.repo.try_borrow_mut().map_err(Into::into)
    }

    fn state(&self) -> &easy::State {
        &self.state
    }
}

impl easy::AccessGAT for EasyArcExclusive {
    type RepoRef<'a> = ArcRwLockReadGuard<parking_lot::RawRwLock, Repository>;
    type RepoRefMut<'a> = ArcRwLockWriteGuard<parking_lot::RawRwLock, Repository>;

    fn repo(&self) -> Result<Self::RepoRef<'_>, easy::borrow::repo::Error> {
        Ok(self.repo.read_arc())
    }

    fn repo_mut(&self) -> Result<Self::RepoRefMut<'_>, easy::borrow::repo::Error> {
        Ok(self.repo.write_arc())
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
