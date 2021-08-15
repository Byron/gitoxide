use bstr::BString;
use git_hash::ObjectId;

use crate::{
    store::{file, file::Transaction},
    transaction::RefEdit,
    Namespace,
};

/// A function receiving an object id to resolve, returning its decompressed bytes.
///
/// Resolution means to follow tag objects until the end of the chain.
pub type FindObjectFn =
    dyn FnMut(
        git_hash::ObjectId,
        &mut Vec<u8>,
    ) -> Result<Option<git_object::Kind>, Box<dyn std::error::Error + Send + Sync + 'static>>;

/// How to handle packed refs during a transaction
pub enum PackedRefs {
    /// Only propagate deletions of references. This is the default
    DeletionsOnly,
    /// Propagate deletions as well as updates to references which are peeled, that is contain an object id
    DeletionsAndNonSymbolicUpdates(Box<FindObjectFn>),
    /// Propagate deletions as well as updates to references which are peeled, that is contain an object id. Furthermore delete the
    /// reference which is originally updated if it exists. If it doesn't, the new value will be written into the packed ref right away.
    DeletionsAndNonSymbolicUpdatesRemoveLooseSourceReference(Box<FindObjectFn>),
}

impl Default for PackedRefs {
    fn default() -> Self {
        PackedRefs::DeletionsOnly
    }
}

#[derive(Debug)]
pub(in crate::store::file) struct Edit {
    update: RefEdit,
    lock: Option<git_lock::Marker>,
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
    pub fn transaction(&self) -> Transaction<'_> {
        Transaction {
            store: self,
            packed_transaction: None,
            updates: None,
            packed_refs: PackedRefs::default(),
            namespace: None,
        }
    }
}

impl<'s> Transaction<'s> {
    /// Configure the way packed refs are handled during the transaction
    pub fn packed_refs(mut self, packed_refs: PackedRefs) -> Self {
        self.packed_refs = packed_refs;
        self
    }

    /// Configure the namespace within which all edits should take place.
    /// For example, with namespace `foo`, edits destined for `HEAD` will affect `refs/namespaces/foo/HEAD` instead.
    /// Set `None` to not use any namespace, which also is the default.
    ///
    /// This also means that edits returned when [`commit(…)`ing](Transaction::commit()) will have their name altered to include
    /// the namespace automatically, so it must be stripped when returning them to the user to keep them 'invisible'.
    pub fn namespace(mut self, namespace: impl Into<Option<Namespace>>) -> Self {
        self.namespace = namespace.into();
        self
    }
}

///
pub mod prepare;

///
pub mod commit;
