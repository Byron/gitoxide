pub use sink::{sink, Sink};
use std::cell::RefCell;
use std::sync::Arc;

pub mod compound;
pub mod handle;
pub mod linked;
pub mod loose;

///
pub mod sink;

/// Note that this type is only `Send` if `git-features/parallel` is toggled on.
pub struct Handle<S> {
    store: S,
    // TODO: have single-threaded code-paths also for pack-creation (entries from counts) so that we can use OwnShared here
    // instead of Arc
    new_pack_cache: Option<Arc<handle::NewPackCacheFn>>,
    new_object_cache: Option<Arc<handle::NewObjectCacheFn>>,
    pack_cache: Option<RefCell<Box<handle::PackCache>>>,
    object_cache: Option<RefCell<Box<handle::ObjectCache>>>,
}
