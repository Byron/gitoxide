use crate::data;
use crate::multi_index::{File, PackIndex};
use byteorder::{BigEndian, ByteOrder};
use std::convert::{TryFrom, TryInto};
use std::path::{Path, PathBuf};

/// Represents an entry within a multi index file, effectively mapping object [`IDs`][git_hash::ObjectId] to pack data files and the offset
/// within.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Entry {
    /// The ID of the object.
    pub oid: git_hash::ObjectId,
    /// The offset to the object's header in the pack data file.
    pub pack_offset: data::Offset,
    /// The index of the pack matching our [`File::index_names()`] slice.
    pub pack_index: PackIndex,
}

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

    pub fn pack_offset_and_pack_id_at_index(&self, index: u32) -> (PackIndex, data::Offset) {
        const OFFSET_ENTRY_SIZE: usize = 4 + 4;
        let index = index as usize;
        let start = self.offsets_ofs + index * OFFSET_ENTRY_SIZE;

        const HIGH_BIT: u32 = 1 << 31;

        let pack_index = BigEndian::read_u32(&self.data[start..][..4]);
        let offset = &self.data[start + 4..][..4];
        let ofs32 = BigEndian::read_u32(offset);
        let pack_offset = if (ofs32 & HIGH_BIT) == HIGH_BIT {
            let offsets_64 = self
                .large_offsets_ofs
                .expect("non-malformed file that has large offsets if these are contained");
            let from = offsets_64 + (ofs32 ^ HIGH_BIT) as usize * 8;
            BigEndian::read_u64(&self.data[from..][..8])
        } else {
            ofs32 as u64
        };
        (pack_index, pack_offset)
    }

    pub fn iter(&self) -> impl Iterator<Item = Entry> + '_ {
        (0..self.num_objects).map(move |idx| {
            let (pack_index, pack_offset) = self.pack_offset_and_pack_id_at_index(idx);
            Entry {
                oid: self.oid_at_index(idx).to_owned(),
                pack_offset,
                pack_index,
            }
        })
    }
}
