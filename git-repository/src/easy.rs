//! ### Which `Easy*` is for me?
//!
//! * Use `Easy*Exclusive` when the underlying `Repository` eventually needs mutation, for instance to update data structures
//!    - This is useful for long-running applications that eventually need to adapt to changes in the repository and pick up
//!      new packs after a GC operation or a received pack.
//! * Use the non-exclusive variants if the `Repository` doesn't ever have to change, for example as in one-off commands.
//!
//! ### Implementation Notes
//!
//! - Why no `Easy*` with simply an owned `Repository`, instead `Rc<Repository>` is enforced
//!    - When this is desired, rather use `EasyShared` and drop the `EasyShared` once mutable access to the `Repository` is needed.
//!      `Access` is not usable for functions that require official `&mut` mutability, it's made for interior mutability to support
//!       trees of objects.
use std::cell::RefCell;

use crate::{odb, refs, Repository};
use std::ops::Deref;

type PackCache = odb::pack::cache::Never; // TODO: choose great all-round cache

#[derive(Default)]
pub struct State {
    packed_refs: RefCell<Option<refs::packed::Buffer>>,
    pack_cache: RefCell<PackCache>,
    buf: RefCell<Vec<u8>>,
}

pub trait Access {
    fn repo(&self) -> &Repository;
    fn state(&self) -> &State;
}

pub trait Access2 {
    type RepoRef: Deref<Target = Repository>;

    fn repo(&self) -> Self::RepoRef;
    fn state(&self) -> &State;
}

pub type Result<T> = std::result::Result<T, state::borrow::Error>;

pub mod state {
    use std::ops::DerefMut;

    use crate::easy::PackCache;
    use crate::{
        easy, refs,
        refs::{file, packed},
    };
    use std::cell::{Ref, RefMut};

    pub mod borrow {
        use quick_error::quick_error;
        quick_error! {
            #[derive(Debug)]
            pub enum Error {
                Borrow(err: std::cell::BorrowError) {
                    display("A state member could not be borrowed")
                    from()
                }
                BorrowMut(err: std::cell::BorrowMutError) {
                    display("A state member could not be mutably borrowed")
                    from()
                }
            }
        }
    }

    impl easy::State {
        // TODO: this method should be on the Store itself, as one day there will be reftable support which lacks packed-refs
        pub(crate) fn assure_packed_refs_present(&self, file: &file::Store) -> Result<(), packed::buffer::open::Error> {
            if self.packed_refs.borrow().is_none() {
                *self.packed_refs.borrow_mut().deref_mut() = file.packed()?;
            }
            Ok(())
        }

        #[inline]
        pub(crate) fn try_borrow_packed_refs(&self) -> Result<Ref<'_, Option<refs::packed::Buffer>>, borrow::Error> {
            self.packed_refs.try_borrow().map_err(Into::into)
        }

        #[inline]
        pub(crate) fn try_borrow_mut_pack_cache(&self) -> Result<RefMut<'_, PackCache>, borrow::Error> {
            self.pack_cache.try_borrow_mut().map_err(Into::into)
        }

        #[inline]
        pub(crate) fn try_borrow_mut_buf(&self) -> Result<RefMut<'_, Vec<u8>>, borrow::Error> {
            self.buf.try_borrow_mut().map_err(Into::into)
        }

        #[inline]
        pub(crate) fn try_borrow_buf(&self) -> Result<Ref<'_, Vec<u8>>, borrow::Error> {
            self.buf.try_borrow().map_err(Into::into)
        }
    }
}

mod impls {
    use std::{rc::Rc, sync::Arc};

    use crate::{easy, Easy, EasyArc, EasyArcExclusive, EasyShared, Repository};
    use parking_lot::lock_api::ArcRwLockReadGuard;

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
    }

    impl easy::Access for Easy {
        fn repo(&self) -> &Repository {
            self.repo.as_ref()
        }

        fn state(&self) -> &easy::State {
            &self.state
        }
    }

    impl easy::Access for EasyArc {
        fn repo(&self) -> &Repository {
            self.repo.as_ref()
        }

        fn state(&self) -> &easy::State {
            &self.state
        }
    }

    impl<'repo> easy::Access for EasyShared<'repo> {
        fn repo(&self) -> &Repository {
            self.repo
        }

        fn state(&self) -> &easy::State {
            &self.state
        }
    }

    impl<'repo> easy::Access2 for EasyShared<'repo> {
        type RepoRef = &'repo Repository;

        fn repo(&self) -> Self::RepoRef {
            self.repo
        }
        fn state(&self) -> &easy::State {
            &self.state
        }
    }

    impl easy::Access2 for Easy {
        type RepoRef = Rc<Repository>;

        fn repo(&self) -> Self::RepoRef {
            self.repo.clone()
        }
        fn state(&self) -> &easy::State {
            &self.state
        }
    }

    impl easy::Access2 for EasyArc {
        type RepoRef = Arc<Repository>;

        fn repo(&self) -> Self::RepoRef {
            self.repo.clone()
        }
        fn state(&self) -> &easy::State {
            &self.state
        }
    }

    impl easy::Access2 for EasyArcExclusive {
        type RepoRef = ArcRwLockReadGuard<parking_lot::RawRwLock, Repository>;

        fn repo(&self) -> Self::RepoRef {
            self.repo.read_arc()
        }

        fn state(&self) -> &easy::State {
            &self.state
        }
    }
}
