//! This crate provides the [`Repository`] abstraction which serves as a hub into all the functionality of git.
//!
//! It's powerful and won't sacrifice performance while still increasing convenience compared to using the sub-crates
//! individually. Sometimes it may hide complexity under the assumption that the performance difference doesn't matter
//! for all but the fewest tools out there, which would be using the underlying crates directly or file an issue.
//!
//! # The prelude and extensions
//!
//! With `use git_repository::prelude::*` you should be ready to go as it pulls in various extension traits to make functionality
//! available on objects that may use it.
//!
//! The method signatures are still complex and may require various arguments for configuration and cache control.
//!
//! ## Easy-Mode
//!
//! Most extensions to existing objects provide an `obj_with_extension.easy(&repo).an_easier_version_of_a_method()` or `easy(&repo)`
//! method to hide all complex arguments and sacrifice some performance for a lot of convenience.
//!
//! When starting out, use `easy(…)` and migrate to the more detailed method signatures to squeeze out more performance.
//!
//! # Cargo-features
//!
//! ## One-stop-shop
//!
//! To make using  _sub-crates_ easier these are re-exported into the root of this crate.
//!
//! `git_repository::`
//! * [`hash`]
//! * [`url`]
//! * [`actor`]
//! * [`object`]
//!   * [`bstr`][object::bstr]
//! * [`odb`]
//!   * [`pack`][odb::pack]
//! * [`refs`]
//! * [`interrupt`]
//! * [`tempfile`]
//! * [`traverse`]
//! * [`diff`]
//! * [`Progress`]
//! * [`progress`]
//! * [`interrupt`]
//! * [`protocol`]
//!   * [`transport`][protocol::transport]
//!
#![deny(unsafe_code, rust_2018_idioms)]
#![allow(missing_docs, unused)]

use crate::hash::ObjectId;
use std::{cell::RefCell, path::PathBuf, rc::Rc, sync::Arc};

// Re-exports to make this a potential one-stop shop crate avoiding people from having to reference various crates themselves.
// This also means that their major version changes affect our major version, but that's alright as we directly expose their
// APIs/instances anyway.
pub use git_actor as actor;
#[cfg(feature = "git-diff")]
pub use git_diff as diff;
pub use git_features::{parallel, progress, progress::Progress};
pub use git_hash as hash;
pub use git_object as object;
pub use git_odb as odb;
#[cfg(feature = "git-protocol")]
pub use git_protocol as protocol;
pub use git_ref as refs;
pub use git_tempfile as tempfile;
#[cfg(feature = "git-traverse")]
pub use git_traverse as traverse;
#[cfg(feature = "git-url")]
pub use git_url as url;

pub mod interrupt;

#[cfg(feature = "git-traverse")]
pub mod ext;
pub mod prelude {
    pub use git_features::parallel::reduce::Finalize;
    pub use git_odb::{Find, FindExt, Write};

    #[cfg(all(feature = "git-traverse"))]
    pub use crate::ext::*;
    pub use crate::reference::ReferencesExt;
}

pub mod init;

pub mod path;
pub use path::Path;

pub mod repository;

pub struct Repository {
    pub refs: git_ref::file::Store,
    pub odb: git_odb::linked::Store,
    pub working_tree: Option<PathBuf>,
}

mod handles {
    use std::{cell::RefCell, rc::Rc, sync::Arc};

    use crate::{Cache, Repository};

    pub struct Shared {
        pub repo: Rc<Repository>,
        pub cache: RefCell<Cache>,
    }

    /// A handle is what threaded programs would use to have thread-local but otherwise shared versions the same `Repository`.
    ///
    /// Mutable data present in the `Handle` itself while keeping the parent `Repository` (which has its own cache) shared.
    /// Otherwise handles reflect the API of a `Repository`.
    pub struct SharedArc {
        pub repo: Arc<Repository>,
        pub cache: RefCell<Cache>,
    }

    impl Clone for Shared {
        fn clone(&self) -> Self {
            Shared {
                repo: Rc::clone(&self.repo),
                cache: RefCell::new(Default::default()),
            }
        }
    }

    impl Clone for SharedArc {
        fn clone(&self) -> Self {
            SharedArc {
                repo: Arc::clone(&self.repo),
                cache: RefCell::new(Default::default()),
            }
        }
    }

    impl From<Repository> for Shared {
        fn from(repo: Repository) -> Self {
            Shared {
                repo: Rc::new(repo),
                cache: RefCell::new(Default::default()),
            }
        }
    }

    impl From<Repository> for SharedArc {
        fn from(repo: Repository) -> Self {
            SharedArc {
                repo: Arc::new(repo),
                cache: RefCell::new(Default::default()),
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
}
pub use handles::{Shared, SharedArc};

#[derive(Default)]
pub struct Cache {
    packed_refs: Option<refs::packed::Buffer>,
    pub pack: odb::pack::cache::Never, // TODO: choose great alround cache
    pub buf: Vec<u8>,
}

mod traits {
    use std::cell::RefMut;

    use crate::{Cache, Repository, Shared, SharedArc};

    pub trait Access {
        fn repo(&self) -> &Repository;
        fn cache_mut(&self) -> RefMut<'_, Cache>;
    }

    impl Access for Shared {
        fn repo(&self) -> &Repository {
            self.repo.as_ref()
        }

        fn cache_mut(&self) -> RefMut<'_, Cache> {
            self.cache.borrow_mut()
        }
    }

    impl Access for SharedArc {
        fn repo(&self) -> &Repository {
            self.repo.as_ref()
        }

        fn cache_mut(&self) -> RefMut<'_, Cache> {
            self.cache.borrow_mut()
        }
    }
}
pub use traits::Access;

mod cache {
    use crate::{
        refs::{file, packed},
        Cache,
    };

