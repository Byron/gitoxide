use std::mem::size_of;

use crate::{
    data,
    index::{self, EntryIndex, FAN_LEN},
};

const N32_SIZE: usize = size_of::<u32>();
const N64_SIZE: usize = size_of::<u64>();
const V1_HEADER_SIZE: usize = FAN_LEN * N32_SIZE;
const V2_HEADER_SIZE: usize = N32_SIZE * 2 + FAN_LEN * N32_SIZE;
const N32_HIGH_BIT: u32 = 1 << 31;

/// Represents an entry within a pack index file, effectively mapping object [`IDs`][git_hash::ObjectId] to pack data file locations.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Entry {
    /// The ID of the object
    pub oid: git_hash::ObjectId,
    /// The offset to the object's header in the pack data file
    pub pack_offset: data::Offset,
    /// The CRC32 hash over all bytes of the pack data entry.
    ///
    /// This can be useful for direct copies of pack data entries from one pack to another with insurance there was no bit rot.
    /// _Note_: Only available in index version 2 or newer
    pub crc32: Option<u32>,
}

/// Iteration and access
impl index::File {
    fn iter_v1(&self) -> impl Iterator<Item = Entry> + '_ {
        match self.version {
            index::Version::V1 => self.data[V1_HEADER_SIZE..]
                .chunks(N32_SIZE + self.hash_len)
                .take(self.num_objects as usize)
                .map(|c| {
                    let (ofs, oid) = c.split_at(N32_SIZE);
                    Entry {
                        oid: git_hash::ObjectId::from(oid),
                        pack_offset: crate::read_u32(ofs) as u64,
                        crc32: None,
                    }
                }),
            _ => panic!("Cannot use iter_v1() on index of type {:?}", self.version),
        }
    }

    fn iter_v2(&self) -> impl Iterator<Item = Entry> + '_ {
        let pack64_offset = self.offset_pack_offset64_v2();
        match self.version {
            index::Version::V2 => izip!(
                self.data[V2_HEADER_SIZE..].chunks(self.hash_len),
                self.data[self.offset_crc32_v2()..].chunks(N32_SIZE),
                self.data[self.offset_pack_offset_v2()..].chunks(N32_SIZE)
            )
            .take(self.num_objects as usize)
            .map(move |(oid, crc32, ofs32)| Entry {
                oid: git_hash::ObjectId::from(oid),
                pack_offset: self.pack_offset_from_offset_v2(ofs32, pack64_offset),
                crc32: Some(crate::read_u32(crc32)),
            }),
            _ => panic!("Cannot use iter_v2() on index of type {:?}", self.version),
        }
    }

    /// Returns the object hash at the given index in our list of (sorted) sha1 hashes.
    /// The index ranges from 0 to self.num_objects()
    ///
    /// # Panics
    ///
    /// If `index` is out of bounds.
    pub fn oid_at_index(&self, index: EntryIndex) -> &git_hash::oid {
        let index = index as usize;
        let start = match self.version {
            index::Version::V2 => V2_HEADER_SIZE + index * self.hash_len,
            index::Version::V1 => V1_HEADER_SIZE + index * (N32_SIZE + self.hash_len) + N32_SIZE,
        };
        git_hash::oid::from_bytes_unchecked(&self.data[start..][..self.hash_len])
    }

    /// Returns the offset into our pack data file at which to start reading the object at `index`.
    ///
    /// # Panics
    ///
    /// If `index` is out of bounds.
    pub fn pack_offset_at_index(&self, index: EntryIndex) -> data::Offset {
        let index = index as usize;
        match self.version {
            index::Version::V2 => {
                let start = self.offset_pack_offset_v2() + index * N32_SIZE;
                self.pack_offset_from_offset_v2(&self.data[start..][..N32_SIZE], self.offset_pack_offset64_v2())
            }
            index::Version::V1 => {
                let start = V1_HEADER_SIZE + index * (N32_SIZE + self.hash_len);
                crate::read_u32(&self.data[start..][..N32_SIZE]) as u64
            }
        }
    }

    /// Returns the CRC32 of the object at the given `index`.
    ///
    /// _Note_: These are always present for index version 2 or higher.
    /// # Panics
    ///
    /// If `index` is out of bounds.
    pub fn crc32_at_index(&self, index: EntryIndex) -> Option<u32> {
        let index = index as usize;
        match self.version {
            index::Version::V2 => {
                let start = self.offset_crc32_v2() + index * N32_SIZE;
                Some(crate::read_u32(&self.data[start..start + N32_SIZE]))
            }
            index::Version::V1 => None,
        }
    }

    /// Returns the `index` of the given hash for use with the [`oid_at_index()`][index::File::oid_at_index()],
    /// [`pack_offset_at_index()`][index::File::pack_offset_at_index()] or [`crc32_at_index()`][index::File::crc32_at_index()].
    pub fn lookup(&self, id: impl AsRef<git_hash::oid>) -> Option<EntryIndex> {
        let id = id.as_ref();
        let first_byte = id.first_byte() as usize;
        let mut upper_bound = self.fan[first_byte];
        let mut lower_bound = if first_byte != 0 { self.fan[first_byte - 1] } else { 0 };

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

    /// An iterator over all [`Entries`][Entry] of this index file.
    pub fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = Entry> + 'a> {
        match self.version {
            index::Version::V2 => Box::new(self.iter_v2()),
            index::Version::V1 => Box::new(self.iter_v1()),
        }
    }

    /// Return a vector of ascending offsets into our respective pack data file.
    ///
    /// Useful to control an iteration over all pack entries in a cache-friendly way.
    pub fn sorted_offsets(&self) -> Vec<data::Offset> {
        let mut ofs: Vec<_> = match self.version {
            index::Version::V1 => self.iter().map(|e| e.pack_offset).collect(),
            index::Version::V2 => {
                let offset32_start = &self.data[self.offset_pack_offset_v2()..];
                let pack_offset_64_start = self.offset_pack_offset64_v2();
                offset32_start
                    .chunks(N32_SIZE)
                    .take(self.num_objects as usize)
                    .map(|offset| self.pack_offset_from_offset_v2(offset, pack_offset_64_start))
                    .collect()
            }
        };
        ofs.sort_unstable();
        ofs
    }

    #[inline]
    fn offset_crc32_v2(&self) -> usize {
        V2_HEADER_SIZE + self.num_objects as usize * self.hash_len
    }

    #[inline]
    fn offset_pack_offset_v2(&self) -> usize {
        self.offset_crc32_v2() + self.num_objects as usize * N32_SIZE
    }

    #[inline]
    fn offset_pack_offset64_v2(&self) -> usize {
        self.offset_pack_offset_v2() + self.num_objects as usize * N32_SIZE
    }

    #[inline]
    fn pack_offset_from_offset_v2(&self, offset: &[u8], pack64_offset: usize) -> data::Offset {
        debug_assert_eq!(self.version, index::Version::V2);
        let ofs32 = crate::read_u32(offset);
        if (ofs32 & N32_HIGH_BIT) == N32_HIGH_BIT {
            let from = pack64_offset + (ofs32 ^ N32_HIGH_BIT) as usize * N64_SIZE;
            crate::read_u64(&self.data[from..][..N64_SIZE])
        } else {
            ofs32 as u64
        }
    }
}
