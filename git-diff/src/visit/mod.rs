use git_hash::ObjectId;
use git_object::immutable;

#[derive(Default, Clone)]
pub struct State {
    buf1: Vec<u8>,
    buf2: Vec<u8>,
    trees: Vec<(Option<TreeInfo>, Option<TreeInfo>)>,
}

#[derive(Clone)]
pub(crate) struct TreeInfo {
    pub tree_id: ObjectId,
    pub entries_level: usize,
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
