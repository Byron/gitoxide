//! This module is a bit 'misplaced' if spelled out like '`gix_pack::cache::object::`*' but is best placed here for code re-use and
//! general usefulness.
use crate::cache;

#[cfg(feature = "object-cache-dynamic")]
mod memory {
    use std::num::NonZeroUsize;

    use clru::WeightScale;

    use crate::cache;

    struct Entry {
        data: Vec<u8>,
        kind: gix_object::Kind,
    }

    type Key = gix_hash::ObjectId;

    struct CustomScale;

    impl WeightScale<Key, Entry> for CustomScale {
        fn weight(&self, key: &Key, value: &Entry) -> usize {
            value.data.len() + std::mem::size_of::<Entry>() + key.as_bytes().len()
        }
    }

    /// An LRU cache with hash map backing and an eviction rule based on the memory usage for object data in bytes.
    pub struct MemoryCappedHashmap {
        inner: clru::CLruCache<Key, Entry, gix_hashtable::hash::Builder, CustomScale>,
        free_list: Vec<Vec<u8>>,
        debug: gix_features::cache::Debug,
    }

    impl MemoryCappedHashmap {
        /// The amount of bytes we can hold in total, or the value we saw in `new(…)`.
        pub fn capacity(&self) -> usize {
            self.inner.capacity()
        }
        /// Return a new instance which evicts least recently used items if it uses more than `memory_cap_in_bytes`
        /// object data.
        pub fn new(memory_cap_in_bytes: usize) -> MemoryCappedHashmap {
            MemoryCappedHashmap {
                inner: clru::CLruCache::with_config(
                    clru::CLruCacheConfig::new(NonZeroUsize::new(memory_cap_in_bytes).expect("non zero"))
                        .with_hasher(gix_hashtable::hash::Builder)
                        .with_scale(CustomScale),
                ),
                free_list: Vec::new(),
                debug: gix_features::cache::Debug::new(format!("MemoryCappedObjectHashmap({memory_cap_in_bytes}B)")),
            }
        }
    }

    impl cache::Object for MemoryCappedHashmap {
        /// Put the object going by `id` of `kind` with `data` into the cache.
        fn put(&mut self, id: gix_hash::ObjectId, kind: gix_object::Kind, data: &[u8]) {
            self.debug.put();
            let res = self.inner.put_with_weight(
                id,
                Entry {
                    data: self.free_list.pop().map_or_else(
                        || Vec::from(data),
                        |mut v| {
                            v.clear();
                            v.resize(data.len(), 0);
                            v.copy_from_slice(data);
                            v
                        },
                    ),
                    kind,
                },
            );
            match res {
                Ok(Some(previous_entry)) => self.free_list.push(previous_entry.data),
                Ok(None) => {}
                Err((_key, value)) => self.free_list.push(value.data),
            }
        }

        /// Try to retrieve the object named `id` and place its data into `out` if available and return `Some(kind)` if found.
        fn get(&mut self, id: &gix_hash::ObjectId, out: &mut Vec<u8>) -> Option<gix_object::Kind> {
            let res = self.inner.get(id).map(|e| {
                out.resize(e.data.len(), 0);
                out.copy_from_slice(&e.data);
                e.kind
            });
            if res.is_some() {
                self.debug.hit()
            } else {
                self.debug.miss()
            }
            res
        }
    }
}
#[cfg(feature = "object-cache-dynamic")]
pub use memory::MemoryCappedHashmap;

/// A cache implementation that doesn't do any caching.
pub struct Never;

impl cache::Object for Never {
    /// Noop
    fn put(&mut self, _id: gix_hash::ObjectId, _kind: gix_object::Kind, _data: &[u8]) {}

    /// Noop
    fn get(&mut self, _id: &gix_hash::ObjectId, _out: &mut Vec<u8>) -> Option<gix_object::Kind> {
        None
    }
}

impl<T: cache::Object + ?Sized> cache::Object for Box<T> {
    fn put(&mut self, id: gix_hash::ObjectId, kind: gix_object::Kind, data: &[u8]) {
        use std::ops::DerefMut;
        self.deref_mut().put(id, kind, data)
    }

    fn get(&mut self, id: &gix_hash::ObjectId, out: &mut Vec<u8>) -> Option<gix_object::Kind> {
        use std::ops::DerefMut;
        self.deref_mut().get(id, out)
    }
}
