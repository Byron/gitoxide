use git_hash::bstr::BStr;
use git_object::immutable;

/// What to do after an entry was [recorded][Visit::visit()].
#[derive(Clone, Copy, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub enum Action {
    /// Continue the traversal of entries.
    Continue,
    /// Stop the traversal of entries, making this te last call to [visit(…)][Visit::visit()].
    Cancel,
    /// Don't dive into the entry, skipping children effectively.
    Skip,
}

impl Action {
    /// Returns true if this action means to stop the traversal.
    pub fn cancelled(&self) -> bool {
        matches!(self, Action::Cancel)
    }
}

/// A trait to allow responding to a traversal designed to observe all entries in a tree, recursively while keeping track of
/// paths if desired.
pub trait Visit {
    /// A type capable of uniquely identifying paths in a tree.
    type PathId: Clone + Default;

    /// Sets the path associated with the given `id` so future calls to push and pop components affect it instead.
    fn set_current_path(&mut self, id: Self::PathId);
    /// Append a `component` to the end of a path, which may be empty, and associate it with the returned path id.
    fn push_tracked_path_component(&mut self, component: &BStr) -> Self::PathId;
    /// Append a `component` to the end of a path, which may be empty.
    fn push_path_component(&mut self, component: &BStr);
    /// Removes the last component from the path, which may leave it empty.
    fn pop_path_component(&mut self);

    /// Observe a tree entry that is a tree and return an instruction whether to continue or not.
    /// [`Action::Skip`] can be used to prevent traversing it, for example if it's known to the caller already.
    ///
    /// The implementation may use the current path to learn where in the tree the change is located.
    fn visit_tree(&mut self, entry: &immutable::tree::Entry<'_>) -> Action;

    /// Observe a tree entry that is NO tree and return an instruction whether to continue or not.
    /// [`Action::Skip`] has no effect here.
    ///
    /// The implementation may use the current path to learn where in the tree the change is located.
    fn visit_nontree(&mut self, entry: &immutable::tree::Entry<'_>) -> Action;
}
