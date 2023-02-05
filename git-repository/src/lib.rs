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
//! To help with this, convert it with [`.into_sync()`][Repository::into_sync()] into a [`ThreadSafeRepository`].
//!
//! ## Object-Access Performance
//!
//! Accessing objects quickly is the bread-and-butter of working with git, right after accessing references. Hence it's vital
//! to understand which cache levels exist and how to leverage them.
//!
//! When accessing an object, the first cache that's queried is a  memory-capped LRU object cache, mapping their id to data and kind.
//! It has to be specifically enabled a [`Repository`].
//! On miss, the object is looked up and if a pack is hit, there is a small fixed-size cache for delta-base objects.
//!
//! In scenarios where the same objects are accessed multiple times, the object cache can be useful and is to be configured specifically
//! using the [`object_cache_size(…)`][crate::Repository::object_cache_size()] method.
//!
//! Use the `cache-efficiency-debug` cargo feature to learn how efficient the cache actually is - it's easy to end up with lowered
//! performance if the cache is not hit in 50% of the time.
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
//! To make using  _sub-crates_ easier these are re-exported into the root of this crate. Here we list how to access nested plumbing
//! crates which are otherwise harder to discover:
//!
//! **`git_repository::`**
//! * [`odb`]
//!   * [`pack`][odb::pack]
//! * [`protocol`]
//!   * [`transport`][protocol::transport]
//!     * [`packetline`][protocol::transport::packetline]
//!
//!
//! ## Feature Flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![deny(missing_docs, rust_2018_idioms, unsafe_code)]

// Re-exports to make this a potential one-stop shop crate avoiding people from having to reference various crates themselves.
// This also means that their major version changes affect our major version, but that's alright as we directly expose their
// APIs/instances anyway.
pub use git_actor as actor;
pub use git_attributes as attrs;
pub use git_credentials as credentials;
pub use git_date as date;
pub use git_diff as diff;
pub use git_features as features;
use git_features::threading::OwnShared;
pub use git_features::{parallel, progress::Progress, threading};
pub use git_glob as glob;
pub use git_hash as hash;
#[doc(inline)]
pub use git_index as index;
pub use git_lock as lock;
pub use git_object as objs;
pub use git_object::bstr;
pub use git_odb as odb;
pub use git_prompt as prompt;
#[cfg(all(feature = "git-protocol"))]
pub use git_protocol as protocol;
pub use git_ref as refs;
pub use git_refspec as refspec;
pub use git_sec as sec;
pub use git_tempfile as tempfile;
pub use git_traverse as traverse;
pub use git_url as url;
#[doc(inline)]
pub use git_url::Url;
pub use hash::{oid, ObjectId};

pub mod interrupt;

mod ext;
///
pub mod prelude {
    pub use git_features::parallel::reduce::Finalize;
    pub use git_odb::{Find, FindExt, Header, HeaderExt, Write};

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
    Commit, Head, Id, Kind, Object, ObjectDetached, Reference, Remote, Repository, Tag, ThreadSafeRepository, Tree,
    Worktree,
};

///
pub mod clone;
pub mod commit;
pub mod head;
pub mod id;
pub mod object;
pub mod reference;
mod repository;
pub mod tag;

///
pub mod progress {
    pub use git_features::progress::*;
    pub use prodash::tree;
}

/// See [ThreadSafeRepository::discover()], but returns a [`Repository`] instead.
pub fn discover(directory: impl AsRef<std::path::Path>) -> Result<Repository, discover::Error> {
    ThreadSafeRepository::discover(directory).map(Into::into)
}

/// See [ThreadSafeRepository::init()], but returns a [`Repository`] instead.
pub fn init(directory: impl AsRef<std::path::Path>) -> Result<Repository, init::Error> {
    ThreadSafeRepository::init(directory, create::Kind::WithWorktree, create::Options::default()).map(Into::into)
}

/// See [ThreadSafeRepository::init()], but returns a [`Repository`] instead.
pub fn init_bare(directory: impl AsRef<std::path::Path>) -> Result<Repository, init::Error> {
    ThreadSafeRepository::init(directory, create::Kind::Bare, create::Options::default()).map(Into::into)
}

/// Create a platform for configuring a bare clone from `url` to the local `path`, using default options for opening it (but
/// amended with using configuration from the git installation to ensure all authentication options are honored).
///
/// See [`clone::PrepareFetch::new()] for a function to take full control over all options.
#[allow(clippy::result_large_err)]
pub fn prepare_clone_bare<Url, E>(
    url: Url,
    path: impl AsRef<std::path::Path>,
) -> Result<clone::PrepareFetch, clone::Error>
where
    Url: std::convert::TryInto<git_url::Url, Error = E>,
    git_url::parse::Error: From<E>,
{
    clone::PrepareFetch::new(
        url,
        path,
        create::Kind::Bare,
        create::Options::default(),
        open_opts_with_git_binary_config(),
    )
}

/// Create a platform for configuring a clone with main working tree from `url` to the local `path`, using default options for opening it
/// (but amended with using configuration from the git installation to ensure all authentication options are honored).
///
/// See [`clone::PrepareFetch::new()] for a function to take full control over all options.
#[allow(clippy::result_large_err)]
pub fn prepare_clone<Url, E>(url: Url, path: impl AsRef<std::path::Path>) -> Result<clone::PrepareFetch, clone::Error>
where
    Url: std::convert::TryInto<git_url::Url, Error = E>,
    git_url::parse::Error: From<E>,
{
    clone::PrepareFetch::new(
        url,
        path,
        create::Kind::WithWorktree,
        create::Options::default(),
        open_opts_with_git_binary_config(),
    )
}

fn open_opts_with_git_binary_config() -> open::Options {
    use git_sec::trust::DefaultForLevel;
    let mut opts = open::Options::default_for_level(git_sec::Trust::Full);
    opts.permissions.config.git_binary = true;
    opts
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
        ///
        pub mod resource {
            ///
            pub type Error = git_sec::permission::Error<std::path::PathBuf>;
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
pub mod remote;

///
pub mod init;

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

pub mod env;

mod kind;
