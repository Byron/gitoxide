use std::fmt::Formatter;

use gix_hash::ObjectId;
use gix_object::bstr::BString;

use crate::{
    store_impl::{file, file::Transaction},
    transaction::RefEdit,
};

/// A function receiving an object id to resolve, returning its decompressed bytes,
/// used to obtain the peeled object ids for storage in packed-refs files.
///
/// Resolution means to follow tag objects until the end of the chain.
pub type FindObjectFn<'a> = dyn FnMut(
        gix_hash::ObjectId,
        &mut Vec<u8>,
    ) -> Result<Option<gix_object::Kind>, Box<dyn std::error::Error + Send + Sync + 'static>>
    + 'a;

/// How to handle packed refs during a transaction
#[derive(Default)]
pub enum PackedRefs<'a> {
    /// Only propagate deletions of references. This is the default
    #[default]
    DeletionsOnly,
    /// Propagate deletions as well as updates to references which are peeled, that is contain an object id
    DeletionsAndNonSymbolicUpdates(Box<FindObjectFn<'a>>),
    /// Propagate deletions as well as updates to references which are peeled, that is contain an object id. Furthermore delete the
    /// reference which is originally updated if it exists. If it doesn't, the new value will be written into the packed ref right away.
    /// Note that this doesn't affect symbolic references at all, which can't be placed into packed refs.
    DeletionsAndNonSymbolicUpdatesRemoveLooseSourceReference(Box<FindObjectFn<'a>>),
}

#[derive(Debug)]
pub(in crate::store_impl::file) struct Edit {
    update: RefEdit,
    lock: Option<gix_lock::Marker>,
    /// Set if this update is coming from a symbolic reference and used to make it appear like it is the one that is handled,
    /// instead of the referent reference.
    parent_index: Option<usize>,
    /// For symbolic refs, this is the previous OID to put into the reflog instead of our own previous value. It's the
    /// peeled value of the leaf referent.
    leaf_referent_previous_oid: Option<ObjectId>,
}

impl Edit {
    fn name(&self) -> BString {
        self.update.name.0.clone()
    }
}

impl std::borrow::Borrow<RefEdit> for Edit {
    fn borrow(&self) -> &RefEdit {
        &self.update
    }
}

impl std::borrow::BorrowMut<RefEdit> for Edit {
    fn borrow_mut(&mut self) -> &mut RefEdit {
        &mut self.update
    }
}

/// Edits
impl file::Store {
    /// Open a transaction with the given `edits`, and determine how to fail if a `lock` cannot be obtained.
    /// A snapshot of packed references will be obtained automatically if needed to fulfill this transaction
    /// and will be provided as result of a successful transaction. Note that upon transaction failure, packed-refs
    /// will never have been altered.
    ///
    /// The transaction inherits the parent namespace.
    pub fn transaction(&self) -> Transaction<'_, '_> {
        Transaction {
            store: self,
            packed_transaction: None,
            updates: None,
            packed_refs: PackedRefs::default(),
        }
    }
}

impl<'s, 'p> Transaction<'s, 'p> {
    /// Configure the way packed refs are handled during the transaction
    pub fn packed_refs(mut self, packed_refs: PackedRefs<'p>) -> Self {
        self.packed_refs = packed_refs;
        self
    }
}

impl std::fmt::Debug for Transaction<'_, '_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Transaction")
            .field("store", self.store)
            .field("edits", &self.updates.as_ref().map(Vec::len))
            .finish_non_exhaustive()
    }
}

///
pub mod prepare;

///
pub mod commit;
