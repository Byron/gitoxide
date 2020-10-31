pub trait DecodeEntry {
    fn put(&mut self, offset: u64, data: &[u8], kind: git_object::Kind, compressed_size: usize);
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
struct LRUEntry {
    offset: u64,
    data: Vec<u8>,
    kind: git_object::Kind,
    compressed_size: usize,
}

#[derive(Default)]
pub struct LRU(uluru::LRUCache<[uluru::Entry<LRUEntry>; 64]>);

impl DecodeEntry for LRU {
    fn put(&mut self, offset: u64, data: &[u8], kind: git_object::Kind, compressed_size: usize) {
        self.0.insert(LRUEntry {
            offset,
            data: Vec::from(data),
            kind,
            compressed_size,
        })
    }

    fn get(&mut self, offset: u64, out: &mut Vec<u8>) -> Option<(git_object::Kind, usize)> {
        self.0.lookup(|e: &mut LRUEntry| {
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
