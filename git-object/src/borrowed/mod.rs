mod commit;
pub use commit::Commit;

mod tag;
pub use tag::Tag;

mod tree;
//FIXME: keep tree mode and entry in tree export it from there? Alternatively rename to TreeMode, TreeEntry?
pub use tree::{Entry as TreeEntry, Mode as TreeMode, Tree};

#[derive(PartialEq, Eq, Debug, Hash)]
pub struct Blob<'data>(pub &'data [u8]);

mod object;
pub use object::*;

mod util;
