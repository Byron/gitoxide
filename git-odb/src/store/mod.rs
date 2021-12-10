use std::{cell::RefCell, sync::Arc};

pub use sink::{sink, Sink};

pub mod compound;
pub mod general;
///
pub mod handle;
pub mod linked;
pub mod loose;

///
pub mod sink;

/// A way to access objects along with pre-configured thread-local caches for packed base objects as well as objects themselves.
///
/// By default, no cache will be used.
pub struct Cache<S> {
    store: S,
    // TODO: have single-threaded code-paths also for pack-creation (entries from counts) so that we can use OwnShared here
    //       instead of Arc. However, it's probably not that important as these aren't called often.
    new_pack_cache: Option<Arc<handle::NewPackCacheFn>>,
    new_object_cache: Option<Arc<handle::NewObjectCacheFn>>,
    pack_cache: Option<RefCell<Box<handle::PackCache>>>,
    object_cache: Option<RefCell<Box<handle::ObjectCache>>>,
}
