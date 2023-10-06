use std::collections::VecDeque;

use gix_object::bstr::{BStr, BString};

/// A trait to allow responding to a traversal designed to observe all entries in a tree, recursively while keeping track of
/// paths if desired.
pub trait Visit {
    /// Sets the full path path in front of the queue so future calls to push and pop components affect it instead.
    fn pop_front_tracked_path_and_set_current(&mut self);
    /// Append a `component` to the end of a path, which may be empty.
    fn push_back_tracked_path_component(&mut self, component: &BStr);
    /// Append a `component` to the end of a path, which may be empty.
    fn push_path_component(&mut self, component: &BStr);
    /// Removes the last component from the path, which may leave it empty.
    fn pop_path_component(&mut self);

    /// Observe a tree entry that is a tree and return an instruction whether to continue or not.
    /// [`Action::Skip`][visit::Action::Skip] can be used to prevent traversing it, for example if it's known to the caller already.
    ///
    /// The implementation may use the current path to learn where in the tree the change is located.
    fn visit_tree(&mut self, entry: &gix_object::tree::EntryRef<'_>) -> visit::Action;

    /// Observe a tree entry that is NO tree and return an instruction whether to continue or not.
    /// [`Action::Skip`][visit::Action::Skip] has no effect here.
    ///
    /// The implementation may use the current path to learn where in the tree the change is located.
    fn visit_nontree(&mut self, entry: &gix_object::tree::EntryRef<'_>) -> visit::Action;
}

/// A [Visit] implementation to record every observed change and keep track of the changed paths.
///
/// Recorders can also be instructed to track the filename only, or no location at all.
#[derive(Clone, Debug)]
pub struct Recorder {
    path_deque: VecDeque<BString>,
    path: BString,
    /// How to track the location.
    location: Option<recorder::Location>,
    /// The observed entries.
    pub records: Vec<recorder::Entry>,
}

///
pub mod visit {
    /// What to do after an entry was [recorded][super::Visit::visit_tree()].
    #[derive(Clone, Copy, PartialOrd, PartialEq, Ord, Eq, Hash)]
    pub enum Action {
        /// Continue the traversal of entries.
        Continue,
        /// Stop the traversal of entries, making this te last call to [`visit_(tree|nontree)(…)`][super::Visit::visit_nontree()].
        Cancel,
        /// Don't dive into the entry, skipping children effectively. Only useful in [`visit_tree(…)`][super::Visit::visit_tree()].
        Skip,
    }

    impl Action {
        /// Returns true if this action means to stop the traversal.
        pub fn cancelled(&self) -> bool {
            matches!(self, Action::Cancel)
        }
    }
}

///
pub mod recorder;

///
pub mod breadthfirst;
pub use breadthfirst::impl_::traverse as breadthfirst;
