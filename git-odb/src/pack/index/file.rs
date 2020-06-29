use crate::pack;
use crate::pack::{NoopEntryCache, ResolvedBase};
use byteorder::{BigEndian, ByteOrder};
use filebuffer::FileBuffer;
use git_object::{self as object, SHA1_SIZE};
use quick_error::quick_error;
use std::{convert::TryFrom, convert::TryInto, mem::size_of, path::Path};

const V2_SIGNATURE: &[u8] = b"\xfftOc";
const FOOTER_SIZE: usize = SHA1_SIZE * 2;
const N32_SIZE: usize = size_of::<u32>();
const N64_SIZE: usize = size_of::<u64>();
const FAN_LEN: usize = 256;
const V1_HEADER_SIZE: usize = FAN_LEN * N32_SIZE;
const V2_HEADER_SIZE: usize = N32_SIZE * 2 + FAN_LEN * N32_SIZE;
const N32_HIGH_BIT: u32 = 1 << 31;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Io(err: std::io::Error, path: std::path::PathBuf) {
            display("Could not open pack index file at '{}'", path.display())
            cause(err)
        }
        Corrupt(msg: String) {
            display("{}", msg)
        }
        UnsupportedVersion(version: u32) {
            display("Unsupported index version: {}", version)
        }
    }
}

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
pub enum Kind {
    V1,
    V2,
}

impl Default for Kind {
    fn default() -> Self {
        Kind::V2
    }
}

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
pub struct Entry {
    pub oid: object::Id,
    /// The offset of the object's header in the pack
    pub pack_offset: u64,
    pub crc32: Option<u32>,
}

quick_error! {
    #[derive(Debug)]
    pub enum ChecksumError {
        Mismatch { expected: object::Id, actual: object::Id } {
            display("index checksum mismatch: expected {}, got {}", expected, actual)
        }
        PackChecksum(err: pack::ChecksumError) {
            display("The pack of this index file failed to verify its checksums")
            from()
            cause(err)
        }
        PackDecode(err: pack::Error, id: object::Id, offset: u64) {
            display("Object {} at offset {} could not be decoded", id, offset)
            cause(err)
        }
        PackMismatch { expected: object::Id, actual: object::Id } {
            display("The packfiles checksum didn't match the index file checksum: expected {}, got {}", expected, actual)
        }
        PackObjectMismatch { expected: object::Id, actual: object::Id, offset: u64, kind: object::Kind} {
            display("The SHA1 of {} object at offset {} didn't match the checksum in the index file: expected {}, got {}", kind, offset, expected, actual)
        }
        Crc32Mismatch { expected: u32, actual: u32, offset: u64, kind: object::Kind} {
            display("The CRC32 of {} object at offset {} didn't match the checksum in the index file: expected {}, got {}", kind, offset, expected, actual)
        }
    }
}

pub struct File {
    data: FileBuffer,
    kind: Kind,
    version: u32,
    num_objects: u32,
    fan: [u32; FAN_LEN],
}

impl File {
    pub fn kind(&self) -> Kind {
        self.kind.clone()
    }
    pub fn num_objects(&self) -> u32 {
        self.num_objects
    }
    pub fn version(&self) -> u32 {
        self.version
    }
    pub fn checksum_of_index(&self) -> object::Id {
        object::Id::from_20_bytes(&self.data[self.data.len() - SHA1_SIZE..])
    }

