use std::collections::VecDeque;

use gix_hash::ObjectId;
use gix_object::{bstr::BString, TreeRefIter};

/// The state required to visit [Changes] to be instantiated with `State::default()`.
#[derive(Default, Clone)]
pub struct State {
    buf1: Vec<u8>,
    buf2: Vec<u8>,
    trees: VecDeque<TreeInfoPair>,
}

type TreeInfoPair = (Option<ObjectId>, Option<ObjectId>);

impl State {
    fn clear(&mut self) {
        self.trees.clear();
        self.buf1.clear();
        self.buf2.clear();
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
#[doc(inline)]
pub use visit::Visit;

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
