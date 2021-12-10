#![allow(missing_docs, unused, dead_code)]

use std::ops::Deref;
use std::path::PathBuf;
use std::sync::atomic::AtomicUsize;

/// This effectively acts like a handle but exists to be usable from the actual `crate::Handle` implementation which adds caches on top.
/// Each store is quickly cloned and contains thread-local state for shared packs.
#[derive(Clone)]
pub struct Handle<S>
where
    S: Deref<Target = Store> + Clone,
{
    state: S,
}

pub struct Store {
    /// The source directory from which all content is loaded, and the central write lock for use when a directory refresh is needed.
    path: parking_lot::Mutex<PathBuf>,

    /// The amount of handles that would prevent us from unloading packs or indices
    pub(crate) num_handles_stable: AtomicUsize,
    /// The amount of handles that don't affect our ability to compact our internal data structures or unload packs or indices.
    pub(crate) num_handles_unstable: AtomicUsize,
}

mod find {
    use git_hash::oid;
    use git_object::Data;
    use git_pack::cache::DecodeEntry;
    use git_pack::data::entry::Location;
    use git_pack::index::Entry;
    use std::ops::Deref;

    impl<S> crate::pack::Find for super::Handle<S>
    where
        S: Deref<Target = super::Store> + Clone,
    {
        type Error = crate::compound::find::Error;

        fn contains(&self, id: impl AsRef<oid>) -> bool {
            todo!()
        }

        fn try_find_cached<'a>(
            &self,
            id: impl AsRef<oid>,
            buffer: &'a mut Vec<u8>,
            pack_cache: &mut impl DecodeEntry,
        ) -> Result<Option<(Data<'a>, Option<Location>)>, Self::Error> {
            todo!()
        }

        fn location_by_oid(&self, id: impl AsRef<oid>, buf: &mut Vec<u8>) -> Option<Location> {
            todo!()
        }

        fn index_iter_by_pack_id(&self, pack_id: u32) -> Option<Box<dyn Iterator<Item = Entry> + '_>> {
            todo!()
        }

        fn entry_by_location(&self, location: &Location) -> Option<git_pack::find::Entry<'_>> {
            todo!()
        }
    }
}

mod store {
    use git_features::threading::OwnShared;
    use std::path::PathBuf;
    use std::sync::atomic::AtomicUsize;
    use std::sync::Arc;

    impl super::Store {
        pub fn at(objects_dir: impl Into<PathBuf>) -> std::io::Result<Self> {
            let objects_dir = objects_dir.into();
            if !objects_dir.is_dir() {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other, // TODO: use NotADirectory when stabilized
                    format!("'{}' wasn't a directory", objects_dir.display()),
                ));
            }
            Ok(super::Store {
                path: parking_lot::Mutex::new(objects_dir),
                num_handles_stable: Default::default(),
                num_handles_unstable: Default::default(),
            })
        }

        pub fn to_handle(self: &OwnShared<Self>) -> super::Handle<OwnShared<super::Store>> {
            super::Handle { state: self.clone() }
        }
    }
}
