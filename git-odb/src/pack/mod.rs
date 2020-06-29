pub mod index;

mod cache {
    pub trait EntryCache {
        fn put(&mut self, offset: u64, data: &[u8], kind: git_object::Kind, compressed_size: usize);
        fn get(&mut self, offset: u64, out: &mut Vec<u8>) -> Option<(git_object::Kind, usize)>;
    }

    pub struct NoopEntryCache;

    impl EntryCache for NoopEntryCache {
        fn put(
            &mut self,
            _offset: u64,
            _data: &[u8],
            _kind: git_object::Kind,
            _compressed_size: usize,
        ) {
        }
        fn get(&mut self, _offset: u64, _out: &mut Vec<u8>) -> Option<(git_object::Kind, usize)> {
            None
        }
    }

    struct LRUCacheEntry {
        offset: u64,
        data: Vec<u8>,
        kind: git_object::Kind,
        compressed_size: usize,
    }

    #[derive(Default)]
    pub struct LRUEntryCache(uluru::LRUCache<[uluru::Entry<LRUCacheEntry>; 32]>);

    impl EntryCache for LRUEntryCache {
        fn put(
            &mut self,
            offset: u64,
            data: &[u8],
            kind: git_object::Kind,
            compressed_size: usize,
        ) {
            self.0.insert(LRUCacheEntry {
                offset,
                data: Vec::from(data),
                kind,
                compressed_size,
            })
        }

        fn get(&mut self, offset: u64, out: &mut Vec<u8>) -> Option<(git_object::Kind, usize)> {
            self.0.lookup(|e: &mut LRUCacheEntry| {
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
}

pub use cache::*;

mod file;

pub use self::file::*;
