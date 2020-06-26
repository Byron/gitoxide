use crate::pack;
use byteorder::{BigEndian, ByteOrder};
use filebuffer::FileBuffer;
use git_object::{self as object, SHA1_SIZE};
use quick_error::quick_error;
use std::{convert::TryFrom, mem::size_of, path::Path};

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
    pub offset: u64,
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
    }
}

pub struct File {
    data: FileBuffer,
    kind: Kind,
    version: u32,
    num_objects: u32,
    _fan: [u32; FAN_LEN],
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
            let mut hasher = crate::sha1::Sha1::default();
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
                for index_entry in self.iter() {
                    let pack_entry = pack.entry(index_entry.offset);
                    let object_kind = pack
                        .decode_entry(pack_entry, &mut buf, |id, _| {
                            unimplemented!("TODO: in-pack lookup of objects by SHA1: {}", id)
                        })
                        .map_err(|e| {
                            ChecksumError::PackDecode(e, index_entry.oid, index_entry.offset)
                        })?;
                    let mut header_buf = [0u8; 64];
                    let header_size = crate::loose::db::serde::write_header(
                        object_kind,
                        buf.len(),
                        &mut header_buf[..],
                    )
                    .expect("header buffer to be big enough");
                    let mut hasher = crate::sha1::Sha1::default();
                    hasher.update(&header_buf[..header_size]);
                    hasher.update(buf.as_slice());
                    let actual_oid = hasher.digest();
                    if actual_oid != index_entry.oid {
                        return Err(ChecksumError::PackObjectMismatch {
                            actual: actual_oid,
                            expected: index_entry.oid.clone(),
                            offset: index_entry.offset,
                            kind: object_kind,
                        });
                    }
                    // TODO: CRC32 (in-pack data)
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
                        offset: BigEndian::read_u32(ofs) as u64,
                        crc32: None,
                    }
                }),
            _ => unreachable!("Cannot use iter_v1() on index of type {:?}", self.kind),
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
                offset: {
                    let ofs32 = BigEndian::read_u32(ofs32);
                    if (ofs32 & N32_HIGH_BIT) == N32_HIGH_BIT {
                        let from = pack64_offset + (ofs32 ^ N32_HIGH_BIT) as usize * N64_SIZE;
                        BigEndian::read_u64(&self.data[from..from + N64_SIZE])
                    } else {
                        ofs32 as u64
                    }
                },
                crc32: Some(BigEndian::read_u32(crc32)),
            }),
            _ => unreachable!("Cannot use iter_v2() on index of type {:?}", self.kind),
        }
    }

    pub fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = Entry> + 'a> {
        match self.kind {
            Kind::V1 => Box::new(self.iter_v1()),
            Kind::V2 => Box::new(self.iter_v2()),
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
            _fan: fan,
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
