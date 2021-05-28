//! This crate provides the [`Repository`] abstraction which serves as a hub into all the functionality of git.
//!
//! It's powerful and won't sacrifice performance while still increasing convenience compared to using the sub-crates
//! individually.
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
//! * `hash`
//! * `object`
//!   * `bstr`
//! * `odb`
//!   * `pack`
//! * `refs`
//! * `traverse`
//! * `diff`
//! * `Progress`
//!
#![forbid(unsafe_code)]
#![deny(rust_2018_idioms)]
#![allow(missing_docs)]

use std::path::PathBuf;

// Re-exports to make this a potential one-stop shop crate avoiding people from having to reference various crates themselves.
// This also means that their major version changes affect our major version, but that's alright as we directly expose their
// APIs/instances anyway.
#[cfg(feature = "one-stop-shop")]
pub use git_diff as diff;
#[cfg(feature = "one-stop-shop")]
pub use git_features::{interrupt, progress, progress::Progress};
#[cfg(feature = "one-stop-shop")]
pub use git_hash as hash;
#[cfg(feature = "one-stop-shop")]
pub use git_object as object;
pub use git_odb as odb;
pub use git_ref as refs;
#[cfg(feature = "one-stop-shop")]
pub use git_traverse as traverse;

#[cfg(feature = "one-stop-shop")]
pub mod ext;
pub mod prelude {
    #[cfg(feature = "one-stop-shop")]
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
