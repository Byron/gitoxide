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

use std::path::PathBuf;

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

pub struct Easy {
    pub repo: RefCell<Repository>,
    pub cache: Cache,
}

mod easy {
    use crate::{Easy, Repository};
    use std::cell::RefCell;

    impl Into<Easy> for Repository {
        fn into(self) -> Easy {
            Easy {
                repo: RefCell::new(self),
                cache: Default::default(),
            }
        }
    }

    mod references {
        use crate::{
            reference::Backing,
            refs::{file::find::Error, PartialName},
            Easy, Reference, Repository,
        };
        use std::cell::RefCell;
        use std::convert::TryInto;

        /// Obtain and alter references comfortably
        impl Easy {
            pub fn find_reference<'a, Name, E>(
                &mut self,
                name: Name,
            ) -> Result<Option<Reference<'_>>, crate::reference::find::Error>
            where
                Name: TryInto<PartialName<'a>, Error = E>,
                Error: From<E>,
            {
                let repo = self.repo.borrow();
                match repo.refs.find(name, self.cache.packed_refs(&repo.refs)?) {
                    Ok(r) => match r {
                        Some(r) => Ok(Some(Reference {
                            backing: Backing::File(r),
                            repo: &self.repo,
                        })),
                        None => Ok(None),
                    },
                    Err(err) => Err(err.into()),
                }
            }
        }
    }
}

#[derive(Default)]
pub struct Cache {
    packed_refs: Option<refs::packed::Buffer>,
    pack: odb::pack::cache::lru::StaticLinkedList<64>,
    buf: Vec<u8>,
}

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
                Some(ref packed) => return Ok(Some(packed)),
                None => {
                    self.packed_refs = file.packed()?;
                    Ok(self.packed_refs.as_ref())
                }
            }
        }
    }
}

mod handle {
    use crate::{Cache, Repository};
    /// A handle is what threaded programs would use to have thread-local but otherwise shared versions the same `Repository`.
    ///
    /// Mutable data present in the `Handle` itself while keeping the parent `Repository` (which has its own cache) shared.
    /// Otherwise handles reflect the API of a `Repository`.
    pub struct Handle<'a> {
        parent: &'a Repository,
        cache: Cache,
    }
}

mod object_impl {
    use crate::{odb::pack, Repository};
    use git_hash::ObjectId;

    pub struct Object<'a> {
        id: ObjectId,
        data: pack::data::Object<'a>,
        access: &'a mut Repository,
    }
}
pub use object_impl::Object;

mod reference {
    use crate::{refs, Object, Repository};
    use std::cell::RefCell;

    pub(crate) enum Backing<'p> {
        File(refs::file::Reference<'p>),
    }

    pub struct Reference<'p> {
        pub(crate) backing: Backing<'p>,
        pub(crate) repo: &'p RefCell<Repository>,
    }

    impl<'p> Reference<'p> {
        pub fn peel_to_end(&mut self) -> Result<Object<'p>, ()> {
            todo!("peel and get lazy object")
        }
    }

    pub mod find {
        mod error {
            use crate::refs;
            use quick_error::quick_error;

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
        pub use error::Error;
    }
}
pub use reference::Reference;
use std::cell::RefCell;

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
