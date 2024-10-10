/// The error returned by [tree()](crate::tree()).
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {}

/// The outcome produced by [tree()](crate::tree()).
pub struct Outcome<'a> {
    /// The ready-made (but unwritten) tree if `conflicts` is empty, or the best-possible tree when facing `conflicts`.
    ///
    /// The tree may contain blobs with conflict markers, and will be missing directories or files that were conflicting
    /// without a resolution strategy.
    tree: gix_object::tree::Editor<'a>,
    /// The set of conflicts we encountered. Can be empty to indicate there was no conflict.
    conflicts: Vec<Conflict>,
}

/// A description of a conflict (i.e. merge issue without an auto-resolution) as seen during a [tree-merge](crate::tree()).
pub struct Conflict;

pub(super) mod function {
    use crate::tree::{Error, Outcome};

    /// Perform a merge between `our_tree` and `their_tree`, using `base_trees` as merge-base.
    /// Note that if `base_trees` is empty, an empty tree is assumed to be the merge base.
    /// If there are more than one tree `base_trees`, it will merge them into one with the specialty that binary
    /// files will always be `our` side without conflicting. However, any other conflict will be fatal.
    ///
    /// `objects` provides access to trees when diffing them.
    pub fn tree<'a>(
        base_trees: &[gix_object::Object],
        our_tree: &gix_hash::oid,
        their_tree: &gix_hash::oid,
        objects: &'a dyn gix_object::FindExt,
    ) -> Result<Outcome<'a>, Error> {
        todo!()
    }
}
