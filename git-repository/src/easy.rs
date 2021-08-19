use std::cell::RefCell;

use crate::{odb, refs, Repository};

type PackCache = odb::pack::cache::Never; // TODO: choose great all-round cache

#[derive(Default)]
pub struct State {
    packed_refs: RefCell<Option<refs::packed::Buffer>>,
    pub(crate) pack: RefCell<PackCache>,
    pub(crate) buf: RefCell<Vec<u8>>,
}

pub trait Access {
    fn repo(&self) -> &Repository;
    fn state(&self) -> &State;
}

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
        pub(crate) fn try_borrow_mut_pack(&self) -> Result<RefMut<'_, PackCache>, borrow::Error> {
            self.pack.try_borrow_mut().map_err(Into::into)
        }
    }
}

mod impls {
    use std::{rc::Rc, sync::Arc};

    use crate::{easy, Easy, EasyArc, Repository};

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

    impl easy::Access for Easy {
        fn repo(&self) -> &Repository {
            self.repo.as_ref()
        }

        fn state(&self) -> &easy::State {
            &self.cache
        }
    }

    impl easy::Access for EasyArc {
        fn repo(&self) -> &Repository {
            self.repo.as_ref()
        }

        fn state(&self) -> &easy::State {
            &self.cache
        }
    }
}
