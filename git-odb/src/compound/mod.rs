use crate::{loose, pack};

pub struct Db {
    pub loose: loose::Db,
    pub packs: Vec<pack::Bundle>,
}

mod object {
    use crate::loose;

    #[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
    pub enum Object<'a> {
        Loose(loose::Object),
        Borrowed(crate::borrowed::Object<'a>),
    }
}
pub use object::Object;

mod init;
mod locate;
mod write;
