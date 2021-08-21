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
//! * no implicit object lookups, thus `Oid` needs to get an `Object` first to start out with data via `object()`
//! * Objects with `Ref` suffix can only exist one at a time unless they are transformed into an owned version of it OR
//!   multiple `Easy` handles are present, each providing another 'slot' for an object as long as its retrieved through
//!   the respective `Easy` object.
//! * `ObjectRef` blocks the current buffer, hence many of its operations that use the buffer are consuming
//! * All methods that access a any field from `Easy`'s mutable `State` are fallible, and return `easy::Result<_>` at least, to avoid
//!   panics if the field can't be referenced due to borrow rules of `RefCell`.
//! * Anything attached to `Access` can be detached to lift the object limit or make them `Send`-able. They can be `attached` to another
//!   `Access` if needed.
//! * git-repository functions related to `Access` extensions will always return attached versions of return values, like `Oid` instead
//!   of `ObjectId`, `ObjectRef` instead of `git_odb::data::Object`, or `Reference` instead of `git_ref::file::Reference`.
//! * Obtaining mutable is currently a weak spot as these only work with Arc<RwLock> right now and can't work with `Rc<RefCell>` due
//!   to missing GATs, presumably. All `Easy*!Exclusive` types are unable to provide a mutable reference to the underlying repository.
//!   However, other ways to adjust the `Repository` of long-running applications are possible. For instance, there could be a flag that
//!   indicates a new `Repository` should be created (for instance, after it was changed) which causes the next server connection to
//!   create a new one. This instance is the one to use when spawning new `EasyArc` instances.
//!
//! #### Limitations
//!
//! * types containing `&impl Access` can't access extension traits directly but have to use a workaround. This is due to the way
//!   extension traits can't apply internally if if it is implemented, but must be part of the external interface. This is only
//!   relevant for code within `git-repository`
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
//! * [`lock`]
//! * [`traverse`]
//! * [`diff`]
//! * [`Progress`]
//! * [`progress`]
//! * [`interrupt`]
//! * [`protocol`]
//!   * [`transport`][protocol::transport]
//!
#![deny(missing_docs, unsafe_code, rust_2018_idioms)]

use std::{path::PathBuf, rc::Rc, sync::Arc};

// Re-exports to make this a potential one-stop shop crate avoiding people from having to reference various crates themselves.
// This also means that their major version changes affect our major version, but that's alright as we directly expose their
// APIs/instances anyway.
pub use git_actor as actor;
#[cfg(feature = "git-diff")]
pub use git_diff as diff;
pub use git_features::{parallel, progress, progress::Progress};
pub use git_hash as hash;
pub use git_lock as lock;
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
pub use path::Path;

pub mod interrupt;

mod ext;
///
pub mod prelude {
    pub use git_features::parallel::reduce::Finalize;
    pub use git_odb::{Find, FindExt, Write};

    pub use crate::{easy::ext::*, ext::*};
}

///
pub mod init;

///
pub mod path;
///
pub mod repository;

/// A instance with access to everything a git repository entails, best imagined as container for _most_ for system resources required
/// to interact with a `git` repository which are loaded in once the instance is created.
///
/// These resources are meant to be shareable across threads and used by most using an `Easy*` type has a handle carrying additional
/// in-memory data to accelerate data access or hold volatile data. Depending on context, `EasyShared` gets the fastest read-only
/// access to the repository, whereas `Easy` has to go through an `Rc` and `EasyArcExclusive` through an `Arc<RwLock>`.
///
/// Namely, this is an object database, a reference database to point to objects.
pub struct Repository {
    /// A store for references to point at objects
    pub refs: git_ref::file::Store,
    /// A store for objects that contain data
    pub odb: git_odb::linked::Store,
    /// TODO: git-config should be here - it's read a lot but not written much in must applications, so shouldn't be in `State`.
    ///       Probably it's best reload it on signal (in servers) or refresh it when it's known to have been changed similar to how
    ///       packs are refreshed.
    /// The path to the worktree at which to find checked out files
    pub work_tree: Option<PathBuf>,
}

/// A handle to a `Repository` for use when the repository needs to be shared, providing state for one `ObjectRef` at a time, , created with [`Repository::into_easy()`].
///
/// For use in one-off single-threaded commands that don't have to deal with the changes they potentially incur.
/// TODO: There should be an `EasyExclusive` using `Rc<RefCell<…>>` but that needs GATs.
#[derive(Clone)]
pub struct Easy {
    /// The repository
    pub repo: Rc<Repository>,
    /// The state with interior mutability
    pub state: easy::State,
}

/// A handle to a repository for use when the repository needs to be shared using an actual reference, providing state for one `ObjectRef` at a time, created with [`Repository::to_easy()`]
///
/// For use in one-off commands that don't have to deal with the changes they potentially incur.
#[derive(Clone)]
pub struct EasyShared<'a> {
    /// The repository
    pub repo: &'a Repository,
    /// The state with interior mutability
    pub state: easy::State,
}

/// A handle to a `Repository` for sharing across threads, with each thread having one or more caches,
/// created with [`Repository::into_easy_arc()`]
///
/// For use in one-off commands in threaded applications that don't have to deal with the changes they potentially incur.
#[derive(Clone)]
pub struct EasyArc {
    /// The repository
    pub repo: Arc<Repository>,
    /// The state with interior mutability
    pub state: easy::State,
}

/// A handle to a optionally mutable `Repository` for use in long-running applications that eventually need to update the `Repository`
/// to adapt to changes they triggered or that were caused by other processes.
///
/// Using it incurs costs as each `Repository` access has to go through an indirection and involve an _eventually fair_ `RwLock`.
/// However, it's vital to precisely updating the `Repository` instance as opposed to creating a new one while serving other requests
/// on an old instance, which potentially duplicates the resource costs.
#[derive(Clone)]
pub struct EasyArcExclusive {
    /// The repository
    pub repo: Arc<parking_lot::RwLock<Repository>>,
    /// The state with interior mutability
    pub state: easy::State,
}

pub mod easy;

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
pub fn discover(directory: impl AsRef<std::path::Path>) -> Result<Repository, repository::discover::Error> {
    Repository::discover(directory)
}
