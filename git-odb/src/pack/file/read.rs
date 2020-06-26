use crate::{
    pack::{Entry, Error, File},
    zlib::Inflate,
};
use git_object::Id;
use smallvec::SmallVec;
use std::{convert::TryInto, ops::Range};

#[derive(Debug)]
struct Delta {
    data: Range<usize>,
    base_size: usize,
    result_size: usize,

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

        self.decompress_entry_from_data_offset(entry.data_offset, out)
    }

    // Decompress the object expected at the given data offset, sans pack header. This information is only
    // known after the pack header was parsed.
    // Note that this method does not resolve deltified objects, but merely decompresses their content
    // `out` is expected to be large enough to hold `entry.size` bytes.
    fn decompress_entry_from_data_offset(
        &self,
        data_offset: u64,
        out: &mut [u8],
    ) -> Result<(), Error> {
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
        let mut total_delta_data_size: u64 = 0;
        while cursor.header.is_delta() {
            total_delta_data_size += cursor.size;
            let decompressed_size = cursor
                .size
                .try_into()
                .expect("a single delta size small enough to fit a usize");
            chain.push(Delta {
                data: Range {
                    start: 0,
                    end: decompressed_size,
                },
                base_size: 0,
                result_size: 0,
                decompressed_size,
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

        // First pass will decompress all delta data and keep it in our output buffer
        // [<possibly resolved base object>]<delta-1..delta-n>...
        // so that we can find the biggest result size.
        let total_delta_data_size: usize = total_delta_data_size
            .try_into()
            .expect("delta data to fit in memory");

        let (first_buffer_end, second_buffer_end) = {
            let delta_start = base_buffer_size.unwrap_or(0);
            out.resize(delta_start + total_delta_data_size, 0);

            let delta_range = Range {
                start: delta_start,
                end: delta_start + total_delta_data_size,
            };
            let mut instructions = &mut out[delta_range.clone()];
            let mut relative_delta_start = 0;
            let mut biggest_result_size = 0;
            for delta in chain.iter_mut().rev() {
                self.decompress_entry_from_data_offset(
                    delta.data_offset,
                    &mut instructions[..delta.decompressed_size],
                )?;

                let (base_size, offset) = delta_header_size_ofs(instructions);
                let mut bytes_consumed_by_header = offset;
                biggest_result_size = biggest_result_size.max(base_size);
                delta.base_size = base_size.try_into().expect("base size fits into usize");

                let (result_size, offset) = delta_header_size_ofs(&instructions[offset..]);
                bytes_consumed_by_header += offset;
                biggest_result_size = biggest_result_size.max(result_size);
                delta.result_size = result_size.try_into().expect("result size fits into usize");

                // the absolute location into the instructions buffer, so we keep track of the end point of the last
                delta.data.start = relative_delta_start + bytes_consumed_by_header;
                relative_delta_start += delta.decompressed_size;
                delta.data.end = relative_delta_start;

                instructions = &mut instructions[delta.decompressed_size..];
            }

            // Now we can produce a buffer like this
            // [<biggest-result-buffer, possibly filled with resolved base object data>]<biggest-result-buffer><delta-1..delta-n>
            // from [<possibly resolved base object>]<delta-1..delta-n>...
            let biggest_result_size: usize = biggest_result_size
                .try_into()
                .expect("biggest result size small enough to fit into usize");
            let first_buffer_size = biggest_result_size;
            let second_buffer_size = first_buffer_size;
            out.resize(
                first_buffer_size + second_buffer_size + total_delta_data_size,
                0,
            );

            // Now 'rescue' the deltas, because in the next step we possibly overwrite that portion
            // of memory with the base object (in the majority of cases)
            let second_buffer_end = {
                let end = first_buffer_size + second_buffer_size;
                let (buffers, instructions) = out.split_at_mut(end);
                instructions.copy_from_slice(&buffers[delta_range]);
                end
            };

            // If we don't have a out-of-pack object already, fill the base-buffer by decompressing the full object
            // at which the cursor is left after the iteration
            if let None = base_buffer_size {
                let base_entry = cursor;
                debug_assert!(!base_entry.header.is_delta());
                self.decompress_entry_from_data_offset(base_entry.data_offset, out)?;
            }

            (first_buffer_size, second_buffer_end)
        };

        // From oldest to most recent, apply all deltas, swapping the buffer back and forth
        // TODO: once we have more tests, we could optimize this memory-intensive work to
        // analyse the delta-chains to only copy data once - after all, with 'copy-from-base' deltas,
        // all data originates from one base at some point.
        // `out` is: [source-buffer][target-buffer][max-delta-instructions-buffer]
        let (buffers, instructions) = out.split_at_mut(second_buffer_end);
        let (mut source_buf, mut target_buf) = buffers.split_at_mut(first_buffer_end);

        let mut last_result_size = None;
        for (
            delta_idx,
            Delta {
                data,
                base_size,
                result_size,
                ..
            },
        ) in chain.iter().rev().enumerate()
        {
            let data = &mut instructions[data.clone()];
            if delta_idx + 1 == chain.len() {
                last_result_size = Some(*result_size);
            }
            apply_delta(
                &source_buf[..*base_size],
                &mut target_buf[..*result_size],
                data,
            );
            // use the target as source for the next delta
            std::mem::swap(&mut source_buf, &mut target_buf);
        }

        let last_result_size = last_result_size.expect("at least one delta chain item");
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

fn delta_header_size_ofs(d: &[u8]) -> (u64, usize) {
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
