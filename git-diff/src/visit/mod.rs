use git_hash::ObjectId;
use git_object::immutable;
use std::collections::VecDeque;

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
    pub fn clear(&mut self) {
        self.trees.clear();
        self.buf1.clear();
        self.buf2.clear();
    }
}

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

pub mod record;
pub use record::Record;

pub mod recorder;
pub use recorder::Recorder;
