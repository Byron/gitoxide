use std::cmp::Ordering;

use bstr::{BString, ByteSlice};

use crate::extension::Tree;

/// The error returned by [`Tree::verify()`][crate::extension::Tree::verify()].
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error("The entry {entry_id} at path '{name}' in parent tree {parent_id} wasn't found in the nodes children, making it incomplete")]
    MissingTreeDirectory {
        parent_id: gix_hash::ObjectId,
        entry_id: gix_hash::ObjectId,
        name: BString,
    },
    #[error("The tree with id {oid} wasn't found in the object database")]
    TreeNodeNotFound { oid: gix_hash::ObjectId },
    #[error("The tree with id {oid} should have {expected_childcount} children, but its cached representation had {actual_childcount} of them")]
    TreeNodeChildcountMismatch {
        oid: gix_hash::ObjectId,
        expected_childcount: usize,
        actual_childcount: usize,
    },
    #[error("The root tree was named '{name}', even though it should be empty")]
    RootWithName { name: BString },
    #[error(
        "Expected not more than {expected} entries to be reachable from the top-level, but actual count was {actual}"
    )]
    EntriesCount { actual: u32, expected: u32 },
    #[error(
        "Parent tree '{parent_id}' contained out-of order trees prev = '{previous_path}' and next = '{current_path}'"
    )]
    OutOfOrder {
        parent_id: gix_hash::ObjectId,
        current_path: BString,
        previous_path: BString,
    },
}

impl Tree {
    ///
    pub fn verify<F>(&self, use_find: bool, mut find: F) -> Result<(), Error>
    where
        F: for<'a> FnMut(&gix_hash::oid, &'a mut Vec<u8>) -> Option<gix_object::TreeRefIter<'a>>,
    {
        fn verify_recursive<F>(
            parent_id: gix_hash::ObjectId,
            children: &[Tree],
            mut find_buf: Option<&mut Vec<u8>>,
            find: &mut F,
        ) -> Result<Option<u32>, Error>
        where
            F: for<'a> FnMut(&gix_hash::oid, &'a mut Vec<u8>) -> Option<gix_object::TreeRefIter<'a>>,
        {
            if children.is_empty() {
                return Ok(None);
            }
            let mut entries = 0;
            let mut prev = None::<&Tree>;
            for child in children {
                entries += child.num_entries.unwrap_or(0);
                if let Some(prev) = prev {
                    if prev.name.cmp(&child.name) != Ordering::Less {
                        return Err(Error::OutOfOrder {
                            parent_id,
                            previous_path: prev.name.as_bstr().into(),
                            current_path: child.name.as_bstr().into(),
                        });
                    }
                }
                prev = Some(child);
            }
            if let Some(buf) = find_buf.as_mut() {
                let tree_entries = find(&parent_id, buf).ok_or(Error::TreeNodeNotFound { oid: parent_id })?;
                let mut num_entries = 0;
                for entry in tree_entries
                    .filter_map(Result::ok)
                    .filter(|e| e.mode == gix_object::tree::EntryMode::Tree)
                {
                    children
                        .binary_search_by(|e| e.name.as_bstr().cmp(entry.filename))
                        .map_err(|_| Error::MissingTreeDirectory {
                            parent_id,
                            entry_id: entry.oid.to_owned(),
                            name: entry.filename.to_owned(),
                        })?;
                    num_entries += 1;
                }

                if num_entries != children.len() {
                    return Err(Error::TreeNodeChildcountMismatch {
                        oid: parent_id,
                        expected_childcount: num_entries,
                        actual_childcount: children.len(),
                    });
                }
            }
            for child in children {
                // This is actually needed here as it's a mut ref, which isn't copy. We do a re-borrow here.
                #[allow(clippy::needless_option_as_deref)]
                let actual_num_entries = verify_recursive(child.id, &child.children, find_buf.as_deref_mut(), find)?;
                if let Some((actual, num_entries)) = actual_num_entries.zip(child.num_entries) {
                    if actual > num_entries {
                        return Err(Error::EntriesCount {
                            actual,
                            expected: num_entries,
                        });
                    }
                }
            }
            Ok(entries.into())
        }
        let _span = gix_features::trace::coarse!("gix_index::extension::Tree::verify()");

        if !self.name.is_empty() {
            return Err(Error::RootWithName {
                name: self.name.as_bstr().into(),
            });
        }

        let mut buf = Vec::new();
        let declared_entries = verify_recursive(self.id, &self.children, use_find.then_some(&mut buf), &mut find)?;
        if let Some((actual, num_entries)) = declared_entries.zip(self.num_entries) {
            if actual > num_entries {
                return Err(Error::EntriesCount {
                    actual,
                    expected: num_entries,
                });
            }
        }

        Ok(())
    }
}
