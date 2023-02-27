use std::convert::TryInto;

use gix_hash::ObjectId;

use crate::{
    extension::Tree,
    util::{split_at_byte_exclusive, split_at_pos},
};

/// A recursive data structure
pub fn decode(data: &[u8], object_hash: gix_hash::Kind) -> Option<Tree> {
    let (tree, data) = one_recursive(data, object_hash.len_in_bytes())?;
    assert!(
        data.is_empty(),
        "BUG: should fully consume the entire tree extension chunk, got {} left",
        data.len()
    );
    Some(tree)
}

fn one_recursive(data: &[u8], hash_len: usize) -> Option<(Tree, &[u8])> {
    let (path, data) = split_at_byte_exclusive(data, 0)?;

    let (entry_count, data) = split_at_byte_exclusive(data, b' ')?;
    let num_entries: i32 = btoi::btoi(entry_count).ok()?;

    let (subtree_count, data) = split_at_byte_exclusive(data, b'\n')?;
    let subtree_count: usize = btoi::btou(subtree_count).ok()?;

    let (id, mut data) = if num_entries >= 0 {
        let (hash, data) = split_at_pos(data, hash_len)?;
        (ObjectId::from(hash), data)
    } else {
        (
            ObjectId::null(gix_hash::Kind::from_hex_len(hash_len * 2).expect("valid hex_len")),
            data,
        )
    };

    let mut subtrees = Vec::with_capacity(subtree_count);
    for _ in 0..subtree_count {
        let (tree, rest) = one_recursive(data, hash_len)?;
        subtrees.push(tree);
        data = rest;
    }

    subtrees.sort_by(|a, b| a.name.cmp(&b.name));
    let num_trees = subtrees.len();
    subtrees.dedup_by(|a, b| a.name == b.name);
    if num_trees != subtrees.len() {
        return None;
    }

    Some((
        Tree {
            id,
            num_entries: num_entries.try_into().ok(),
            name: path.into(),
            children: subtrees,
        },
        data,
    ))
}
