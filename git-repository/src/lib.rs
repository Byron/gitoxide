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
//! Most extensions to existing objects provide an `obj_with_extension.attach(&repo).an_easier_version_of_a_method()` or `repo.to_easy()`
//! method to hide all complex arguments and trade a little control for a lot of convenience.
//!
//! When starting out, use [`easy::Handle`] and migrate to the more detailed method signatures to
//! squeeze out the last inkling of performance if it really does make a difference.
//!
//! ## Object-Access Performance
//!
//! Accessing objects quickly is the bread-and-butter of working with git, right after accessing references. Hence it's vital
//! to understand which cache levels exist and how to leverage them.
//!
//! When accessing an object, the first cache that's queried is a  memory-capped LRU object cache, mapping their id to data and kind.
//! On miss, the object is looked up and if ia pack is hit, there is a small fixed-size cache for delta-base objects.
//!
//! In scenarios where the same objects are accessed multiple times, an object cache can be useful and is to be configured specifically
//! using the [`object_cache_size(…)`][easy::Handle::object_cache_size()] method.
//!
//! Use the `cache-efficiency-debug` cargo feature to learn how efficient the cache actually is - it's easy to end up with lowered
//! performance if the cache is not hit in 50% of the time.
//!
//! Environment variables can also be used for configuration if the application is calling
//! [`apply_environment()`][easy::Handle::apply_environment()] on their `Easy*` accordingly.
//!
//! ### Shortcomings & Limitations
//!
//! - Only a single `easy::Object` or derivatives can be held in memory at a time, _per `Easy*`_.
//! - Changes made to the configuration, packs, and alternates aren't picked up automatically, but the current object store
//!   needs a manual refresh.
//!
//! ### Design Sketch
//!
//! Goal is to make the lower-level plumbing available without having to deal with any caches or buffers, and avoid any allocation
//! beyond sizing the buffer to fit the biggest object seen so far.
//!
//! * no implicit object lookups, thus `Oid` needs to get an `Object` first to start out with data via `object()`
//! * Objects with `Ref` suffix can only exist one at a time unless they are transformed into an owned version of it OR
//!   multiple `Easy` handles are present, each providing another 'slot' for an object as long as its retrieved through
//!   the respective `Easy` object.
//! * `ObjectRef` blocks the current buffer, hence many of its operations that use the buffer are consuming
//! * All methods that access a any field from `Easy`'s mutable `State` are fallible, and return `easy::Result<_>` at least, to avoid
//!   panics if the field can't be referenced due to borrow rules of `RefCell`.
//! * Anything attached to `Access` can be detached to lift the object limit or make them `Send`-able. They can be `attached` to another
//!   `Access` if needed.
//! * `git-repository` functions related to `Access` extensions will always return attached versions of return values, like `Oid` instead
//!   of `git_hash::ObjectId`, `ObjectRef` instead of `git_odb::data::Object`, or `Reference` instead of `git_ref::Reference`.
//! * Obtaining mutable is currently a weak spot as these only work with Arc<RwLock> right now and can't work with `Rc<RefCell>` due
//!   to missing GATs, presumably. All `Easy*!Exclusive` types are unable to provide a mutable reference to the underlying repository.
//!   However, other ways to adjust the `Repository` of long-running applications are possible. For instance, there could be a flag that
//!   indicates a new `Repository` should be created (for instance, after it was changed) which causes the next server connection to
//!   create a new one. This instance is the one to use when spawning new `EasyArc` instances.
//! * `Platform` types are used to hold mutable or shared versions of required state for use in dependent objects they create, like iterators.
//!   These come with the benefit of allowing for nicely readable call chains. Sometimes these are called `Platform` for a lack of a more specific
//!   term, some are called more specifically like `Ancestors`.
//!
//! ### Terminology
//!
//! #### WorkingTree and WorkTree
//!
//! When reading the documentation of the canonical git-worktree program one gets the impression work tree and working tree are used
//! interchangeably. We use the term _work tree_ only and try to do so consistently as its shorter and assumed to be the same.
//!
//! # Cargo-features
//!
//! ## With the optional "unstable" cargo feature
//!
//! To make using  _sub-crates_ easier these are re-exported into the root of this crate. Note that these may change their major version
//! even if this crate doesn't, hence breaking downstream.
//!
//! `git_repository::`
//! * [`hash`]
//! * [`url`]
//! * [`actor`]
//! * [`bstr`][bstr]
//! * [`index`]
//! * [`objs`]
//! * [`odb`]
//!   * [`pack`][odb::pack]
//! * [`refs`]
//! * [`interrupt`]
//! * [`tempfile`]
//! * [`lock`]
//! * [`traverse`]
//! * [`diff`]
//! * [`parallel`]
//! * [`Progress`]
//! * [`progress`]
//! * [`interrupt`]
//! * [`protocol`]
//!   * [`transport`][protocol::transport]
//!     * [`packetline`][protocol::transport::packetline]
//!
//! ## Feature Flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![deny(missing_docs, unsafe_code, rust_2018_idioms)]

