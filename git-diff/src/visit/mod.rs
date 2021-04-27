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

impl<'a> From<Option<immutable::TreeIter<'a>>> for Changes<'a> {
    fn from(v: Option<immutable::TreeIter<'a>>) -> Self {
        Changes(v)
    }
}

mod changes;

pub mod record;
pub use record::Record;

pub mod recorder;
pub use recorder::Recorder;
