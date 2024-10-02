use crate::tree::visit::Relation;
use bstr::BStr;
use gix_hash::ObjectId;
use gix_object::{bstr::BString, TreeRefIter};
use std::collections::VecDeque;

/// A trait to allow responding to a traversal designed to figure out the [changes](visit::Change)
/// to turn tree A into tree B.
pub trait Visit {
    /// Sets the full path in front of the queue so future calls to push and pop components affect it instead.
    fn pop_front_tracked_path_and_set_current(&mut self);
    /// Append a `component` to the end of a path, which may be empty.
    fn push_back_tracked_path_component(&mut self, component: &BStr);
    /// Append a `component` to the end of a path, which may be empty.
    fn push_path_component(&mut self, component: &BStr);
    /// Removes the last component from the path, which may leave it empty.
    fn pop_path_component(&mut self);
    /// Record a `change` and return an instruction whether to continue or not.
    ///
    /// The implementation may use the current path to lean where in the tree the change is located.
    fn visit(&mut self, change: visit::Change) -> visit::Action;
}

/// The state required to visit [Changes] to be instantiated with `State::default()`.
#[derive(Default, Clone)]
pub struct State {
    buf1: Vec<u8>,
    buf2: Vec<u8>,
    trees: VecDeque<TreeInfoTuple>,
    change_id: visit::ChangeId,
}

type TreeInfoTuple = (Option<ObjectId>, Option<ObjectId>, Option<Relation>);

impl State {
    fn clear(&mut self) {
        self.trees.clear();
        self.buf1.clear();
        self.buf2.clear();
        self.change_id = 0;
    }
}

/// An iterator over changes of a tree, instantiated using `Changes::from(…)`.
pub struct Changes<'a>(Option<TreeRefIter<'a>>);

impl<'a, T> From<T> for Changes<'a>
where
    T: Into<Option<TreeRefIter<'a>>>,
{
    fn from(v: T) -> Self {
        Changes(v.into())
    }
}

///
pub mod changes;

///
pub mod visit;

/// A [Visit] implementation to record every observed change and keep track of the changed paths.
#[derive(Clone, Debug)]
pub struct Recorder {
    path_deque: VecDeque<BString>,
    path: BString,
    location: Option<recorder::Location>,
    /// The observed changes.
    pub records: Vec<recorder::Change>,
}

/// Useful for use as delegate implementing [`Visit`] to keep track of all seen changes. Useful for debugging or printing primarily.
pub mod recorder;
