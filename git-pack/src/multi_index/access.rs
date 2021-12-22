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
}
