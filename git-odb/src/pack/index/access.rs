use crate::pack::index::{self, FAN_LEN};
use byteorder::{BigEndian, ByteOrder};
use git_object::{borrowed, owned, SHA1_SIZE};
use std::{
    convert::{TryFrom, TryInto},
    mem::size_of,
};

const N32_SIZE: usize = size_of::<u32>();
const N64_SIZE: usize = size_of::<u64>();
const V1_HEADER_SIZE: usize = FAN_LEN * N32_SIZE;
const V2_HEADER_SIZE: usize = N32_SIZE * 2 + FAN_LEN * N32_SIZE;
const N32_HIGH_BIT: u32 = 1 << 31;

pub(crate) type PackOffset = u64;

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Entry {
    pub oid: owned::Id,
    /// The offset to the object's header in the pack
    pub pack_offset: PackOffset,
    pub crc32: Option<u32>,
}

/// Iteration and access
impl index::File {
    pub fn iter_v1<'a>(&'a self) -> impl Iterator<Item = Entry> + 'a {
        match self.kind {
            index::Kind::V1 => self.data[V1_HEADER_SIZE..]
                .chunks(N32_SIZE + SHA1_SIZE)
                .take(self.num_objects as usize)
                .map(|c| {
                    let (ofs, oid) = c.split_at(N32_SIZE);
                    Entry {
                        oid: owned::Id::from_20_bytes(oid),
                        pack_offset: BigEndian::read_u32(ofs) as u64,
                        crc32: None,
                    }
                }),
            _ => panic!("Cannot use iter_v1() on index of type {:?}", self.kind),
        }
    }

    pub fn iter_v2<'a>(&'a self) -> impl Iterator<Item = Entry> + 'a {
        let pack64_offset = self.offset_pack_offset64_v2();
        match self.kind {
            index::Kind::V2 => izip!(
                self.data[V2_HEADER_SIZE..].chunks(SHA1_SIZE),
                self.data[self.offset_crc32_v2()..].chunks(N32_SIZE),
                self.data[self.offset_pack_offset_v2()..].chunks(N32_SIZE)
            )
            .take(self.num_objects as usize)
            .map(move |(oid, crc32, ofs32)| Entry {
                oid: owned::Id::from_20_bytes(oid),
                pack_offset: self.pack_offset_from_offset_v2(ofs32, pack64_offset),
                crc32: Some(BigEndian::read_u32(crc32)),
            }),
            _ => panic!("Cannot use iter_v2() on index of type {:?}", self.kind),
        }
    }

    /// Returns 20 bytes sha1 at the given index in our list of (sorted) sha1 hashes.
    /// The index ranges from 0 to self.num_objects()
    pub fn oid_at_index(&self, index: u32) -> borrowed::Id<'_> {
        let index: usize = index
            .try_into()
            .expect("an architecture able to hold 32 bits of integer");
        let start = match self.kind {
            index::Kind::V2 => V2_HEADER_SIZE + index * SHA1_SIZE,
            index::Kind::V1 => V1_HEADER_SIZE + index * (N32_SIZE + SHA1_SIZE) + N32_SIZE,
        };
        borrowed::Id::try_from(&self.data[start..start + SHA1_SIZE]).expect("20 bytes SHA1 to be alright")
    }

    pub fn pack_offset_at_index(&self, index: u32) -> PackOffset {
        let index: usize = index
            .try_into()
            .expect("an architecture able to hold 32 bits of integer");
        match self.kind {
            index::Kind::V2 => {
                let start = self.offset_pack_offset_v2() + index * N32_SIZE;
                self.pack_offset_from_offset_v2(&self.data[start..start + N32_SIZE], self.offset_pack_offset64_v2())
            }
            index::Kind::V1 => {
                let start = V1_HEADER_SIZE + index * (N32_SIZE + SHA1_SIZE);
                BigEndian::read_u32(&self.data[start..start + N32_SIZE]) as u64
            }
        }
    }

    pub fn crc32_at_index(&self, index: u32) -> Option<u32> {
        let index: usize = index
            .try_into()
            .expect("an architecture able to hold 32 bits of integer");
        match self.kind {
            index::Kind::V2 => {
                let start = self.offset_crc32_v2() + index * N32_SIZE;
                Some(BigEndian::read_u32(&self.data[start..start + N32_SIZE]))
            }
            index::Kind::V1 => None,
        }
    }

    /// Returns the offset of the given SHA1 for use with the `(oid|pack_offset|crc32)_at_index()`
    pub fn lookup(&self, id: borrowed::Id<'_>) -> Option<u32> {
        let first_byte = id.first_byte() as usize;
        let mut upper_bound = self.fan[first_byte];
        let mut lower_bound = if first_byte != 0 { self.fan[first_byte - 1] } else { 0 };

        // Bisect using indices
        // TODO: Performance of V2 could possibly be better if we would be able to do a binary search
        // on 20 byte chunks directly, but doing so requires transmuting and that is unsafe, even though
        // it should not be if the bytes match up and the type has no destructor.
        while lower_bound < upper_bound {
            let mid = (lower_bound + upper_bound) / 2;
            let mid_sha = self.oid_at_index(mid);

            use std::cmp::Ordering::*;
            match id.cmp(&mid_sha) {
                Less => upper_bound = mid,
                Equal => return Some(mid),
                Greater => lower_bound = mid + 1,
            }
        }
        None
    }

    pub fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = Entry> + 'a> {
        match self.kind {
            index::Kind::V2 => Box::new(self.iter_v2()),
            index::Kind::V1 => Box::new(self.iter_v1()),
        }
    }

    pub fn sorted_offsets(&self) -> Vec<PackOffset> {
        let mut ofs: Vec<_> = match self.kind {
            index::Kind::V1 => self.iter().map(|e| e.pack_offset).collect(),
            index::Kind::V2 => {
                let mut v = Vec::with_capacity(self.num_objects as usize);
                let mut ofs32 = &self.data[self.offset_pack_offset_v2()..];
                let pack_offset_64 = self.offset_pack_offset64_v2();
                for _ in 0..self.num_objects {
                    v.push(self.pack_offset_from_offset_v2(ofs32, pack_offset_64));
                    ofs32 = &ofs32[4..];
                }
                v
            }
        };
        ofs.sort();
        ofs
    }

    fn offset_crc32_v2(&self) -> usize {
        V2_HEADER_SIZE + self.num_objects as usize * SHA1_SIZE
    }

    fn offset_pack_offset_v2(&self) -> usize {
        self.offset_crc32_v2() + self.num_objects as usize * N32_SIZE
    }

    fn offset_pack_offset64_v2(&self) -> usize {
        self.offset_pack_offset_v2() + self.num_objects as usize * N32_SIZE
    }

    fn pack_offset_from_offset_v2(&self, offset: &[u8], pack64_offset: usize) -> PackOffset {
        debug_assert_eq!(self.kind, index::Kind::V2);
        let ofs32 = BigEndian::read_u32(offset);
        if (ofs32 & N32_HIGH_BIT) == N32_HIGH_BIT {
            let from = pack64_offset + (ofs32 ^ N32_HIGH_BIT) as usize * N64_SIZE;
            BigEndian::read_u64(&self.data[from..from + N64_SIZE])
        } else {
            ofs32 as u64
        }
    }
}
