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
    pub refresh_mode: RefreshMode,

    pub(crate) token: Option<handle::Mode>,
    snapshot: RefCell<load_index::Snapshot>,
}

/// Define how packs will be refreshed when all indices are loaded, which is useful if a lot of objects are missing.
#[derive(Clone, Copy)]
pub enum RefreshMode {
    /// Check for new or changed pack indices (and pack data files) when the last known index is loaded.
    /// During runtime we will keep pack indices stable by never reusing them, however, there is the option for
    /// clearing internal caches which is likely to change pack ids and it will trigger unloading of packs as they are missing on disk.
    AfterAllIndicesLoaded,
    /// Use this if you expect a lot of missing objects that shouldn't trigger refreshes even after all packs are loaded.
    /// This comes at the risk of not learning that the packs have changed in the mean time.
    Never,
}

impl Default for RefreshMode {
    fn default() -> Self {
        RefreshMode::AfterAllIndicesLoaded
    }
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
