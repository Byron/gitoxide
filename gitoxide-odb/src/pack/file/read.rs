use crate::{
    pack::{Entry, Error, File},
    zlib::Inflate,
};
use git_object::Id;
use smallvec::SmallVec;
use std::convert::TryInto;

#[derive(Debug)]
struct Delta {
    decompressed_size: usize,
    data_offset: u64,
}

#[derive(Debug)]
pub enum ResolvedBase {
    InPack(Entry),
    OutOfPack { end: usize },
}

/// Reading of objects
impl File {
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
        let mut cursor = last.clone();
        let mut base_buffer_size: Option<usize> = None;
        // Find the first full base, either an undeltified object in the pack or a reference to another object.
        while cursor.header.is_delta() {
            chain.push(Delta {
                decompressed_size: cursor
                    .size
                    .try_into()
                    .expect("delta sizes small enough to fit a usize"),
                data_offset: cursor.data_offset,
            });
            cursor = match cursor.header {
                Header::OfsDelta { pack_offset } => self.entry(pack_offset),
                Header::RefDelta { oid } => match resolve(&oid, out) {
                    ResolvedBase::InPack(entry) => entry,
                    ResolvedBase::OutOfPack { end } => {
                        base_buffer_size = Some(end);
                        break;
                    }
                },
                _ => unreachable!("cursor.is_delta() only allows deltas here"),
            };
        }
        let (first_buffer_end, second_buffer_end) = {
            let biggest_delta_instructions_size: u64 = chain
                .iter()
                .map(|d| d.decompressed_size as u64)
                .max()
                .expect("at least one delta");
            let base_buffer_size = match base_buffer_size {
                None => {
                    let base_entry = cursor;
                    out.resize(
                        (base_entry.size * 2 + biggest_delta_instructions_size) // * 2 for worst-case guess
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
                Some(size) => {
                    out.resize(
                        (size as u64 * 2 // * 2 for worst-case guess
                            + biggest_delta_instructions_size)
                            .try_into()
                            .expect("usize to be big enough for all deltas"),
                        0,
                    );
                    size
                }
            };
            (base_buffer_size, base_buffer_size * 2) // works because we have two equally sized sequential regions
        };

        // From oldest to most recent, apply all deltas, swapping the buffer back and forth
        // TODO: once we have more tests, we could optimize this memory-intensive work to
        // analyse the delta-chains to only copy data once.
        // `out` is: [source-buffer][target-buffer][max-delta-instructions-buffer]
        let (buffers, instructions) = out.split_at_mut(second_buffer_end);
        let (mut source_buf, mut target_buf) = buffers.split_at_mut(first_buffer_end);

        let mut last_result_size = None;
        for (
            delta_idx,
            Delta {
                decompressed_size,
                data_offset: pack_offset,
            },
        ) in chain.iter().rev().enumerate()
        {
            let data = &mut instructions[..*decompressed_size];
            self.decompress_entry_inner(*pack_offset, data)?;
            let (base_size, data) = delta_header_size(data);
            let (result_size, data) = delta_header_size(data);
            if delta_idx + 1 == chain.len() {
                last_result_size = Some(result_size);
            }
            apply_delta(
                &source_buf[..base_size as usize],
                &mut target_buf[..result_size as usize],
                data,
            );
            // use the target as source for the next delta
            std::mem::swap(&mut source_buf, &mut target_buf);
        }

        let last_result_size = last_result_size.expect("at least one delta chain item") as usize;
        // uneven chains leave the target buffer after the source buffer
        if chain.len() % 2 == 1 {
            source_buf[..last_result_size].copy_from_slice(&target_buf[..last_result_size]);
        }
        out.resize(last_result_size, 0);
        Ok(())
    }
}

fn apply_delta(base: &[u8], mut target: &mut [u8], mut data: &[u8]) {
    let mut i = 0;
    while let Some(cmd) = data.get(i) {
        i += 1;
        match cmd {
            cmd if cmd & 0b1000_0000 != 0 => {
                let (mut ofs, mut size): (u32, u32) = (0, 0);
                if cmd & 0b0000_0001 != 0 {
                    ofs = data[i] as u32;
                    i += 1;
                }
                if cmd & 0b0000_0010 != 0 {
                    ofs |= (data[i] as u32) << 8;
                    i += 1;
                }
                if cmd & 0b0000_0100 != 0 {
                    ofs |= (data[i] as u32) << 16;
                    i += 1;
                }
                if cmd & 0b0000_1000 != 0 {
                    ofs |= (data[i] as u32) << 24;
                    i += 1;
                }
                if cmd & 0b0001_0000 != 0 {
                    size = data[i] as u32;
                    i += 1;
                }
                if cmd & 0b0010_0000 != 0 {
                    size |= (data[i] as u32) << 8;
                    i += 1;
                }
                if cmd & 0b0100_0000 != 0 {
                    size |= (data[i] as u32) << 16;
                    i += 1;
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
                let (dest, rest) = data.split_at(*size as usize);
                std::io::Write::write(&mut target, dest)
                    .expect("delta copy data: slice sizes to match up");
                data = rest;
            }
        }
    }
}

fn delta_header_size(d: &[u8]) -> (u64, &[u8]) {
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
    (size, &d[consumed..])
}
