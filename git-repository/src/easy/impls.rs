use std::{rc::Rc, sync::Arc};

use parking_lot::lock_api::{ArcRwLockReadGuard, ArcRwLockWriteGuard};

use crate::{easy, Easy, EasyArc, EasyArcExclusive, EasyShared, Repository};

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

/// # Which `Easy` is for me?
///
/// For one-off commands and single-threaded applications, use [`EasyShared`] or [`Easy`] as they don't allow mutable repository
/// access which won't be needed.
///
/// When multiple threads are involved, use [`EasyArc`] instead.
///
/// Finally, if there is the need for adapting to changed object packs on disk or working with namespaces, mutable `Repository` access
/// is needed, as provided by `EasyArcExclusive`. Currently mutable shared access is only available in thread-save versions, but that
/// shall be fixed in the future as Rust's support for GATs becomes stable.
// TODO: Update this once there is `EasyExclusive` as well.
impl Repository {
    /// Transform this instance into an [`EasyShared`] with borrowed `Repository`.
    ///
    /// Since a standard reference is used, the repository can never be mutated, which is required for the
    /// fewest operations.
    pub fn to_easy(&self) -> EasyShared<'_> {
        EasyShared {
            repo: self,
            state: Default::default(),
        }
    }
    /// Transform this instance into an [`Easy`], offering shared immutable access to the repository, for the current thread.
    pub fn into_easy(self) -> Easy {
        self.into()
    }

    /// Transform this instance into an [`EasyArc`], offering shared immutable access to the repository for use across threads.
    pub fn into_easy_arc(self) -> EasyArc {
        self.into()
    }

    /// Transform this instance into an [`EasyArcExclusive`], offering shared immutable access to the repository for use across threads.
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
    type RepoRef = Rc<Repository>; // TODO: this could be a reference with GATs
    type RepoRefMut = &'static mut Repository; // this is a lie

    fn repo(&self) -> Result<Self::RepoRef, easy::borrow::repo::Error> {
        Ok(self.repo.clone())
    }

    /// TODO: With GATs, this can return an actual RefMut<'a, _> so mutability is possible in single-threaded mode.
    fn repo_mut(&self) -> Result<Self::RepoRefMut, easy::borrow::repo::Error> {
        Err(easy::borrow::repo::Error)
    }

    fn state(&self) -> &easy::State {
        &self.state
    }
}

impl easy::Access for EasyArc {
    type RepoRef = Arc<Repository>;
    type RepoRefMut = &'static mut Repository; // this is a lie

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
