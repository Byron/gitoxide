use crate::extension::{Signature, Tree};
use crate::util::split_at_byte_exclusive;
use git_hash::ObjectId;

pub const SIGNATURE: Signature = *b"TREE";

pub struct NodeId {
    /// The id of the directory tree of the associated tree object.
    id: git_hash::ObjectId,
    /// The amount of non-tree entries contained within, and definitely not zero.
    entry_count: u32,
}

/// A recursive data structure
pub fn decode(data: &[u8], object_hash: git_hash::Kind) -> Option<Tree> {
    let (tree, data) = one_recursive(data, object_hash.len_in_bytes())?;
    assert!(
        data.is_empty(),
        "BUG: should fully consume the entire tree extension chunk, got {} left",
        data.len()
    );
    Some(tree)
}

pub fn one_recursive(data: &[u8], hash_len: usize) -> Option<(Tree, &[u8])> {
    let (path, data) = split_at_byte_exclusive(data, 0)?;

    let (entry_count, data) = split_at_byte_exclusive(data, b' ')?;
    let entry_count: u32 = atoi::atoi(entry_count)?;

    let (subtree_count, mut data) = split_at_byte_exclusive(data, b'\n')?;
    let subtree_count: usize = atoi::atoi(subtree_count)?;

    let node_id = (entry_count != 0)
        .then(|| {
            (data.len() >= hash_len).then(|| {
                let (hash, rest) = data.split_at(hash_len);
                data = rest;
                ObjectId::from(hash)
            })
        })
        .flatten()
        .map(|id| NodeId { id, entry_count });

    let mut subtrees = Vec::with_capacity(subtree_count);
    for _ in 0..subtree_count {
        let (tree, rest) = one_recursive(data, hash_len)?;
        subtrees.push(tree);
        data = rest;
    }

    Some((
        Tree {
            id: node_id,
            name: path.into(),
            children: subtrees,
        },
        data,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_of_tree() {
        assert_eq!(std::mem::size_of::<Tree>(), 88);
    }
}
