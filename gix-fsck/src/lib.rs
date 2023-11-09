//! A library for performing object database integrity and connectivity checks
#![deny(rust_2018_idioms, unsafe_code, missing_docs)]

use gix_hash::ObjectId;
use gix_hashtable::HashSet;
use gix_object::{tree::EntryMode, Exists, FindExt, Kind};
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};

/// Perform a connectivity check.
pub struct Connectivity<T, F>
where
    T: FindExt + Exists,
    F: FnMut(&ObjectId, Kind),
{
    /// ODB handle to use for the check
    db: T,
    /// Closure to invoke when a missing object is encountered
    missing_cb: F,
    /// Set of Object IDs already (or about to be) scanned during the check
    oid_set: HashSet,
    /// A free-list of buffers for recursive tree decoding.
    free_list: FreeList,
}

impl<T, F> Connectivity<T, F>
where
    T: FindExt + Exists,
    F: FnMut(&ObjectId, Kind),
{
    /// Instantiate a connectivity check.
    pub fn new(db: T, missing_cb: F) -> Connectivity<T, F> {
        Connectivity {
            db,
            missing_cb,
            oid_set: HashSet::default(),
            free_list: Default::default(),
        }
    }

    /// Run the connectivity check on the provided commit `oid`.
    ///
    /// ### Algorithm
    ///
    /// Walk the trees and blobs referenced by the commit and verify they exist in the ODB.
    /// Any objects previously encountered by this instance will be skipped silently.
    /// Any referenced blobs that are not present in the ODB will result in a call to the  `missing_cb`.
    /// Missing commits or trees will cause an error to be returned.
    ///     - TODO: consider how to handle a missing commit (invoke `missing_cb`, or possibly return a Result?)
    pub fn check_commit(&mut self, oid: &ObjectId) -> Result<(), gix_object::find::existing_object::Error> {
        // Attempt to insert the commit ID in the set, and if already present, return immediately
        if !self.oid_set.insert(*oid) {
            return Ok(());
        }
        // Obtain the commit's tree ID
        let tree_id = {
            let mut buf = self.free_list.buf();
            let commit = self.db.find_commit(oid, &mut buf)?;
            commit.tree()
        };

        if self.oid_set.insert(tree_id) {
            check_tree(
                &tree_id,
                &self.db,
                &mut self.free_list,
                &mut self.missing_cb,
                &mut self.oid_set,
            );
        }

        Ok(())
    }
}

#[derive(Default)]
struct FreeList(RefCell<Vec<Vec<u8>>>);

impl FreeList {
    fn buf(&self) -> ReturnToFreeListOnDrop<'_> {
        let buf = self.0.borrow_mut().pop().unwrap_or_default();
        ReturnToFreeListOnDrop { buf, list: &self.0 }
    }
}

struct ReturnToFreeListOnDrop<'a> {
    list: &'a RefCell<Vec<Vec<u8>>>,
    buf: Vec<u8>,
}

impl Drop for ReturnToFreeListOnDrop<'_> {
    fn drop(&mut self) {
        if !self.buf.is_empty() {
            self.list.borrow_mut().push(std::mem::take(&mut self.buf));
        }
    }
}

impl Deref for ReturnToFreeListOnDrop<'_> {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.buf
    }
}

impl DerefMut for ReturnToFreeListOnDrop<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.buf
    }
}

fn check_blob<F>(db: impl Exists, oid: &ObjectId, mut missing_cb: F)
where
    F: FnMut(&ObjectId, Kind),
{
    // Check if the blob is missing from the ODB
    if !db.exists(oid) {
        // Blob is missing, so invoke `missing_cb`
        missing_cb(oid, Kind::Blob);
    }
}

fn check_tree<F>(
    oid: &ObjectId,
    db: &(impl FindExt + Exists),
    list: &FreeList,
    missing_cb: &mut F,
    oid_set: &mut HashSet,
) where
    F: FnMut(&ObjectId, Kind),
{
    let mut buf = list.buf();
    let Ok(tree) = db.find_tree(oid, &mut buf) else {
        missing_cb(oid, Kind::Tree);
        return;
    };

    // Build up a set of trees and a set of blobs
    // For each entry in the tree
    for entry_ref in tree.entries.iter() {
        match entry_ref.mode {
            EntryMode::Tree => {
                let tree_id = entry_ref.oid.to_owned();
                if oid_set.insert(tree_id) {
                    check_tree(&tree_id, &*db, list, &mut *missing_cb, oid_set);
                }
            }
            EntryMode::Blob | EntryMode::BlobExecutable | EntryMode::Link => {
                let blob_id = entry_ref.oid.to_owned();
                if oid_set.insert(blob_id) {
                    check_blob(&*db, &blob_id, &mut *missing_cb);
                }
            }
            EntryMode::Commit => {
                // This implies a submodule (OID is the commit hash of the submodule)
                // Skip it as it's not in this repository!
            }
        }
    }
}
