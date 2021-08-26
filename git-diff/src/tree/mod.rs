use std::collections::VecDeque;

use git_hash::ObjectId;
use git_object::tree;

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
pub struct Changes<'a>(Option<tree::RefIter<'a>>);

impl<'a, T> From<T> for Changes<'a>
where
    T: Into<Option<tree::RefIter<'a>>>,
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

/// Useful for use as delegate implementing [`Visit`] to keep track of all seen changes. Useful for debugging or printing primarily.
pub mod recorder;
#[doc(inline)]
pub use recorder::Recorder;