    /// If `pack` is provided, it is expected (and validated to be) the pack belonging to this index.
    /// It will be used to validate internal integrity of the pack before checking each objects integrity
    /// is indeed as advertised via its SHA1 as stored in this index, as well as the CRC32 hash.
    #[cfg(any(feature = "fast-sha1", feature = "minimal-sha1"))]
    pub fn verify_checksum_of_index(
        &self,
        pack: Option<&pack::File>,
    ) -> Result<object::Id, ChecksumError> {
        let verify_self = || {
            let mut hasher = crate::hash::Sha1::default();
            hasher.update(&self.data[..self.data.len() - SHA1_SIZE]);
            let actual = hasher.digest();

            let expected = self.checksum_of_index();
            if actual == expected {
                Ok(actual)
            } else {
                Err(ChecksumError::Mismatch { actual, expected })
            }
        };
        match pack {
            None => verify_self(),
            Some(pack) => {
                if self.checksum_of_pack() != pack.checksum() {
                    return Err(ChecksumError::PackMismatch {
                        actual: pack.checksum(),
                        expected: self.checksum_of_pack(),
                    });
                }
                pack.verify_checksum()?;
                let id = verify_self()?;

                let mut buf = Vec::with_capacity(2048);
                let index_entries = {
                    let mut v: Vec<_> = self.iter().collect();
                    v.sort_by_key(|e| e.pack_offset);
                    v
                };

                for index_entry in index_entries.into_iter() {
                    let pack_entry = pack.entry(index_entry.pack_offset);
                    let pack_entry_data_offset = pack_entry.data_offset;
                    let (object_kind, consumed_input) = pack
                        .decode_entry(
                            pack_entry,
                            &mut buf,
                            |id, _| {
                                self.lookup_index(&id).map(|index| {
                                    ResolvedBase::InPack(
                                        pack.entry(self.pack_offset_at_index(index)),
                                    )
                                })
                            },
                            NoopEntryCache,
                        )
                        .map_err(|e| {
                            ChecksumError::PackDecode(e, index_entry.oid, index_entry.pack_offset)
                        })?;

                    let mut header_buf = [0u8; 64];
                    let header_size = crate::loose::db::serde::write_header(
                        object_kind,
                        buf.len(),
                        &mut header_buf[..],
                    )
                    .expect("header buffer to be big enough");
                    let mut hasher = crate::hash::Sha1::default();
                    hasher.update(&header_buf[..header_size]);
                    hasher.update(buf.as_slice());
                    let actual_oid = hasher.digest();
                    if actual_oid != index_entry.oid {
                        return Err(ChecksumError::PackObjectMismatch {
                            actual: actual_oid,
                            expected: index_entry.oid.clone(),
                            offset: index_entry.pack_offset,
                            kind: object_kind,
                        });
                    }
                    if let Some(desired_crc32) = index_entry.crc32 {
                        let actual_crc32 = pack.entry_crc32(
                            index_entry.pack_offset,
                            (pack_entry_data_offset - index_entry.pack_offset) as usize
                                + consumed_input,
                        );
                        if actual_crc32 != desired_crc32 {
                            return Err(ChecksumError::Crc32Mismatch {
                                actual: actual_crc32,
                                expected: desired_crc32,
                                offset: index_entry.pack_offset,
                                kind: object_kind,
                            });
                        }
                    }
                }
                Ok(id)
            }
        }
    }

