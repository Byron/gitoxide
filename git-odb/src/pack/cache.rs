/// A trait to model putting objects at a given pack `offset` into a cache, and fetching them.
///
/// It is used to speed up [pack traversals][crate::pack::index::File::traverse()].
pub trait DecodeEntry {
    /// Store a fully decoded object at `offset` of `kind` with `compressed_size` and `data` in the cache.
    ///
    /// It is up to the cache implementation whether that actually happens or not.
    fn put(&mut self, pack_id: u32, offset: u64, data: &[u8], kind: git_object::Kind, compressed_size: usize);
    /// Attempt to fetch the object at `offset` and store its decoded bytes in `out`, as previously stored with [`DecodeEntry::put()`], and return
    /// its (object `kind`, `decompressed_size`)
    fn get(&mut self, pack_id: u32, offset: u64, out: &mut Vec<u8>) -> Option<(git_object::Kind, usize)>;
}

/// A cache that stores nothing and retrieves nothing.
#[derive(Default)]
pub struct Noop;

impl DecodeEntry for Noop {
    fn put(&mut self, _pack_id: u32, _offset: u64, _data: &[u8], _kind: git_object::Kind, _compressed_size: usize) {}
    fn get(&mut self, _pack_id: u32, _offset: u64, _out: &mut Vec<u8>) -> Option<(git_object::Kind, usize)> {
        None
    }
}

/// Various implementations of [`DecodeEntry`] using least-recently-used algorithms.
#[cfg(any(feature = "pack-cache-lru-dynamic", feature = "pack-cache-lru-static"))]
pub mod lru {
    use super::DecodeEntry;

    #[cfg(feature = "pack-cache-lru-dynamic")]
    mod memory {
        use super::DecodeEntry;
        struct Entry {
            data: Vec<u8>,
            kind: git_object::Kind,
            compressed_size: usize,
        }

        impl memory_lru::ResidentSize for Entry {
            fn resident_size(&self) -> usize {
                self.data.len()
            }
        }

        /// An LRU cache with hash map backing and an eviction rule based on the memory usage for object data in bytes.
        pub struct MemoryCappedHashmap(memory_lru::MemoryLruCache<(u32, u64), Entry>);

        impl MemoryCappedHashmap {
            /// Return a new instance which evicts least recently used items if it uses more than `memory_cap_in_bytes`
            /// object data.
            pub fn new(memory_cap_in_bytes: usize) -> MemoryCappedHashmap {
                MemoryCappedHashmap(memory_lru::MemoryLruCache::new(memory_cap_in_bytes))
            }
        }

        impl DecodeEntry for MemoryCappedHashmap {
            fn put(&mut self, pack_id: u32, offset: u64, data: &[u8], kind: git_object::Kind, compressed_size: usize) {
                self.0.insert(
                    (pack_id, offset),
                    Entry {
                        data: Vec::from(data),
                        kind,
                        compressed_size,
                    },
                )
            }

            fn get(&mut self, pack_id: u32, offset: u64, out: &mut Vec<u8>) -> Option<(git_object::Kind, usize)> {
                self.0.get(&(pack_id, offset)).map(|e| {
                    out.resize(e.data.len(), 0);
                    out.copy_from_slice(&e.data);
                    (e.kind, e.compressed_size)
                })
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
        #[derive(Default)]
        pub struct StaticLinkedList<const SIZE: usize>(uluru::LRUCache<Entry, SIZE>);

        impl<const SIZE: usize> DecodeEntry for StaticLinkedList<SIZE> {
            fn put(&mut self, pack_id: u32, offset: u64, data: &[u8], kind: git_object::Kind, compressed_size: usize) {
                self.0.insert(Entry {
                    offset,
                    pack_id,
                    data: Vec::from(data),
                    kind,
                    compressed_size,
                })
            }

            fn get(&mut self, pack_id: u32, offset: u64, out: &mut Vec<u8>) -> Option<(git_object::Kind, usize)> {
                self.0.lookup(|e: &mut Entry| {
                    if e.pack_id == pack_id && e.offset == offset {
                        out.resize(e.data.len(), 0);
                        out.copy_from_slice(&e.data);
                        Some((e.kind, e.compressed_size))
                    } else {
                        None
                    }
                })
            }
        }
    }
    #[cfg(feature = "pack-cache-lru-static")]
    pub use _static::StaticLinkedList;
}
