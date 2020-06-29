pub mod index;

pub trait EntryCache {
    fn put(&self, offset: u64, data: &[u8], kind: git_object::Kind, compressed_size: usize);
    fn get(&self, offset: u64, out: &mut Vec<u8>) -> Option<(git_object::Kind, usize)>;
}

pub struct NoopEntryCache;

impl EntryCache for NoopEntryCache {
    fn put(&self, _offset: u64, _data: &[u8], _kind: git_object::Kind, _compressed_size: usize) {}
    fn get(&self, _offset: u64, _out: &mut Vec<u8>) -> Option<(git_object::Kind, usize)> {
        None
    }
}

mod file;

pub use self::file::*;
