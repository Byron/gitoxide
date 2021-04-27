use git_object::immutable;

#[derive(Default, Clone)]
pub struct State {
    buf1: Vec<u8>,
    buf2: Vec<u8>,
}

pub struct Changes<'a>(Option<&'a immutable::Tree<'a>>);

impl<'a, T> From<T> for Changes<'a>
where
    T: Into<Option<&'a immutable::Tree<'a>>>,
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
