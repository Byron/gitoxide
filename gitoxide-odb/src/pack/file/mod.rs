use crate::zlib::Inflate;
use byteorder::{BigEndian, ByteOrder};
use filebuffer::FileBuffer;
use git_object::{Id, SHA1_SIZE};
use quick_error::quick_error;
use smallvec::SmallVec;
use std::convert::TryInto;
use std::ops::Range;
use std::{convert::TryFrom, mem::size_of, path::Path};

#[derive(Debug)]
struct Delta {
    data: Range<usize>,
    header_size: usize,
    base_size: u64,
    result_size: u64,
}

#[derive(Debug)]
pub enum ResolvedBase {
    InPack(Entry),
    OutOfPack { end: usize },
}

impl Delta {
    fn size(&self) -> usize {
        self.data.end - self.data.start
    }
}

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Io(err: std::io::Error, path: std::path::PathBuf) {
            display("Could not open pack file at '{}'", path.display())
            cause(err)
        }
        Corrupt(msg: String) {
            display("{}", msg)
        }
        UnsupportedVersion(version: u32) {
            display("Unsupported pack version: {}", version)
        }
        ZlibInflate(err: crate::zlib::Error, msg: &'static str) {
            display("{}", msg)
            cause(err)
        }
    }
}

const N32_SIZE: usize = size_of::<u32>();

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
pub enum Kind {
    V2,
    V3,
}

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
pub struct Entry {
    pub header: decoded::Header,
    /// The decompressed size of the object in bytes
    pub size: u64,
    /// absolute offset to compressed object data in the pack
    pub data_offset: u64,
}

pub struct File {
    data: FileBuffer,
    kind: Kind,
    num_objects: u32,
}

impl File {
    pub fn kind(&self) -> Kind {
        self.kind.clone()
    }
    pub fn num_objects(&self) -> u32 {
        self.num_objects
    }

    fn assure_v2(&self) {
        assert!(
            if let Kind::V2 = self.kind.clone() {
                true
            } else {
                false
            },
            "Only V2 is implemented"
        );
    }

    pub fn entry(&self, offset: u64) -> Entry {
        self.assure_v2();
        let pack_offset: usize = offset.try_into().expect("offset representable by machine");
        assert!(pack_offset <= self.data.len(), "offset out of bounds");

        let object_data = &self.data[pack_offset..];
        let (object, decompressed_size, consumed_bytes) =
            decoded::Header::from_bytes(object_data, offset);
        Entry {
            header: object,
            size: decompressed_size,
            data_offset: offset + consumed_bytes,
        }
    }

    pub fn at(path: impl AsRef<Path>) -> Result<File, Error> {
        File::try_from(path.as_ref())
    }

    // Note that this method does not resolve deltified objects, but merely decompresses their content
    // `out` is expected to be large enough to hold `entry.size` bytes.
    pub fn decompress_entry(&self, entry: &Entry, out: &mut [u8]) -> Result<(), Error> {
        assert!(
            out.len() as u64 >= entry.size,
            "output buffer isn't large enough to hold decompressed result, want {}, have {}",
            entry.size,
            out.len()
        );

        self.decompress_entry_inner(entry.data_offset, out)
    }

    // Note that this method does not resolve deltified objects, but merely decompresses their content
    // `out` is expected to be large enough to hold `entry.size` bytes.
    fn decompress_entry_inner(&self, data_offset: u64, out: &mut [u8]) -> Result<(), Error> {
        let offset: usize = data_offset
            .try_into()
            .expect("offset representable by machine");
        assert!(offset < self.data.len(), "entry offset out of bounds");

        Inflate::default()
            .once(&self.data[offset..], &mut std::io::Cursor::new(out), true)
            .map_err(|e| Error::ZlibInflate(e, "Failed to decompress pack entry"))
            .map(|_| ())
    }

