mod commit;
pub use commit::Commit;

mod tag;
pub use tag::Tag;

mod tree;
//FIXME: keep tree mode and entry in tree export it from there? Alternatively rename to TreeMode, TreeEntry?
pub use tree::{Entry, Mode, Tree};

mod object;
pub use object::*;

mod util;
