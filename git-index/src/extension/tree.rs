use bstr::ByteSlice;
use git_hash::ObjectId;
use std::cmp::Ordering;

use crate::util::split_at_pos;
use crate::{
    extension::{Signature, Tree},
    util::split_at_byte_exclusive,
};

pub const SIGNATURE: Signature = *b"TREE";

pub mod verify {
    use bstr::BString;
    use quick_error::quick_error;

    quick_error! {
        #[derive(Debug)]
        pub enum Error {
            MissingTreeDirectory { parent_id: git_hash::ObjectId, entry_id: git_hash::ObjectId, name: BString } {
                display("The entry {} at path '{}' in parent tree {} wasn't found in the nodes children, making it incomplete", entry_id, name, parent_id)
            }
            TreeNodeNotFound { oid: git_hash::ObjectId } {
                display("The tree with id {} wasn't found in the object database", oid)
            }
            TreeNodeChildcountMismatch { oid: git_hash::ObjectId, expected_childcount: usize, actual_childcount: usize } {
                display("The tree with id {} should have {} children, but its cached representation had {} of them", oid, expected_childcount, actual_childcount)
            }
            RootWithName { name: BString } {
                display("The root tree was named '{}', even though it should be empty", name)
            }
            EntriesCount {actual: u32, expected: u32 } {
                display("Expected not more than {} entries to be reachable from the top-level, but actual count was {}", expected, actual)
            }
            OutOfOrder { parent_id: git_hash::ObjectId, current_path: BString, previous_path: BString } {
                display("Parent tree '{}' contained out-of order trees prev = '{}' and next = '{}'", parent_id, previous_path, current_path)
            }
        }
    }
}

impl Tree {
    pub fn verify<F>(&self, use_find: bool, mut find: F) -> Result<(), verify::Error>
    where
        F: for<'a> FnMut(&git_hash::oid, &'a mut Vec<u8>) -> Option<git_object::TreeRefIter<'a>>,
    {
        fn verify_recursive<F>(
            parent_id: git_hash::ObjectId,
            children: &[Tree],
            mut find_buf: Option<&mut Vec<u8>>,
            find: &mut F,
        ) -> Result<Option<u32>, verify::Error>
        where
            F: for<'a> FnMut(&git_hash::oid, &'a mut Vec<u8>) -> Option<git_object::TreeRefIter<'a>>,
        {
            if children.is_empty() {
                return Ok(None);
            }
            let mut entries = 0;
            let mut prev = None::<&Tree>;
            for child in children {
                entries += child.num_entries;
                if let Some(prev) = prev {
                    if prev.name.cmp(&child.name) != Ordering::Less {
                        return Err(verify::Error::OutOfOrder {
                            parent_id,
                            previous_path: prev.name.as_bstr().into(),
                            current_path: child.name.as_bstr().into(),
                        });
                    }
                }
                prev = Some(child);
            }
            if let Some(buf) = find_buf.as_mut() {
                let tree_entries =
                    find(&parent_id, *buf).ok_or_else(|| verify::Error::TreeNodeNotFound { oid: parent_id })?;
                let mut num_entries = 0;
                for entry in tree_entries
                    .filter_map(Result::ok)
                    .filter(|e| e.mode == git_object::tree::EntryMode::Tree)
                {
                    children
                        .binary_search_by(|e| e.name.as_bstr().cmp(&entry.filename))
                        .map_err(|_| verify::Error::MissingTreeDirectory {
                            parent_id,
                            entry_id: entry.oid.to_owned(),
                            name: entry.filename.to_owned(),
                        })?;
                    num_entries += 1;
                }

                if num_entries != children.len() {
                    return Err(verify::Error::TreeNodeChildcountMismatch {
                        oid: parent_id,
                        expected_childcount: num_entries,
                        actual_childcount: children.len(),
                    });
                }
            }
            for child in children {
                let actual_num_entries = verify_recursive(child.id, &child.children, find_buf.as_deref_mut(), find)?;
                if let Some(actual) = actual_num_entries {
                    if actual > child.num_entries {
                        return Err(verify::Error::EntriesCount {
                            actual,
                            expected: child.num_entries,
                        });
                    }
                }
            }
            Ok(entries.into())
        }

        if !self.name.is_empty() {
            return Err(verify::Error::RootWithName {
                name: self.name.as_bstr().into(),
            });
        }

        let mut buf = Vec::new();
        let declared_entries = verify_recursive(self.id, &self.children, use_find.then(|| &mut buf), &mut find)?;
        if let Some(actual) = declared_entries {
            if actual > self.num_entries {
                return Err(verify::Error::EntriesCount {
                    actual,
                    expected: self.num_entries,
                });
            }
        }

        Ok(())
    }
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
    let num_entries: u32 = atoi::atoi(entry_count)?;

    let (subtree_count, data) = split_at_byte_exclusive(data, b'\n')?;
    let subtree_count: usize = atoi::atoi(subtree_count)?;

    let (hash, mut data) = split_at_pos(data, hash_len)?;
    let id = ObjectId::from(hash);

    let mut subtrees = Vec::with_capacity(subtree_count);
    for _ in 0..subtree_count {
        let (tree, rest) = one_recursive(data, hash_len)?;
        match subtrees.binary_search_by(|t: &Tree| t.name.cmp(&tree.name)) {
            Ok(_existing_index) => return None,
            Err(insert_position) => subtrees.insert(insert_position, tree),
        }
        data = rest;
    }

    Some((
        Tree {
            id,
            num_entries,
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
        assert_eq!(std::mem::size_of::<Tree>(), 80);
    }
}
