use smallvec::SmallVec;

const MIN_SIZE: usize = 4 /* signature */ + 4 /* size */;

pub type Signature = [u8; 4];

pub struct Iter<'a> {
    data: &'a [u8],
    pub consumed: usize,
}

/// A structure to associate object ids of a tree with sections in the index entries list.
///
/// It allows to more quickly build trees by avoiding as it can quickly re-use portions of the index and its associated tree ids
/// if there wa sno change to them. Portions of this tree are invalidated as the index is changed.
pub struct Tree {
    name: SmallVec<[u8; 23]>,
    /// Only set if there are any entries in the index we are associated with.
    id: Option<tree::NodeId>,
    children: Vec<Tree>,
}

mod iter;

pub(crate) mod decode;

pub(crate) mod tree;

pub(crate) mod end_of_index_entry;

pub(crate) mod index_entry_offset_table {
    use crate::{extension, extension::Signature, util::read_u32};

    #[derive(Debug)]
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

    pub fn find(extensions: &[u8], object_hash: git_hash::Kind) -> Option<Vec<Offset>> {
        extension::Iter::new_without_checksum(extensions, object_hash)?
            .find_map(|(sig, ext_data)| (sig == SIGNATURE).then(|| ext_data))
            .and_then(decode)
    }
}
