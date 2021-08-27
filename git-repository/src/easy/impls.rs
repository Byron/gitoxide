#![allow(missing_docs)]
use std::{rc::Rc, sync::Arc};

// #[cfg(feature = "parking_lot_future")]
// use parking_lot_future::lock_api::{ArcRwLockReadGuard, ArcRwLockWriteGuard};

use crate::{easy, Easy, EasyArc, EasyShared, Repository};

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

// #[cfg(feature = "parking_lot_future")]
// impl From<Repository> for crate::EasyArcExclusive {
//     fn from(repo: Repository) -> Self {
//         crate::EasyArcExclusive {
//             repo: Arc::new(parking_lot_future::RwLock::new(repo)),
//             state: Default::default(),
//         }
//     }
// }

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

    // #[cfg(feature = "parking_lot_future")]
    // pub fn into_easy_arc_exclusive(self) -> crate::EasyArcExclusive {
    //     self.into()
    // }
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

// #[cfg(feature = "parking_lot_future")]
// impl easy::Access for crate::EasyArcExclusive {
//     type RepoRef = ArcRwLockReadGuard<parking_lot_future::RawRwLock, Repository>;
//     type RepoRefMut = ArcRwLockWriteGuard<parking_lot_future::RawRwLock, Repository>;
//
//     fn repo(&self) -> Result<Self::RepoRef, easy::borrow::repo::Error> {
//         Ok(self.repo.read_arc())
//     }
//     fn repo_mut(&self) -> Result<Self::RepoRefMut, easy::borrow::repo::Error> {
//         Ok(self.repo.write_arc())
//     }
//     fn state(&self) -> &easy::State {
//         &self.state
//     }
// }