use std::path::PathBuf;

// Re-exports to make this a potential one-stop shop crate avoiding people from having to reference various crates themselves.
// This also means that their major version changes affect our major version, but that's alright as we directly expose their
// APIs/instances anyway.
pub use git_actor as actor;
#[cfg(all(feature = "unstable", feature = "git-diff"))]
pub use git_diff as diff;
#[cfg(feature = "unstable")]
pub use git_features::{parallel, progress, progress::Progress, threading};
pub use git_hash as hash;
#[doc(inline)]
pub use git_hash::{oid, ObjectId};
#[cfg(all(feature = "unstable", feature = "git-index"))]
pub use git_index as index;
pub use git_lock as lock;
pub use git_object as objs;
pub use git_object::bstr;
#[cfg(feature = "unstable")]
pub use git_odb as odb;
#[cfg(all(feature = "unstable", feature = "git-protocol"))]
pub use git_protocol as protocol;
pub use git_ref as refs;
#[cfg(feature = "unstable")]
pub use git_tempfile as tempfile;
#[cfg(feature = "unstable")]
pub use git_traverse as traverse;
#[cfg(all(feature = "unstable", feature = "git-url"))]
pub use git_url as url;
#[doc(inline)]
#[cfg(all(feature = "unstable", feature = "git-url"))]
pub use git_url::Url;

pub mod interrupt;

mod ext;
///
pub mod prelude {
    pub use git_features::parallel::reduce::Finalize;
    pub use git_odb::{Find, FindExt, Write};

    pub use crate::ext::*;
}

///
pub mod path;

///
pub mod sync;
pub use sync::{discover, init, open};

/// The standard type for a store to handle git references.
pub type RefStore = git_ref::file::Store;
/// A handle for finding objects in an object database, abstracting away caches for thread-local use.
pub type OdbHandle = git_odb::Handle;

/// A repository path which either points to a work tree or the `.git` repository itself.
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Path {
    /// The currently checked out or nascent work tree of a git repository.
    WorkTree(PathBuf),
    /// The git repository itself
    Repository(PathBuf),
}

///
pub mod easy;

///
pub mod commit;
///
pub mod reference;

/// The kind of `Repository`
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Kind {
    /// A bare repository does not have a work tree, that is files on disk beyond the `git` repository itself.
    Bare,
    /// A `git` repository along with a checked out files in a work tree.
    WorkTree,
}

impl Kind {
    /// Returns true if this is a bare repository, one without a work tree.
    pub fn is_bare(&self) -> bool {
        matches!(self, Kind::Bare)
    }
}

/// See [Repository::discover()].
pub fn discover(directory: impl AsRef<std::path::Path>) -> Result<easy::Repository, sync::discover::Error> {
    sync::Handle::discover(directory).map(Into::into)
}

/// See [Repository::init()].
pub fn init(directory: impl AsRef<std::path::Path>) -> Result<easy::Repository, sync::init::Error> {
    sync::Handle::init(directory, Kind::WorkTree).map(Into::into)
}

/// See [Repository::init()].
pub fn init_bare(directory: impl AsRef<std::path::Path>) -> Result<easy::Repository, sync::init::Error> {
    sync::Handle::init(directory, Kind::Bare).map(Into::into)
}

/// See [Repository::open()].
pub fn open(directory: impl Into<std::path::PathBuf>) -> Result<easy::Repository, sync::open::Error> {
    sync::Handle::open(directory).map(Into::into)
}
