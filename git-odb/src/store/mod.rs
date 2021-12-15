use std::{cell::RefCell, sync::Arc};

pub use sink::{sink, Sink};

///
pub mod cache;
pub mod compound;
pub mod general;
pub mod linked;
pub mod loose;

///
pub mod sink;

/// A way to access objects along with pre-configured thread-local caches for packed base objects as well as objects themselves.
///
/// By default, no cache will be used.
pub struct Cache<S> {
    /// The inner provider of trait implementations we use in conjunction with our caches.
    pub inner: S,
    // TODO: have single-threaded code-paths also for pack-creation (entries from counts) so that we can use OwnShared here
    //       instead of Arc. However, it's probably not that important as these aren't called often.
    new_pack_cache: Option<Arc<cache::NewPackCacheFn>>,
    new_object_cache: Option<Arc<cache::NewObjectCacheFn>>,
    pack_cache: Option<RefCell<Box<cache::PackCache>>>,
    object_cache: Option<RefCell<Box<cache::ObjectCache>>>,
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
