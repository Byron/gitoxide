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

pub struct Link {
    pub shared_index_checksum: git_hash::ObjectId,
    pub bitmaps: Option<link::Bitmaps>,
}

mod iter;

pub(crate) mod decode;

pub(crate) mod tree;

pub(crate) mod end_of_index_entry;

pub(crate) mod index_entry_offset_table;

pub mod link;

pub(crate) mod resolve_undo {
    use bstr::BString;
    use git_hash::ObjectId;

    use crate::{
        extension::Signature,
        util::{split_at_byte_exclusive, split_at_pos},
    };

    pub type Paths = Vec<ResolvePath>;

    pub struct ResolvePath {
        /// relative to the root of the repository, or what would be stored in the index
        name: BString,

        /// 0 = ancestor/common, 1 = ours, 2 = theirs
        stages: [Option<Stage>; 3],
    }

    pub struct Stage {
        mode: u32,
        id: ObjectId,
    }

    pub const SIGNATURE: Signature = *b"REUC";

    pub fn decode(mut data: &[u8], object_hash: git_hash::Kind) -> Option<Paths> {
        let hash_len = object_hash.len_in_bytes();
        let mut out = Vec::new();

        while !data.is_empty() {
            let (path, rest) = split_at_byte_exclusive(data, 0)?;
            data = rest;

            let mut modes = [0u32; 3];
            for mode in modes.iter_mut() {
                let (mode_ascii, rest) = split_at_byte_exclusive(data, 0)?;
                data = rest;
                *mode = u32::from_str_radix(std::str::from_utf8(mode_ascii).ok()?, 8).ok()?;
            }

            let mut stages = [None, None, None];
            for (mode, stage) in modes.into_iter().zip(stages.iter_mut()) {
                if mode == 0 {
                    continue;
                }
                let (hash, rest) = split_at_pos(data, hash_len)?;
                data = rest;
                *stage = Some(Stage {
                    mode,
                    id: ObjectId::from(hash),
                });
            }

            out.push(ResolvePath {
                name: path.into(),
                stages,
            });
        }
        out.into()
    }
}

pub mod sparse {
    use crate::extension::Signature;

    /// Only used as an indicator
    pub const SIGNATURE: Signature = *b"sdir";
}
