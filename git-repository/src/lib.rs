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
//! ### Design Sketch
//!
//! Goal is to make the lower-level plumbing available without having to deal with any caches or buffers, and avoid any allocation
//! beyond sizing the buffer to fit the biggest object seen so far.
//!
//! * no implicit object lookups, thus `Oid` needs to get an `Object` first to start out with data
//! * Objects with `Ref` suffix can only exist one at a time unless they are transformed into an owned version of it OR
//!   multiple `Easy` handles are present, each providing another 'slot' for an object as long as its retrieved through
//!   the respective `Easy` object.
//! * `ObjectRef` blocks the current buffer, hence many operations that use the buffer are consuming
//! * There can only be one `Object` at a time, but as many `Oids` as you want.
//! * Anything attached to `Access` can be detached to lift the object limit or make them `Send` able. They can be `attached` to another
//!   `Access` if needed.
//! * git-repository functions return `Oid` for oids if they originate in something having or being `Access`
//!
//! #### Limitations
//!
//! * types containing `&impl Access` can't access extension traits directly but have to use a workaround. This is due to the way
//!   extension traits can't apply internally if if it is implemented, but must be part of the external interface
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
//! * [`objs`]
//!   * [`bstr`][objs::bstr]
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

use std::{cell::RefCell, path::PathBuf, rc::Rc, sync::Arc};

// Re-exports to make this a potential one-stop shop crate avoiding people from having to reference various crates themselves.
// This also means that their major version changes affect our major version, but that's alright as we directly expose their
// APIs/instances anyway.
pub use git_actor as actor;
#[cfg(feature = "git-diff")]
pub use git_diff as diff;
pub use git_features::{parallel, progress, progress::Progress};
pub use git_hash as hash;
pub use git_object as objs;
pub use git_odb as odb;
#[cfg(feature = "git-protocol")]
pub use git_protocol as protocol;
pub use git_ref as refs;
pub use git_tempfile as tempfile;
#[cfg(feature = "git-traverse")]
pub use git_traverse as traverse;
#[cfg(feature = "git-url")]
pub use git_url as url;

use crate::hash::ObjectId;

pub mod interrupt;

mod ext;
pub mod prelude {
    pub use git_features::parallel::reduce::Finalize;
    pub use git_odb::{Find, FindExt, Write};

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

mod easy;
pub use easy::{Easy, EasyArc};

#[derive(Default)]
pub struct Cache {
    packed_refs: RefCell<Option<refs::packed::Buffer>>,
    pub(crate) pack: RefCell<odb::pack::cache::Never>, // TODO: choose great all-round cache
    pub(crate) buf: RefCell<Vec<u8>>,
}

mod cache {
    use std::{cell::Ref, ops::DerefMut};

    use crate::{
        refs::{file, packed},
        Cache,
    };

    impl Cache {
        // TODO: this method should be on the Store itself, as one day there will be reftable support which lacks packed-refs
        pub(crate) fn assure_packed_refs_present(&self, file: &file::Store) -> Result<(), packed::buffer::open::Error> {
            use std::ops::Deref;
            if self.packed_refs.borrow().is_none() {
                *self.packed_refs.borrow_mut().deref_mut() = file.packed()?;
            }
            Ok(())
        }
    }
}

mod traits;
pub(crate) use traits::Access;

// TODO: really would ObjectId, but it's different to show it's attached - maybe this is the type used most of the time here?
pub struct Oid<'r, A> {
    id: ObjectId,
    access: &'r A,
}

pub struct ObjectRef<'repo, A> {
    pub id: ObjectId,
    pub kind: objs::Kind,
    pub data: std::cell::Ref<'repo, [u8]>,
    access: &'repo A,
}

pub struct TreeRef<'repo, A> {
    pub id: ObjectId,
    pub data: std::cell::Ref<'repo, [u8]>,
    access: &'repo A,
}

#[derive(Clone)]
pub struct Object {
    pub id: ObjectId,
    pub kind: objs::Kind,
    pub data: Vec<u8>,
}

pub mod object;

pub struct Reference<'r, A> {
    pub(crate) backing: Option<reference::Backing>,
    pub(crate) access: &'r A,
}

pub mod reference;

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
