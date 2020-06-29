use crate::{
    pack::EntryCache,
    pack::{Entry, Error, File},
    zlib::Inflate,
};
use git_object as object;
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
    OutOfPack { kind: object::Kind, end: usize },
}

/// Reading of objects
impl File {
    // Note that this method does not resolve deltified objects, but merely decompresses their content
    // `out` is expected to be large enough to hold `entry.size` bytes.
    pub fn decompress_entry(&self, entry: &Entry, out: &mut [u8]) -> Result<usize, Error> {
        assert!(
            out.len() as u64 >= entry.decompressed_size,
            "output buffer isn't large enough to hold decompressed result, want {}, have {}",
            entry.decompressed_size,
            out.len()
        );

        self.decompress_entry_from_data_offset(entry.data_offset, out)
    }

    /// Decompress the object expected at the given data offset, sans pack header. This information is only
    /// known after the pack header was parsed.
    /// Note that this method does not resolve deltified objects, but merely decompresses their content
    /// `out` is expected to be large enough to hold `entry.size` bytes.
    /// Returns the amount of packed bytes there were decompressed into `out`
    fn decompress_entry_from_data_offset(
        &self,
        data_offset: u64,
        out: &mut [u8],
    ) -> Result<usize, Error> {
        let offset: usize = data_offset
            .try_into()
            .expect("offset representable by machine");
        assert!(offset < self.data.len(), "entry offset out of bounds");

        Inflate::default()
            .once(&self.data[offset..], &mut std::io::Cursor::new(out), true)
            .map_err(|e| Error::ZlibInflate(e, "Failed to decompress pack entry"))
            .map(|(_, consumed_in, _)| consumed_in)
    }

    /// Decode an entry, resolving delta's as needed, while growing the output vector if there is not enough
    /// space to hold the result object.
    /// Returns (object_kind, compressed_size), referring to the `entry` in-pack size for use with CRC32 checks
    /// such as in `crc32(pack_data[entry.data_offset..entry.data_offset + compressed_size])`
    pub fn decode_entry(
        &self,
        entry: Entry,
        out: &mut Vec<u8>,
        resolve: impl Fn(&object::Id, &mut Vec<u8>) -> Option<ResolvedBase>,
        cache: impl EntryCache,
    ) -> Result<(object::Kind, usize), Error> {
        use crate::pack::decoded::Header::*;
        match entry.header {
            Tree | Blob | Commit | Tag => {
                out.resize(
                    entry
                        .decompressed_size
                        .try_into()
                        .expect("size representable by machine"),
                    0,
                );
                self.decompress_entry(&entry, out.as_mut_slice())
                    .map(|consumed_input| {
                        (
                            entry.header.to_kind().expect("a non-delta entry"),
                            consumed_input,
                        )
                    })
            }
            OfsDelta { .. } | RefDelta { .. } => self.resolve_deltas(entry, resolve, out, cache),
        }
    }

