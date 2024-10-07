pub use object_id::ObjectIdExt;
pub use reference::ReferenceExt;
#[cfg(feature = "revision")]
pub use rev_spec::RevSpecExt;
#[cfg(feature = "blob-diff")]
pub use tree::TreeDiffChangeExt;
pub use tree::{TreeEntryExt, TreeEntryRefExt, TreeIterExt};

mod object_id;
mod reference;
#[cfg(feature = "revision")]
mod rev_spec;
mod tree;