    // Decode an entry, resolving delta's as needed, while growing the output vector if there is not enough
    // space to hold the result object.
    pub fn decode_entry(
        &self,
        entry: Entry,
        out: &mut Vec<u8>,
        resolve: impl Fn(&Id, &mut Vec<u8>) -> ResolvedBase,
    ) -> Result<(), Error> {
        use crate::pack::decoded::Header::*;
        match entry.header {
            Tree | Blob | Commit | Tag => {
                out.resize(
                    entry
                        .size
                        .try_into()
                        .expect("size representable by machine"),
                    0,
                );
                self.decompress_entry(&entry, out.as_mut_slice())
            }
            OfsDelta { .. } | RefDelta { .. } => self.resolve_deltas(entry, resolve, out),
        }
    }

    fn resolve_deltas(
        &self,
        last: Entry,
        resolve: impl Fn(&Id, &mut Vec<u8>) -> ResolvedBase,
        out: &mut Vec<u8>,
    ) -> Result<(), Error> {
        use crate::pack::decoded::Header;
        // all deltas, from the one that produces the desired object (first) to the oldest at the end of the chain
        let mut chain = SmallVec::<[Delta; 5]>::default();
        let mut instruction_buffer_size = 0usize;
        let mut cursor = last.clone();
        let mut base_buffer_end: Option<usize> = None;
        // Find the first full base, either an undeltified object in the pack or a reference to another object.
        while cursor.header.is_delta() {
            let end = instruction_buffer_size
                .checked_add(cursor.size as usize)
                .expect("no overflow");
            chain.push(Delta {
                data: Range {
                    start: instruction_buffer_size,
                    end,
                },
                header_size: 0,
                base_size: cursor.data_offset, // keep this value around for later
                result_size: 0,
            });
            instruction_buffer_size = end;
            cursor = match cursor.header {
                Header::OfsDelta { pack_offset } => self.entry(pack_offset),
                Header::RefDelta { oid } => match resolve(&oid, out) {
                    ResolvedBase::InPack(entry) => entry,
                    ResolvedBase::OutOfPack { end } => {
                        base_buffer_end = Some(end);
                        break;
                    }
                },
                _ => unreachable!("cursor.is_delta() only allows deltas here"),
            };
        }
        let (first_buffer_end, second_buffer_end) = {
            let delta_instructions_size: u64 = chain.iter().map(|d| d.size() as u64).sum();
            let base_buffer_end = match base_buffer_end {
                None => {
                    let base_entry = cursor;
                    out.resize(
                        (base_entry.size * 2 + delta_instructions_size) // * 2 for worst-case guess
                            .try_into()
                            .expect("usize to be big enough for all deltas"),
                        0,
                    );
                    self.decompress_entry_inner(base_entry.data_offset, out)?;
                    base_entry
                        .size
                        .try_into()
                        .expect("usize big enough for single entry base object size")
                }
                Some(end) => {
                    out.resize(
                        (end as u64 * 2 // * 2 for worst-case guess
                            + delta_instructions_size)
                            .try_into()
                            .expect("usize to be big enough for all deltas"),
                        0,
                    );
                    end
                }
            };
            (base_buffer_end, base_buffer_end + base_buffer_end) // works because we have two equally sized sequential regions
        };

        // move the instructions offsets to a range where they won't be overwritten, past the second result buffer
        // conceptually, `out` is: [source-buffer][target-buffer][delta-1..delta-n]
        for delta in chain.iter_mut() {
            let data = Range {
                start: second_buffer_end + delta.data.start,
                end: second_buffer_end + delta.data.end,
            };
            let buf = &mut out[data];
            self.decompress_entry_inner(
                delta.base_size, // == entry.data_offset; we just use the slot to carry over necessary information
                buf,
            )?;
            let (base_size, consumed) = delta_header_size(buf);
            delta.header_size += consumed;
            delta.base_size = base_size;
            let (result_size, consumed) = delta_header_size(&buf[consumed..]);
            delta.header_size += consumed;
            delta.result_size = result_size;
        }

        // From oldest to most recent, apply all deltas, swapping the buffer back and forth
        // TODO: once we have more tests, we could optimize this memory-intensive work to
        // analyse the delta-chains to only copy data once.
        let (buffers, instructions) = out.split_at_mut(second_buffer_end);
        let (mut source_buf, mut target_buf) = buffers.split_at_mut(first_buffer_end);

        for Delta {
            data,
            header_size,
            base_size,
            result_size,
        } in chain.iter().rev()
        {
            apply_delta(
                &source_buf[..*base_size as usize],
                &mut target_buf[..*result_size as usize],
                &instructions[data.start + header_size..data.end],
            );
            std::mem::swap(&mut source_buf, &mut target_buf);
        }

        let result_size = chain
            .first()
            .expect("at least one delta chain item")
            .result_size as usize;
        // uneven chains leave the target buffer after the source buffer
        if chain.len() % 2 == 1 {
            source_buf.copy_from_slice(&target_buf[..result_size]);
        }
        out.resize(result_size, 0);
        Ok(())
    }
}

