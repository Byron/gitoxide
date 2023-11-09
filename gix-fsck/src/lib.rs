//! A library for performing object database integrity and connectivity checks
#![deny(rust_2018_idioms)]

use gix_hash::ObjectId;
use gix_hashtable::HashSet;
use gix_object::{tree::EntryMode, Exists, FindExt, Kind};

pub struct ConnectivityCheck<'a, T, F>
where
    T: FindExt + Exists,
    F: FnMut(&ObjectId, Kind),
{
    /// ODB handle to use for the check
    db: &'a T,
    /// Closure to invoke when a missing object is encountered
    missing_cb: F,
    /// Set of Object IDs already (or about to be) scanned during the check
    oid_set: HashSet,
    /// Single buffer for decoding objects from the ODB
    /// This is slightly faster than allocating throughout the connectivity check (and reduces the memory requirements)
    buf: Vec<u8>,
}

impl<'a, T, F> ConnectivityCheck<'a, T, F>
where
    T: FindExt + Exists,
    F: FnMut(&ObjectId, Kind),
{
    /// Instantiate a connectivity check
    pub fn new(db: &'a T, missing_cb: F) -> ConnectivityCheck<'a, T, F> {
        ConnectivityCheck {
            db,
            missing_cb,
            oid_set: HashSet::default(),
            buf: Vec::new(),
        }
    }

    /// Run the connectivity check on the provided commit object ID
    /// - This will walk the trees and blobs referenced by the commit and verify they exist in the ODB
    /// - Any objects previously encountered by this [`ConnectivityCheck`] instance will be skipped
    /// - Any referenced blobs that are not present in the ODB will result in a call to the  `missing_cb`
    /// - Missing commits or trees will currently result in panic
    ///     - TODO: consider how to handle a missing commit (invoke `missing_cb`, or possibly return a Result?)
    pub fn check_commit(&mut self, oid: &ObjectId) {
        // Attempt to insert the commit ID in the set, and if already present, return immediately
        if !self.oid_set.insert(*oid) {
            return;
        }
        // Obtain the commit's tree ID
        let tree_id = {
            let commit = self.db.find_commit(oid, &mut self.buf).expect("failed to find commit");
            commit.tree()
        };

        // Attempt to insert the tree ID in the set, and if already present, return immediately
        if self.oid_set.insert(tree_id) {
            self.check_tree(&tree_id);
        }
    }

    fn check_tree(&mut self, oid: &ObjectId) {
        let tree = match self.db.find_tree(oid, &mut self.buf) {
            Ok(tree) => tree,
            Err(_) => {
                // Tree is missing, so invoke `missing_cb`
                (self.missing_cb)(oid, Kind::Tree);
                return;
            }
        };

        // Keeping separate sets for trees and blobs for now...
        // This is about a wash when compared to using a HashMap<ObjectID, Kind>
        struct TreeEntries {
            trees: HashSet<ObjectId>,
            blobs: HashSet<ObjectId>,
        }

        // Build up a set of trees and a set of blobs
        let entries: TreeEntries = {
            let mut entries = TreeEntries {
                trees: HashSet::default(),
                blobs: HashSet::default(),
            };

            // For each entry in the tree
            for entry_ref in tree.entries.iter() {
                match entry_ref.mode {
                    EntryMode::Tree => {
                        let tree_id = entry_ref.oid.to_owned();
                        // Check if the tree has already been encountered
                        if self.oid_set.insert(tree_id) {
                            entries.trees.insert(tree_id);
                        }
                    }
                    EntryMode::Blob | EntryMode::BlobExecutable | EntryMode::Link => {
                        let blob_id = entry_ref.oid.to_owned();
                        // Check if the blob has already been encountered
                        if self.oid_set.insert(blob_id) {
                            entries.blobs.insert(blob_id);
                        }
                    }
                    EntryMode::Commit => {
                        // This implies a submodule (OID is the commit hash of the submodule)
                        // Skip it as it's not in this repository!
                    }
                }
            }
            entries
        };

        for tree_id in entries.trees.iter() {
            self.check_tree(tree_id);
        }
        for blob_id in entries.blobs.iter() {
            self.check_blob(blob_id);
        }
    }

    fn check_blob(&mut self, oid: &ObjectId) {
        // Check if the blob is missing from the ODB
        if !self.db.exists(oid) {
            // Blob is missing, so invoke `missing_cb`
            (self.missing_cb)(oid, Kind::Blob);
        }
    }
}
