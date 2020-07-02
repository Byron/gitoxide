mod commit;
pub use commit::Commit;

mod tag;
pub use tag::Tag;

mod tree;
//FIXME: keep tree mode and entry in tree export it from there? Alternatively rename to TreeMode, TreeEntry?
pub use tree::{Entry as TreeEntry, Mode as TreeMode, Tree};

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Blob<'data> {
    #[cfg_attr(feature = "serde1", serde(borrow))]
    pub data: &'data [u8],
}

mod object;
pub use object::*;

mod util;
