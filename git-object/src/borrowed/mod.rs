mod commit;
pub use commit::Commit;

mod tag;
pub use tag::Tag;

pub mod tree;
pub use tree::Tree;

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Blob<'data> {
    pub data: &'data [u8],
}

mod object;
pub use object::*;

mod util;