    impl Cache {
        pub fn packed_refs(
            &mut self,
            file: &file::Store,
        ) -> Result<Option<&packed::Buffer>, packed::buffer::open::Error> {
            match self.packed_refs {
                Some(ref packed) => Ok(Some(packed)),
                None => {
                    self.packed_refs = file.packed()?;
                    Ok(self.packed_refs.as_ref())
                }
            }
        }
    }
}

pub struct Object<'r, A> {
    id: ObjectId,
    // data: odb::pack::data::Object<'a>,
    access: &'r A,
}

pub struct Reference<'r, A> {
    pub(crate) backing: reference::Backing,
    pub(crate) access: &'r A,
}

pub mod reference {
    use std::cell::RefCell;

    use git_hash::ObjectId;

    use crate::{refs, refs::mutable, Access, Object, Reference, Repository};

    pub(crate) enum Backing {
        OwnedPacked {
            /// The validated full name of the reference.
            name: mutable::FullName,
            /// The target object id of the reference, hex encoded.
            target: ObjectId,
            /// The fully peeled object id, hex encoded, that the ref is ultimately pointing to
            /// i.e. when all indirections are removed.
            object: Option<ObjectId>,
        },
        LooseFile(refs::file::loose::Reference),
    }

    pub mod peel_to_id_in_place {
        use quick_error::quick_error;

        use crate::refs;

        quick_error! {
            #[derive(Debug)]
            pub enum Error {
                LoosePeelToId(err: refs::file::loose::reference::peel::to_id::Error) {
                    display("Could not peel loose reference")
                    from()
                    source(err)
                }
                PackedRefsOpen(err: refs::packed::buffer::open::Error) {
                    display("The packed-refs file could not be opened")
                    from()
                    source(err)
                }
            }
        }
    }

    impl<'r, A> Reference<'r, A>
    where
        A: Access + Sized,
    {
        pub fn peel_to_id_in_place(&mut self) -> Result<Object<'r, A>, peel_to_id_in_place::Error> {
            let repo = self.access.repo();
            match &mut self.backing {
                Backing::LooseFile(r) => {
                    let oid = r.peel_to_id_in_place(
                        &repo.refs,
                        self.access.cache_mut().packed_refs(&repo.refs)?,
                        |oid, buf| {
                            repo.odb
                                .find(oid, buf, &mut self.access.cache_mut().pack)
                                .map(|po| po.map(|o| (o.kind, o.data)))
                        },
                    )?;
                    todo!("loose")
                }
                Backing::OwnedPacked { name, target, object } => {
                    todo!("packed")
                }
            }
        }
    }

    mod access {
        use std::{cell::RefCell, convert::TryInto};

        use crate::{
            hash::ObjectId,
            reference::Backing,
            refs,
            refs::{file::find::Error, PartialName},
            Access, Reference, Repository,
        };

        /// Obtain and alter references comfortably
        pub trait ReferencesExt: Access + Sized {
            fn find_reference<'a, Name, E>(
                &self,
                name: Name,
            ) -> Result<Option<Reference<'_, Self>>, crate::reference::find::Error>
            where
                Name: TryInto<PartialName<'a>, Error = E>,
                Error: From<E>,
            {
                match self
                    .repo()
                    .refs
                    .find(name, self.cache_mut().packed_refs(&self.repo().refs)?)
                {
                    Ok(r) => match r {
                        Some(r) => Ok(Some(Reference {
                            backing: match r {
                                refs::file::Reference::Packed(p) => Backing::OwnedPacked {
                                    name: p.name.into(),
                                    target: p.target(),
                                    object: p
                                        .object
                                        .map(|hex| ObjectId::from_hex(hex).expect("a hash kind we know")),
                                },
                                refs::file::Reference::Loose(l) => Backing::LooseFile(l),
                            },
                            access: self,
                        })),
                        None => Ok(None),
                    },
                    Err(err) => Err(err.into()),
                }
            }
        }

        impl<A> ReferencesExt for A where A: Access + Sized {}
    }
    use crate::odb::Find;
    pub use access::ReferencesExt;

    pub mod find {
        use quick_error::quick_error;

        use crate::refs;

        quick_error! {
            #[derive(Debug)]
            pub enum Error {
                Find(err: refs::file::find::Error) {
                    display("An error occurred when trying to find a reference")
                    from()
                    source(err)
                }
                PackedRefsOpen(err: refs::packed::buffer::open::Error) {
                    display("The packed-refs file could not be opened")
                    from()
                    source(err)
                }
            }
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Kind {
    Bare,
    WorkingTree,
}

impl Kind {
    pub fn is_bare(&self) -> bool {
        matches!(self, Kind::Bare)
    }
}

pub fn discover(directory: impl AsRef<std::path::Path>) -> Result<Repository, repository::discover::Error> {
    Repository::discover(directory)
}
