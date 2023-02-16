use crate::{extension, extension::Signature, util::read_u32};

#[derive(Debug, Clone, Copy)]
pub struct Offset {
    pub from_beginning_of_file: u32,
    pub num_entries: u32,
}

pub const SIGNATURE: Signature = *b"IEOT";

pub fn decode(data: &[u8]) -> Option<Vec<Offset>> {
    let (version, mut data) = read_u32(data)?;
    match version {
        1 => {}
        _unknown => return None,
    }

    let entry_size = 4 + 4;
    let num_offsets = data.len() / entry_size;
    if num_offsets == 0 || data.len() % entry_size != 0 {
        return None;
    }

    let mut out = Vec::with_capacity(entry_size);
    for _ in 0..num_offsets {
        let (offset, chunk) = read_u32(data)?;
        let (num_entries, chunk) = read_u32(chunk)?;
        out.push(Offset {
            from_beginning_of_file: offset,
            num_entries,
        });
        data = chunk;
    }
    debug_assert!(data.is_empty());

    out.into()
}

pub fn find(extensions: &[u8], object_hash: gix_hash::Kind) -> Option<Vec<Offset>> {
    extension::Iter::new_without_checksum(extensions, object_hash)?
        .find_map(|(sig, ext_data)| (sig == SIGNATURE).then_some(ext_data))
        .and_then(decode)
}
