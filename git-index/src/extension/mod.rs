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
    use crate::extension::Signature;
    use bstr::BString;
    use git_hash::ObjectId;

    pub type Paths = Vec<ResolvePath>;

    pub struct ResolvePath {
        /// relative to the root of the repository, or what would be stored in the index
        name: BString,

        /// common ancestor, stage 1
        common: Option<Stage>,
        /// stage 2
        ours: Option<Stage>,
        /// stage 3
        theirs: Option<Stage>,
    }

    pub struct Stage {
        mode: u32,
        id: ObjectId,
    }

    pub const SIGNATURE: Signature = *b"REUC";

    pub fn decode(data: &[u8], object_hash: git_hash::Kind) -> Option<Paths> {
        todo!("decode REUC")
    }
}

pub mod sparse {
    use crate::extension::Signature;

    /// Only used as an indicator
    pub const SIGNATURE: Signature = *b"sdir";
}
