use crate::{hash, pack, pack::index::V2_SIGNATURE};
use byteorder::{BigEndian, WriteBytesExt};
use git_object::owned;
use std::io;

pub(crate) fn to_write(
    out: impl io::Write,
    entries_sorted_by_oid: Vec<(u64, owned::Id, u32)>,
    pack_hash: &owned::Id,
    kind: pack::index::Kind,
) -> io::Result<owned::Id> {
    use io::Write;
    assert!(
        !entries_sorted_by_oid.is_empty(),
        "Empty packs do not exists, or so I think"
    );
    assert_eq!(kind, pack::index::Kind::V2, "Can only write V2 packs right now");
    assert!(
        entries_sorted_by_oid.len() <= u32::MAX as usize,
        "a pack cannot have more than u32::MAX objects"
    );

    // Write header
    let mut out = hash::Write::new(out, kind.hash());
    out.write_all(V2_SIGNATURE)?;
    out.write_u32::<BigEndian>(kind as u32)?;

    const LARGE_OFFSET_THRESHOLD: u64 = 0x7fff_ffff;
    const HIGH_BIT: u32 = 0x8000_0000;

    let needs_64bit_offsets = entries_sorted_by_oid.last().expect("at least one pack entry").0 > LARGE_OFFSET_THRESHOLD;
    let mut fan_out_be = [0u32; 256];

    let mut iter = entries_sorted_by_oid.iter();
    for (offset, byte) in fan_out_be.iter_mut().zip(0u8..=255) {
        let pos = iter
            .position(|(_, id, _)| id.as_slice()[0] != byte)
            .unwrap_or_else(|| entries_sorted_by_oid.len());
        *offset = pos as u32;
    }

    // SAFETY: It's safe to interpret 4BE bytes * 256 into 1byte * 1024 for the purpose of writing
    #[allow(unsafe_code)]
    out.write_all(unsafe { std::mem::transmute::<&[u32; 256], &[u8; 256 * 4]>(&fan_out_be) })?;

    for (_, id, _) in &entries_sorted_by_oid {
        out.write_all(id.as_slice())?;
    }
    for (_, _, crc32) in &entries_sorted_by_oid {
        out.write_u32::<BigEndian>(*crc32)?;
    }

    {
        let mut offsets64_be = Vec::<u64>::new();
        for (pack_offset, _, _) in &entries_sorted_by_oid {
            out.write_u32::<BigEndian>(if needs_64bit_offsets && *pack_offset > LARGE_OFFSET_THRESHOLD {
                assert!(
                    offsets64_be.len() < LARGE_OFFSET_THRESHOLD as usize,
                    "Encoding breakdown - way too many 64bit offsets"
                );
                offsets64_be.push(pack_offset.to_be());
                (offsets64_be.len() as u32) & HIGH_BIT
            } else {
                *pack_offset as u32
            })?;
        }
        if needs_64bit_offsets {
            // SAFETY: It's safe to interpret 8BE bytes * N as 1byte * N * 8 for the purpose of writing
            #[allow(unsafe_code)]
            out.write_all(unsafe {
                std::slice::from_raw_parts(offsets64_be.as_ptr() as *const u8, offsets64_be.len() * 8)
            })?;
        }
    }

    out.write_all(pack_hash.as_slice())?;

    let index_hash: owned::Id = out.hash.digest().into();
    out.inner.write_all(index_hash.as_slice())?;

    Ok(index_hash)
}
