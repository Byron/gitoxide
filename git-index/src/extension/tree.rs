use crate::extension::{Signature, Tree};

pub const SIGNATURE: Signature = *b"TREE";

pub struct NodeId {
    /// The id of the directory tree of the associated tree object.
    id: git_hash::ObjectId,
    /// The amount of non-tree entries contained within, and definitely not zero.
    entry_count: u32,
}

/// A recursive data structure
pub fn decode(data: &[u8], object_hash: git_hash::Kind) -> Option<Tree> {
    todo!("decode tree")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_of_tree() {
        assert_eq!(std::mem::size_of::<Tree>(), 88);
    }
}
