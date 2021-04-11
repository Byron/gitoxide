/// A trait to model putting objects at a given pack `offset` into a cache, and fetching them.
///
/// It is used to speed up [pack traversals][crate::pack::index::File::traverse()].
pub trait DecodeEntry {
    /// Store a fully decoded object at `offset` of `kind` with `compressed_size` and `data` in the cache.
    ///
    /// It is up to the cache implementation whether that actually happens or not.
    fn put(&mut self, offset: u64, data: &[u8], kind: git_object::Kind, compressed_size: usize);
    /// Attempt to fetch the object at `offset` and store its decoded bytes in `out`, as previously stored with [`DecodeEntry::put()`], and return
    /// its (object `kind`, `decompressed_size`)
    fn get(&mut self, offset: u64, out: &mut Vec<u8>) -> Option<(git_object::Kind, usize)>;
}

/// A cache that stores nothing and retrieves nothing.
#[derive(Default)]
pub struct Noop;

impl DecodeEntry for Noop {
    fn put(&mut self, _offset: u64, _data: &[u8], _kind: git_object::Kind, _compressed_size: usize) {}
    fn get(&mut self, _offset: u64, _out: &mut Vec<u8>) -> Option<(git_object::Kind, usize)> {
        None
    }
}

/// A cache using a least-recently-used implementation capable of storing the 64 most recent objects.
struct LruEntry {
    offset: u64,
    data: Vec<u8>,
    kind: git_object::Kind,
    compressed_size: usize,
}

/// A least-recently-used cache to accelerate pack traversal
#[derive(Default)]
pub struct Lru<const SIZE: usize>(uluru::LRUCache<LruEntry, SIZE>);

impl<const SIZE: usize> DecodeEntry for Lru<SIZE> {
    fn put(&mut self, offset: u64, data: &[u8], kind: git_object::Kind, compressed_size: usize) {
        self.0.insert(LruEntry {
            offset,
            data: Vec::from(data),
            kind,
            compressed_size,
        })
    }

    fn get(&mut self, offset: u64, out: &mut Vec<u8>) -> Option<(git_object::Kind, usize)> {
        self.0.lookup(|e: &mut LruEntry| {
            if e.offset == offset {
                out.resize(e.data.len(), 0);
                out.copy_from_slice(&e.data);
                Some((e.kind, e.compressed_size))
            } else {
                None
            }
        })
    }
}
