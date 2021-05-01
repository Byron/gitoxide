use git_hash::ObjectId;
use git_object::immutable;
use std::collections::VecDeque;

/// The state required to visit [Changes] to be instantiated with `State::default()`.
#[derive(Default, Clone)]
pub struct State<PathId: Default + Clone> {
    buf1: Vec<u8>,
    buf2: Vec<u8>,
    trees: VecDeque<TreeInfoPair<PathId>>,
}

type TreeInfoPair<PathId> = (Option<TreeInfo<PathId>>, Option<TreeInfo<PathId>>);

#[derive(Clone)]
pub(crate) struct TreeInfo<PathId: Clone> {
    pub tree_id: ObjectId,
    pub parent_path_id: PathId,
}

impl<P: Clone + Default> State<P> {
    fn clear(&mut self) {
        self.trees.clear();
        self.buf1.clear();
        self.buf2.clear();
    }
}

/// An iterator over changes of a tree, instantiated using `Changes::from(…)`.
pub struct Changes<'a>(Option<immutable::TreeIter<'a>>);

impl<'a, T> From<T> for Changes<'a>
where
    T: Into<Option<immutable::TreeIter<'a>>>,
{
    fn from(v: T) -> Self {
        Changes(v.into())
    }
}

mod changes;

///
pub mod visit;
pub use visit::Visit;

/// Houses a [`visit::Visit`] to keep track of all seen changes. Useful for debugging primarily.
pub mod recorder;
pub use recorder::Recorder;
