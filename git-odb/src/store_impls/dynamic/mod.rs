#![allow(missing_docs)]

use crate::Store;
use std::{cell::RefCell, ops::Deref};

/// This effectively acts like a handle but exists to be usable from the actual `crate::Handle` implementation which adds caches on top.
/// Each store is quickly cloned and contains thread-local state for shared packs.
pub struct Handle<S>
where
    S: Deref<Target = Store> + Clone,
{
    pub(crate) store: S,
    pub refresh_mode: crate::RefreshMode,

    pub(crate) token: Option<handle::Mode>,
    snapshot: RefCell<load_index::Snapshot>,
}

///
pub mod find;

pub mod iter;

///
pub mod write;

pub mod init;

pub(crate) mod types;
pub use types::Metrics;

pub(crate) mod handle;

pub mod load_index;

mod load_pack;

mod metrics;