    fn resolve_deltas(
        &self,
        last: Entry,
        resolve: impl Fn(&object::Id, &mut Vec<u8>) -> Option<ResolvedBase>,
        out: &mut Vec<u8>,
        cache: impl EntryCache,
    ) -> Result<(object::Kind, usize), Error> {
        use crate::pack::decoded::Header;
        // all deltas, from the one that produces the desired object (first) to the oldest at the end of the chain
        let mut chain = SmallVec::<[Delta; 10]>::default();
        let mut cursor = last;
        let mut base_buffer_size: Option<usize> = None;
        let mut object_kind: Option<object::Kind> = None;
        let mut consumed_input: Option<usize> = None;

        // Find the first full base, either an undeltified object in the pack or a reference to another object.
        let mut total_delta_data_size: u64 = 0;
        while cursor.header.is_delta() {
            if let Some((kind, packed_size)) = cache.get(cursor.data_offset, out) {
                object_kind = Some(kind);
                // If the input entry is a cache hit, keep the packed size as it must be returned.
                // Otherwise, the packed size will be determined later when decompressing the input delta
                if total_delta_data_size == 0 {
                    consumed_input = Some(packed_size);
                }
                break;
            }
            total_delta_data_size += cursor.decompressed_size;
            let decompressed_size = cursor
                .decompressed_size
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
                    Some(ResolvedBase::InPack(entry)) => entry,
                    Some(ResolvedBase::OutOfPack { end, kind }) => {
                        base_buffer_size = Some(end);
                        object_kind = Some(kind);
                        break;
                    }
                    None => return Err(Error::DeltaBaseUnresolved(oid)),
                },
                _ => unreachable!("cursor.is_delta() only allows deltas here"),
            };
        }

        if chain.is_empty() {
            return Ok((
                object_kind.expect("object kind as set by cache"),
                consumed_input.expect("consumed bytes as set by cache"),
            ));
        };

        // First pass will decompress all delta data and keep it in our output buffer
        // [<possibly resolved base object>]<delta-1..delta-n>...
        // so that we can find the biggest result size.
        let total_delta_data_size: usize = total_delta_data_size
            .try_into()
            .expect("delta data to fit in memory");

        let chain_len = chain.len();
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
            for (delta_idx, delta) in chain.iter_mut().rev().enumerate() {
                let consumed_from_data_offset = self.decompress_entry_from_data_offset(
                    delta.data_offset,
                    &mut instructions[..delta.decompressed_size],
                )?;
                if delta_idx + 1 == chain_len {
                    consumed_input = Some(consumed_from_data_offset);
                }

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
                if delta_range.start < end {
                    // …this means that the delta size is even larger than two uncompressed worst-case
                    // intermediate results combined. It would already be undesireable to have it bigger
                    // then the target size (as you could just store the object in whole).
                    // However, this just means that it reuses existing deltas smartly, which as we rightfully
                    // remember stand for an object each. However, this means a lot of data is read to restore
                    // a single object sometimes. Fair enough - package size is minimized that way.
                    out.copy_within(delta_range, end);
                } else {
                    let (buffers, instructions) = out.split_at_mut(end);
                    instructions.copy_from_slice(&buffers[delta_range]);
                }
                end
            };

            // If we don't have a out-of-pack object already, fill the base-buffer by decompressing the full object
            // at which the cursor is left after the iteration
            if base_buffer_size.is_none() {
                let base_entry = cursor;
                debug_assert!(!base_entry.header.is_delta());
                object_kind = base_entry.header.to_kind();
                let packed_size =
                    self.decompress_entry_from_data_offset(base_entry.data_offset, out)?;
                cache.put(
                    base_entry.data_offset,
                    &out[..base_entry
                        .decompressed_size
                        .try_into()
                        .expect("successful decompression should make this successful too")],
                    object_kind.expect("non-delta object"),
                    packed_size,
                );
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
            if delta_idx + 1 == chain_len {
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
            // this seems inverted, but remember: we swapped the buffers on the last iteration
            target_buf[..last_result_size].copy_from_slice(&source_buf[..last_result_size]);
        }
        out.resize(last_result_size, 0);

        let object_kind = object_kind
            .expect("a base object as root of any delta chain that we are here to resolve");
        let consumed_input = consumed_input.expect("at least one decompressed delta object");
        cache.put(
            chain.first().expect("at least one delta").data_offset,
            out.as_slice(),
            object_kind,
            consumed_input,
        );
        Ok((object_kind, consumed_input))
    }
}

fn apply_delta(base: &[u8], mut target: &mut [u8], data: &[u8]) {
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
            0 => panic!("encountered unsupported command code: 0"),
            size => {
                std::io::Write::write(&mut target, &data[i..i + *size as usize])
                    .expect("delta copy data: slice sizes to match up");
                i += *size as usize;
            }
        }
    }
    assert_eq!(i, data.len());
    assert_eq!(target.len(), 0);
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
