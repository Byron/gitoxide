use git_object::immutable;

#[derive(Default, Clone)]
pub struct State {
    buf1: Vec<u8>,
    buf2: Vec<u8>,
}

pub(crate) type TreeEntryResult<'a> = Result<immutable::tree::Entry<'a>, immutable::object::decode::Error>;

pub struct Changes<'a, Iter>(Iter)
where
    Iter: Iterator<Item = TreeEntryResult<'a>>;

impl<'a, Iter> From<Iter> for Changes<'a, Iter>
where
    Iter: Iterator<Item = TreeEntryResult<'a>>,
{
    fn from(v: Iter) -> Self {
        Changes(v)
    }
}

mod changes;

pub mod record;
pub use record::Record;

pub mod recorder;
pub use recorder::Recorder;
