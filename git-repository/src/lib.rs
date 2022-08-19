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
//! Most extensions to existing objects provide an `obj_with_extension.attach(&repo).an_easier_version_of_a_method()` for simpler
//! call signatures.
//!
//! ## ThreadSafe Mode
//!
//! By default, the [`Repository`] isn't `Sync` and thus can't be used in certain contexts which require the `Sync` trait.
//!
//! To help with this, convert it with `.to_sync()` into a [`ThreadSafeRepository`].
//!
//! ## Object-Access Performance
//!
//! Accessing objects quickly is the bread-and-butter of working with git, right after accessing references. Hence it's vital
//! to understand which cache levels exist and how to leverage them.
//!
//! When accessing an object, the first cache that's queried is a  memory-capped LRU object cache, mapping their id to data and kind.
//! On miss, the object is looked up and if a pack is hit, there is a small fixed-size cache for delta-base objects.
//!
//! In scenarios where the same objects are accessed multiple times, an object cache can be useful and is to be configured specifically
//! using the [`object_cache_size(…)`][crate::Repository::object_cache_size()] method.
//!
//! Use the `cache-efficiency-debug` cargo feature to learn how efficient the cache actually is - it's easy to end up with lowered
//! performance if the cache is not hit in 50% of the time.
//!
//! Environment variables can also be used for configuration if the application is calling
//! [`apply_environment()`][crate::Repository::apply_environment()].
//!
//! ### Shortcomings & Limitations
//!
//! - Only a single `crate::object` or derivatives can be held in memory at a time, _per `Easy*`_.
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
//! * [`attrs`]
//! * [`hash`]
//! * [`url`]
//! * [`actor`]
//! * [`bstr`][bstr]
//! * [`date`]
//! * [`mod@discover`]
//! * [`index`]
//! * [`glob`]
//! * [`path`]
//! * [`credentials`]
//! * [`sec`]
//! * [`worktree`]
//! * [`mailmap`]
//! * [`objs`]
//! * [`odb`]
//!   * [`pack`][odb::pack]
//! * [`refs`]
//! * [`revision`]
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

// Re-exports to make this a potential one-stop shop crate avoiding people from having to reference various crates themselves.
// This also means that their major version changes affect our major version, but that's alright as we directly expose their
// APIs/instances anyway.
pub use git_actor as actor;
#[cfg(all(feature = "unstable"))]
pub use git_attributes as attrs;
#[cfg(all(feature = "unstable", feature = "git-credentials"))]
pub use git_credentials as credentials;
#[cfg(feature = "unstable")]
pub use git_date as date;
#[cfg(all(feature = "unstable", feature = "git-diff"))]
pub use git_diff as diff;
use git_features::threading::OwnShared;
#[cfg(feature = "unstable")]
pub use git_features::{parallel, progress, progress::Progress, threading};
#[cfg(feature = "unstable")]
pub use git_glob as glob;
pub use git_hash as hash;
#[doc(inline)]
#[cfg(feature = "unstable")]
pub use git_index as index;
pub use git_lock as lock;
pub use git_object as objs;
pub use git_object::bstr;
#[cfg(feature = "unstable")]
pub use git_odb as odb;
#[cfg(all(feature = "unstable", feature = "git-protocol"))]
pub use git_protocol as protocol;
pub use git_ref as refs;
pub use git_sec as sec;
#[cfg(feature = "unstable")]
pub use git_tempfile as tempfile;
#[cfg(feature = "unstable")]
pub use git_traverse as traverse;
#[cfg(all(feature = "unstable", feature = "git-url"))]
pub use git_url as url;
#[doc(inline)]
#[cfg(all(feature = "unstable", feature = "git-url"))]
pub use git_url::Url;
pub use hash::{oid, ObjectId};

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

/// The standard type for a store to handle git references.
pub type RefStore = git_ref::file::Store;
/// A handle for finding objects in an object database, abstracting away caches for thread-local use.
pub type OdbHandle = git_odb::Handle;
/// A way to access git configuration
pub(crate) type Config = OwnShared<git_config::File<'static>>;

///
mod types;
pub use types::{
    Commit, Head, Id, Kind, Object, ObjectDetached, Reference, Repository, Tag, ThreadSafeRepository, Tree, Worktree,
};

pub mod commit;
pub mod head;
pub mod id;
pub mod object;
pub mod reference;
mod repository;
pub mod tag;

/// See [ThreadSafeRepository::discover()], but returns a [`Repository`] instead.
pub fn discover(directory: impl AsRef<std::path::Path>) -> Result<Repository, discover::Error> {
    ThreadSafeRepository::discover(directory).map(Into::into)
}

