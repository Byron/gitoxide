#![deny(missing_docs, unsafe_code, rust_2018_idioms)]

//! Git stores all of its data as _Objects_, which ata along with a hash over all data. Thus it's an
//! object store indexed by data with inherent deduplication: the same data will have the same hash, and thus occupy the same
//! space within the store.
//!
//! There are various flavours of object stores, all of which supporting iteration, reading and possibly writing.
//!
//! * [`loose::Store`]
//!   * A database storing one object per file, named by its hash, using zlib compression.
//!   * O(1) reads and writes, bound by IO operations per second
//! * [`compound::Store`]
//!   * A database using a [`loose::Store`] for writes and multiple [`pack::Bundle`]s for object reading. It can also refer to multiple
//!     additional [`compound::Store`] instances using git-alternates.
//!   * This is the database closely resembling the object database in a git repository, and probably what most people would want to use.
//! * [`linked::Store`]
//!   * A database containing various [`compound::Stores`][compound::Store] as gathered from `alternates` files.
use git_features::threading::OwnShared;
pub use git_pack as pack;
use std::path::PathBuf;

mod store;
pub use store::{compound, general, handle, linked, loose, sink, Cache, Sink};

pub mod alternate;

/// A thread-local handle to access any object.
pub type Handle = Cache<general::Handle<OwnShared<general::Store>>>;

/// Create a new cached odb handle.
pub fn at(objects_dir: impl Into<PathBuf>) -> std::io::Result<Handle> {
    let handle = OwnShared::new(general::Store::at(objects_dir)?).to_handle();
    Ok(Cache::from(handle))
}

///
pub mod find;
mod traits;
pub use traits::{Find, FindExt, Write};
