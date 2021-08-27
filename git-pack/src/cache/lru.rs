use super::DecodeEntry;

#[cfg(feature = "pack-cache-lru-dynamic")]
mod memory {
    use std::num::NonZeroUsize;

    use clru::WeightScale;

    use super::DecodeEntry;

    struct Entry {
        data: Vec<u8>,
        kind: git_object::Kind,
        compressed_size: usize,
    }

    type Key = (u32, u64);
    struct CustomScale;

    impl WeightScale<Key, Entry> for CustomScale {
        fn weight(&self, _key: &Key, value: &Entry) -> usize {
            value.data.len()
        }
    }

    /// An LRU cache with hash map backing and an eviction rule based on the memory usage for object data in bytes.
    pub struct MemoryCappedHashmap {
        inner: clru::CLruCache<Key, Entry, std::collections::hash_map::RandomState, CustomScale>,
        free_list: Vec<Vec<u8>>,
        debug: git_features::cache::Debug,
    }

    impl MemoryCappedHashmap {
        /// Return a new instance which evicts least recently used items if it uses more than `memory_cap_in_bytes`
        /// object data.
        pub fn new(memory_cap_in_bytes: usize) -> MemoryCappedHashmap {
            MemoryCappedHashmap {
                inner: clru::CLruCache::with_config(
                    clru::CLruCacheConfig::new(NonZeroUsize::new(memory_cap_in_bytes).expect("non zero"))
                        .with_scale(CustomScale),
                ),
                free_list: Vec::new(),
                debug: git_features::cache::Debug::new(format!("MemoryCappedHashmap({}B)", memory_cap_in_bytes)),
            }
        }
    }

    impl DecodeEntry for MemoryCappedHashmap {
        fn put(&mut self, pack_id: u32, offset: u64, data: &[u8], kind: git_object::Kind, compressed_size: usize) {
            self.debug.put();
            if let Ok(Some(previous_entry)) = self.inner.put_with_weight(
                (pack_id, offset),
                Entry {
                    data: self
                        .free_list
                        .pop()
                        .map(|mut v| {
                            v.clear();
                            v.resize(data.len(), 0);
                            v.copy_from_slice(data);
                            v
                        })
                        .unwrap_or_else(|| Vec::from(data)),
                    kind,
                    compressed_size,
                },
            ) {
                self.free_list.push(previous_entry.data)
            }
        }

        fn get(&mut self, pack_id: u32, offset: u64, out: &mut Vec<u8>) -> Option<(git_object::Kind, usize)> {
            let res = self.inner.get(&(pack_id, offset)).map(|e| {
                out.resize(e.data.len(), 0);
                out.copy_from_slice(&e.data);
                (e.kind, e.compressed_size)
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

#[cfg(feature = "pack-cache-lru-dynamic")]
pub use memory::MemoryCappedHashmap;

#[cfg(feature = "pack-cache-lru-static")]
mod _static {
    use super::DecodeEntry;
    struct Entry {
        pack_id: u32,
        offset: u64,
        data: Vec<u8>,
        kind: git_object::Kind,
        compressed_size: usize,
    }

    /// A cache using a least-recently-used implementation capable of storing the `SIZE` most recent objects.
    /// The cache must be small as the search is 'naive' and the underlying data structure is a linked list.
    /// Values of 64 seem to improve performance.
    pub struct StaticLinkedList<const SIZE: usize> {
        inner: uluru::LRUCache<Entry, SIZE>,
        free_list: Vec<Vec<u8>>,
        debug: git_features::cache::Debug,
    }

    impl<const SIZE: usize> Default for StaticLinkedList<SIZE> {
        fn default() -> Self {
            StaticLinkedList {
                inner: Default::default(),
                free_list: Vec::new(),
                debug: git_features::cache::Debug::new(format!("StaticLinkedList<{}>", SIZE)),
            }
        }
    }

    impl<const SIZE: usize> DecodeEntry for StaticLinkedList<SIZE> {
        fn put(&mut self, pack_id: u32, offset: u64, data: &[u8], kind: git_object::Kind, compressed_size: usize) {
            self.debug.put();
            if let Some(previous) = self.inner.insert(Entry {
                offset,
                pack_id,
                data: self
                    .free_list
                    .pop()
                    .map(|mut v| {
                        v.clear();
                        v.resize(data.len(), 0);
                        v.copy_from_slice(data);
                        v
                    })
                    .unwrap_or_else(|| Vec::from(data)),
                kind,
                compressed_size,
            }) {
                self.free_list.push(previous.data)
            }
        }

        fn get(&mut self, pack_id: u32, offset: u64, out: &mut Vec<u8>) -> Option<(git_object::Kind, usize)> {
            let res = self.inner.lookup(|e: &mut Entry| {
                if e.pack_id == pack_id && e.offset == offset {
                    out.resize(e.data.len(), 0);
                    out.copy_from_slice(&e.data);
                    Some((e.kind, e.compressed_size))
                } else {
                    None
                }
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

#[cfg(feature = "pack-cache-lru-static")]
pub use _static::StaticLinkedList;