/// See [ThreadSafeRepository::init()], but returns a [`Repository`] instead.
pub fn init(directory: impl AsRef<std::path::Path>) -> Result<Repository, init::Error> {
    ThreadSafeRepository::init(
        directory,
        create::Options {
            bare: false,
            fs_capabilities: None,
        },
    )
    .map(Into::into)
}

/// See [ThreadSafeRepository::init()], but returns a [`Repository`] instead.
pub fn init_bare(directory: impl AsRef<std::path::Path>) -> Result<Repository, init::Error> {
    ThreadSafeRepository::init(
        directory,
        create::Options {
            bare: true,
            fs_capabilities: None,
        },
    )
    .map(Into::into)
}

/// See [ThreadSafeRepository::open()], but returns a [`Repository`] instead.
pub fn open(directory: impl Into<std::path::PathBuf>) -> Result<Repository, open::Error> {
    ThreadSafeRepository::open(directory).map(Into::into)
}

/// See [ThreadSafeRepository::open_opts()], but returns a [`Repository`] instead.
pub fn open_opts(directory: impl Into<std::path::PathBuf>, options: open::Options) -> Result<Repository, open::Error> {
    ThreadSafeRepository::open_opts(directory, options).map(Into::into)
}

///
pub mod permission {
    ///
    pub mod env_var {
        use git_sec::{permission, Access};

        /// A permission to control access to the resource pointed to an environment variable.
        pub type Resource = Access<permission::Resource, git_sec::Permission>;
        ///
        pub mod resource {
            ///
            pub type Error = git_sec::permission::Error<std::path::PathBuf, git_sec::Permission>;
        }
    }
}
///
pub mod permissions {
    pub use crate::repository::permissions::{Config, Environment};
}
pub use repository::permissions::Permissions;

///
pub mod create;

///
pub mod open;

///
pub mod config;

///
pub mod mailmap;

///
pub mod worktree;

pub mod revision;

///
pub mod init {
    use std::path::Path;

    use crate::ThreadSafeRepository;

    /// The error returned by [`crate::init()`].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error(transparent)]
        Init(#[from] crate::create::Error),
        #[error(transparent)]
        Open(#[from] crate::open::Error),
    }

    impl ThreadSafeRepository {
        /// Create a repository with work-tree within `directory`, creating intermediate directories as needed.
        ///
        /// Fails without action if there is already a `.git` repository inside of `directory`, but
        /// won't mind if the `directory` otherwise is non-empty.
        pub fn init(directory: impl AsRef<Path>, options: crate::create::Options) -> Result<Self, Error> {
            use git_sec::trust::DefaultForLevel;
            let path = crate::create::into(directory.as_ref(), options)?;
            let (git_dir, worktree_dir) = path.into_repository_and_work_tree_directories();
            let options = crate::open::Options::default_for_level(git_sec::Trust::Full);
            ThreadSafeRepository::open_from_paths(git_dir, worktree_dir, options).map_err(Into::into)
        }
    }
}

/// Not to be confused with 'status'.
pub mod state {
    /// Tell what operation is currently in progress.
    #[derive(Debug, PartialEq, Eq)]
    pub enum InProgress {
        /// A mailbox is being applied.
        ApplyMailbox,
        /// A rebase is happening while a mailbox is being applied.
        // TODO: test
        ApplyMailboxRebase,
        /// A git bisect operation has not yet been concluded.
        Bisect,
        /// A cherry pick operation.
        CherryPick,
        /// A cherry pick with multiple commits pending.
        CherryPickSequence,
        /// A merge operation.
        Merge,
        /// A rebase operation.
        Rebase,
        /// An interactive rebase operation.
        RebaseInteractive,
        /// A revert operation.
        Revert,
        /// A revert operation with multiple commits pending.
        RevertSequence,
    }
}

///
pub mod discover;

///
pub mod env {
    use std::ffi::OsString;

    /// Equivalent to `std::env::args_os()`, but with precomposed unicode on MacOS and other apple platforms.
    #[cfg(not(target_vendor = "apple"))]
    pub fn args_os() -> impl Iterator<Item = OsString> {
        std::env::args_os()
    }

    /// Equivalent to `std::env::args_os()`, but with precomposed unicode on MacOS and other apple platforms.
    ///
    /// Note that this ignores `core.precomposeUnicode` as git-config isn't available yet. It's default enabled in modern git though.
    #[cfg(target_vendor = "apple")]
    pub fn args_os() -> impl Iterator<Item = OsString> {
        use unicode_normalization::UnicodeNormalization;
        std::env::args_os().map(|arg| match arg.to_str() {
            Some(arg) => arg.nfc().collect::<String>().into(),
            None => arg,
        })
    }
}

mod kind;
