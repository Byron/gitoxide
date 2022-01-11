use smallvec::SmallVec;

use crate::{util::from_be_u32, Version};

const MIN_SIZE: usize = 4 /* signature */ + 4 /* size */;

pub type Signature = [u8; 4];

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

pub struct Iter<'a> {
    data: &'a [u8],
    pub consumed: usize,
}
