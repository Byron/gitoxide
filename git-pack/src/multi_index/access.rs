use crate::multi_index::File;
use std::convert::{TryFrom, TryInto};
use std::path::{Path, PathBuf};

impl File {
    pub fn path(&self) -> &Path {
        &self.path
    }
    pub fn num_packs(&self) -> u32 {
        self.num_packs
    }
    pub fn num_objects(&self) -> u32 {
        self.num_objects
    }
    pub fn object_hash(&self) -> git_hash::Kind {
        self.object_hash
    }
    pub fn checksum(&self) -> git_hash::ObjectId {
        git_hash::ObjectId::from(&self.data[self.data.len() - self.hash_len..])
    }
    pub fn index_names(&self) -> &[PathBuf] {
        &self.index_names
    }
}

impl File {
    pub fn oid_at_index(&self, index: u32) -> &git_hash::oid {
        debug_assert!(index < self.num_objects, "index out of bounds");
        let index: usize = index as usize;
        let start = self.lookup_ofs + index * self.hash_len;
        git_hash::oid::from_bytes_unchecked(&self.data[start..][..self.hash_len])
    }

    pub fn lookup(&self, id: impl AsRef<git_hash::oid>) -> Option<u32> {
        let id = id.as_ref();
        let first_byte = id.first_byte() as usize;
        let mut upper_bound = self.fan[first_byte];
        let mut lower_bound = if first_byte != 0 { self.fan[first_byte - 1] } else { 0 };

        // Bisect using indices
        // TODO: Performance of V2 could possibly be better if we would be able to do a binary search
        // on 20 byte chunks directly, but doing so requires transmuting and that is not safe, even though
        // it should not be if the bytes match up and the type has no destructor.
        while lower_bound < upper_bound {
            let mid = (lower_bound + upper_bound) / 2;
            let mid_sha = self.oid_at_index(mid);

            use std::cmp::Ordering::*;
            match id.cmp(mid_sha) {
                Less => upper_bound = mid,
                Equal => return Some(mid),
                Greater => lower_bound = mid + 1,
            }
        }
        None
    }
}