fn apply_delta(mut base: &[u8], mut target: &mut [u8], instructions: &[u8]) {
    let mut iter = instructions.into_iter();
    while let Some(cmd) = iter.next() {
        match cmd {
            cmd if cmd & 0b1000_0000 != 0 => {
                let (mut ofs, mut size): (u32, u32) = (0, 0);
                if cmd & 0b0000_0001 != 0 {
                    ofs = *iter.next().unwrap() as u32;
                }
                if cmd & 0b0000_0010 != 0 {
                    ofs |= (*iter.next().unwrap() as u32) << 8;
                }
                if cmd & 0b0000_0100 != 0 {
                    ofs |= (*iter.next().unwrap() as u32) << 16;
                }
                if cmd & 0b0000_1000 != 0 {
                    ofs |= (*iter.next().unwrap() as u32) << 24;
                }
                if cmd & 0b0001_0000 != 0 {
                    size = *iter.next().unwrap() as u32;
                }
                if cmd & 0b0010_0000 != 0 {
                    size |= (*iter.next().unwrap() as u32) << 8;
                }
                if cmd & 0b0100_0000 != 0 {
                    size |= (*iter.next().unwrap() as u32) << 16;
                }
                if size == 0 {
                    size = 0x10000; // 65536
                }
                let ofs = ofs as usize;
                std::io::Write::write(&mut target, &base[ofs..ofs + size as usize])
                    .expect("delta copy from base: byte slices must match");
            }
            0 => panic!("encounted unsupported command code: 0"),
            size => {
                // Fixme: must copy from instruction buffer!
                let (dest, rest) = base.split_at(*size as usize);
                std::io::Write::write(&mut target, dest)
                    .expect("delta copy data: slice sizes to match up");
                base = rest;
            }
        }
    }
}

fn delta_header_size(d: &[u8]) -> (u64, usize) {
    let mut i = 0;
    let mut size = 0u64;
    let mut consumed = 0;
    for cmd in d.iter() {
        consumed += 1;
        size |= (*cmd as u64 & 0x7f) << i;
        i += 7;
        if *cmd & 0x80 == 0 {
            break;
        }
    }
    (size, consumed)
}

impl TryFrom<&Path> for File {
    type Error = Error;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let data = FileBuffer::open(path).map_err(|e| Error::Io(e, path.to_owned()))?;
        let pack_len = data.len();
        if pack_len < N32_SIZE * 3 + SHA1_SIZE {
            return Err(Error::Corrupt(format!(
                "Pack file of size {} is too small for even an empty pack",
                pack_len
            )));
        }
        let mut ofs = 0;
        if &data[ofs..ofs + b"PACK".len()] != b"PACK" {
            return Err(Error::Corrupt("Pack file type not recognized".into()));
        }
        ofs += N32_SIZE;
        let kind = match BigEndian::read_u32(&data[ofs..ofs + N32_SIZE]) {
            2 => Kind::V2,
            3 => Kind::V3,
            v => return Err(Error::UnsupportedVersion(v)),
        };
        ofs += N32_SIZE;
        let num_objects = BigEndian::read_u32(&data[ofs..ofs + N32_SIZE]);

        Ok(File {
            data,
            kind,
            num_objects,
        })
    }
}

pub mod decoded;
