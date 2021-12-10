#![allow(missing_docs, unused, dead_code)]

use std::ops::Deref;

/// This effectively acts like a handle but exists to be usable from the actual `crate::Handle` implementation which adds caches on top.
/// Each store is quickly cloned and contains thread-local state for shared packs.
#[derive(Clone)]
pub struct Store<S>
// where
//     S: Deref<Target = State> + Clone,
{
    state: S,
}

mod state {
    use std::path::PathBuf;
    use std::sync::atomic::AtomicUsize;

    pub struct State {
        /// The source directory from which all content is loaded, and the central write lock for use when a directory refresh is needed.
        path: parking_lot::Mutex<PathBuf>,

        /// The amount of handles that would prevent us from unloading packs or indices
        pub(crate) num_handles_stable: AtomicUsize,
        /// The amount of handles that don't affect our ability to compact our internal data structures or unload packs or indices.
        pub(crate) num_handles_unstable: AtomicUsize,
    }

    impl State {
        pub fn new(objects_dir: PathBuf) -> Self {
            State {
                path: parking_lot::Mutex::new(objects_dir),
                num_handles_stable: Default::default(),
                num_handles_unstable: Default::default(),
            }
        }
    }
}
pub use state::State;

mod store {
    use super::Store;
    use crate::store::general::State;
    use git_features::threading::OwnShared;
    use std::ops::Deref;
    use std::path::PathBuf;

    pub fn at(objects_dir: impl Into<PathBuf>) -> std::io::Result<Store<OwnShared<State>>> {
        let object_dir = objects_dir.into();
        if !object_dir.is_dir() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other, // TODO: use NotADirectory when stabilized
                format!("'{}' wasn't a directory", object_dir.display()),
            ));
        }
        Ok(Store {
            state: OwnShared::new(State::new(object_dir)),
        })
    }
}
