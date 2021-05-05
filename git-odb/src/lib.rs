#![deny(missing_docs, unsafe_code, rust_2018_idioms)]

//! Git stores all of its data as _Objects_, which are nothing more than data along with a hash over all data. Thus it's an
//! object store indexed by data with inherent deduplication: the same data will have the same hash, and thus occupy the same
//! space within the database.
//!
//! There are various flavours of object databases, all of which supporting iteration, reading and possibly writing.
//!
//! * [`loose::Db`]
//!   * A database storing one object per file, named by its hash, using zlib compression.
//!   * O(1) reads and writes, bound by IO operations per second
//! * [`pack::Bundle`]
//!   * A database storing multiple objects within an indexed pack file, reaching compression ratios of 60 to 1 or more.
//!   * Slow writes and fast reads
//! * [`compound::Db`]
//!   * A database using a [`loose::Db`] for writes and multiple [`pack::Bundle`]s for object reading. It can also refer to multiple
//!     additional [`compound::Db`] instances using git-alternates.
//!   * This is the database closely resembling the object database in a git repository, and probably what most people would want to use.
//! * [`linked::Db`]
//!   * A database containing various [`compound::Dbs`][compound::Db] as gathered from `alternates` files.

mod zlib;

pub mod alternate;
pub mod compound;
pub mod linked;
pub mod loose;
pub mod pack;

mod sink;
pub use sink::{sink, Sink};

pub(crate) mod hash;

pub mod data;

///
pub mod find;
pub use find::Find;
///
pub mod write;
pub use write::Write;