    pub fn checksum_of_pack(&self) -> object::Id {
        let from = self.data.len() - SHA1_SIZE * 2;
        object::Id::from_20_bytes(&self.data[from..from + SHA1_SIZE])
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

    pub fn iter_v1<'a>(&'a self) -> impl Iterator<Item = Entry> + 'a {
        match self.kind {
            Kind::V1 => self.data[V1_HEADER_SIZE..]
                .chunks(N32_SIZE + SHA1_SIZE)
                .take(self.num_objects as usize)
                .map(|c| {
                    let (ofs, oid) = c.split_at(N32_SIZE);
                    Entry {
                        oid: object::Id::from_20_bytes(oid),
                        pack_offset: BigEndian::read_u32(ofs) as u64,
                        crc32: None,
                    }
                }),
            _ => panic!("Cannot use iter_v1() on index of type {:?}", self.kind),
        }
    }

    fn pack_offset_from_offset_v2(&self, offset: &[u8], pack64_offset: usize) -> u64 {
        debug_assert_eq!(self.kind, Kind::V2);
        let ofs32 = BigEndian::read_u32(offset);
        if (ofs32 & N32_HIGH_BIT) == N32_HIGH_BIT {
            let from = pack64_offset + (ofs32 ^ N32_HIGH_BIT) as usize * N64_SIZE;
            BigEndian::read_u64(&self.data[from..from + N64_SIZE])
        } else {
            ofs32 as u64
        }
    }

    pub fn iter_v2<'a>(&'a self) -> impl Iterator<Item = Entry> + 'a {
        let pack64_offset = self.offset_pack_offset64_v2();
        match self.kind {
            Kind::V2 => izip!(
                self.data[V2_HEADER_SIZE..].chunks(SHA1_SIZE),
                self.data[self.offset_crc32_v2()..].chunks(N32_SIZE),
                self.data[self.offset_pack_offset_v2()..].chunks(N32_SIZE)
            )
            .take(self.num_objects as usize)
            .map(move |(oid, crc32, ofs32)| Entry {
                oid: object::Id::from_20_bytes(oid),
                pack_offset: self.pack_offset_from_offset_v2(ofs32, pack64_offset),
                crc32: Some(BigEndian::read_u32(crc32)),
            }),
            _ => panic!("Cannot use iter_v2() on index of type {:?}", self.kind),
        }
    }

    /// Returns 20 bytes sha1 at the given index in our list of (sorted) sha1 hashes.
    /// The index ranges from 0 to self.num_objects()
    pub fn oid_at_index(&self, index: u32) -> &[u8] {
        let index: usize = index
            .try_into()
            .expect("an architecture able to hold 32 bits of integer");
        let start = match self.kind {
            Kind::V2 => V2_HEADER_SIZE + index * SHA1_SIZE,
            Kind::V1 => V1_HEADER_SIZE + index * (N32_SIZE + SHA1_SIZE) + N32_SIZE,
        };
        &self.data[start..start + SHA1_SIZE]
    }

    pub fn pack_offset_at_index(&self, index: u32) -> u64 {
        let index: usize = index
            .try_into()
            .expect("an architecture able to hold 32 bits of integer");
        match self.kind {
            Kind::V2 => {
                let start = self.offset_pack_offset_v2() + index * N32_SIZE;
                self.pack_offset_from_offset_v2(
                    &self.data[start..start + N32_SIZE],
                    self.offset_pack_offset64_v2(),
                )
            }
            Kind::V1 => {
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
            Kind::V2 => {
                let start = self.offset_crc32_v2() + index * N32_SIZE;
                Some(BigEndian::read_u32(&self.data[start..start + N32_SIZE]))
            }
            Kind::V1 => None,
        }
    }

    /// Returns the offset of the given object for use with the `(oid|pack_offset|crc32)_at_index()`
    pub fn lookup_index(&self, id: &[u8]) -> Option<u32> {
        let first_byte = id[0] as usize;
        let mut upper_bound = self.fan[first_byte];
        let mut lower_bound = if first_byte != 0 {
            self.fan[first_byte - 1]
        } else {
            0
        };

        // Bisect using indices
        // TODO: Performance of V2 could possibly be better if we would be able to do a binary search
        // on 20 byte chunks directly, but doing so requires transmuting and that is unsafe, even though
        // it should not be if the bytes match up and the type has no destructor.
        while lower_bound < upper_bound {
            let mid = (lower_bound + upper_bound) / 2;
            let mid_sha = self.oid_at_index(mid);

            if id < mid_sha {
                upper_bound = mid;
            } else if id == mid_sha {
                return Some(mid);
            } else {
                lower_bound = mid + 1;
            }
        }
        None
    }

    pub fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = Entry> + 'a> {
        match self.kind {
            Kind::V2 => Box::new(self.iter_v2()),
            Kind::V1 => Box::new(self.iter_v1()),
        }
    }

    pub fn at(path: impl AsRef<Path>) -> Result<File, Error> {
        Self::try_from(path.as_ref())
    }
}

impl TryFrom<&Path> for File {
    type Error = Error;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let data = FileBuffer::open(path).map_err(|e| Error::Io(e, path.to_owned()))?;
        let idx_len = data.len();
        if idx_len < FAN_LEN * N32_SIZE + FOOTER_SIZE {
            return Err(Error::Corrupt(format!(
                "Pack index of size {} is too small for even an empty index",
                idx_len
            )));
        }
        let (kind, version, fan, num_objects) = {
            let (kind, d) = {
                let (sig, d) = data.split_at(V2_SIGNATURE.len());
                if sig == V2_SIGNATURE {
                    (Kind::V2, d)
                } else {
                    (Kind::V1, &data[..])
                }
            };
            let (version, d) = {
                let (mut v, mut d) = (1, d);
                if let Kind::V2 = kind {
                    let (vd, dr) = d.split_at(N32_SIZE);
                    d = dr;
                    v = BigEndian::read_u32(vd);
                    if v != 2 {
                        return Err(Error::UnsupportedVersion(v));
                    }
                }
                (v, d)
            };
            let (fan, bytes_read) = read_fan(d);
            let (_, _d) = d.split_at(bytes_read);
            let num_objects = fan[FAN_LEN - 1];

            (kind, version, fan, num_objects)
        };
        Ok(File {
            data,
            kind,
            num_objects,
            version,
            fan,
        })
    }
}

fn read_fan(d: &[u8]) -> ([u32; FAN_LEN], usize) {
    let mut fan = [0; FAN_LEN];
    for (c, f) in d.chunks(N32_SIZE).zip(fan.iter_mut()) {
        *f = BigEndian::read_u32(c);
    }
    (fan, FAN_LEN * N32_SIZE)
}
