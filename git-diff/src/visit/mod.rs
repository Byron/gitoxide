use git_object::immutable;

#[derive(Default, Clone)]
pub struct State {
    buf1: Vec<u8>,
    buf2: Vec<u8>,
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
