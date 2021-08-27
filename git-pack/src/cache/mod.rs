use std::ops::DerefMut;

use git_object::Kind;

/// A trait to model putting objects at a given pack `offset` into a cache, and fetching them.
///
/// It is used to speed up [pack traversals][crate::index::File::traverse()].
pub trait DecodeEntry {
    /// Store a fully decoded object at `offset` of `kind` with `compressed_size` and `data` in the cache.
    ///
    /// It is up to the cache implementation whether that actually happens or not.
    fn put(&mut self, pack_id: u32, offset: u64, data: &[u8], kind: git_object::Kind, compressed_size: usize);
    /// Attempt to fetch the object at `offset` and store its decoded bytes in `out`, as previously stored with [`DecodeEntry::put()`], and return
    /// its (object `kind`, `decompressed_size`)
    fn get(&mut self, pack_id: u32, offset: u64, out: &mut Vec<u8>) -> Option<(git_object::Kind, usize)>;
}

/// A cache that stores nothing and retrieves nothing, thus it _never_ caches.
#[derive(Default)]
pub struct Never;

impl DecodeEntry for Never {
    fn put(&mut self, _pack_id: u32, _offset: u64, _data: &[u8], _kind: git_object::Kind, _compressed_size: usize) {}
    fn get(&mut self, _pack_id: u32, _offset: u64, _out: &mut Vec<u8>) -> Option<(git_object::Kind, usize)> {
        None
    }
}

impl<T: DecodeEntry + ?Sized> DecodeEntry for Box<T> {
    fn put(&mut self, pack_id: u32, offset: u64, data: &[u8], kind: Kind, compressed_size: usize) {
        self.deref_mut().put(pack_id, offset, data, kind, compressed_size)
    }

    fn get(&mut self, pack_id: u32, offset: u64, out: &mut Vec<u8>) -> Option<(Kind, usize)> {
        self.deref_mut().get(pack_id, offset, out)
    }
}

/// Various implementations of [`DecodeEntry`] using least-recently-used algorithms.
#[cfg(any(feature = "pack-cache-lru-dynamic", feature = "pack-cache-lru-static"))]
pub mod lru;
