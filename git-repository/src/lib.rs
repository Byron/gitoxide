//! # One-stop-shop
//!
//! `git_repository::`
//! * `hash`
//! * `object`
//!   * `bstr`
//! * `odb`
//!   * `pack`
//! * `refs`
//! * `prelude::*` -- all extension traits.
#![forbid(unsafe_code)]
#![deny(rust_2018_idioms)]
#![allow(missing_docs)]

use std::path::PathBuf;

// Re-exports to make this a potential one-stop shop crate avoiding people from having to reference various crates themselves.
// This also means that their major version changes affect our major version, but that's alright as we directly expose their
// APIs/instances anyway.
#[cfg(feature = "one-stop-shop")]
pub use git_hash as hash;
#[cfg(feature = "one-stop-shop")]
pub use git_object as object;
pub use git_odb as odb;
pub use git_ref as refs;
#[cfg(feature = "one-stop-shop")]
pub use git_traverse as traverse;

pub mod ext;
pub mod prelude {
    pub use crate::ext::*;
    pub use git_odb::{Find, FindExt};
}

pub mod init;

pub mod path;
pub use path::Path;

pub mod repository;

pub struct Repository {
    pub refs: git_ref::file::Store,
    pub working_tree: Option<PathBuf>,
    pub odb: git_odb::linked::Store,
}

impl Repository {
    pub fn kind(&self) -> Kind {
        match self.working_tree {
            Some(_) => Kind::WorkingTree,
            None => Kind::Bare,
        }
    }

    pub fn git_dir(&self) -> &std::path::Path {
        &self.refs.base
    }
    pub fn objects_dir(&self) -> &std::path::Path {
        &self.odb.dbs[0].loose.path
    }
}

#[cfg(feature = "one-stop-shop")]
mod extensions {
    use crate::Repository;
    use git_hash::{oid, ObjectId};
    use git_object::immutable;
    use git_traverse::commit::ancestors::{Ancestors, State};

    impl Repository {
        pub fn ancestors_iter<Find>(commit: impl Into<ObjectId>, find: Find) -> Ancestors<Find, fn(&oid) -> bool, State>
        where
            Find: for<'a> FnMut(&oid, &'a mut Vec<u8>) -> Option<immutable::CommitIter<'a>>,
        {
            Ancestors::new(Some(commit), State::default(), find)
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Kind {
    Bare,
    WorkingTree,
}

pub fn discover(directory: impl AsRef<std::path::Path>) -> Result<Repository, repository::discover::Error> {
    Repository::discover(directory)
}
