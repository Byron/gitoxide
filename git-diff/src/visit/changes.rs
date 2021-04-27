use crate::visit;
use git_hash::{oid, ObjectId};
use git_object::immutable;
use quick_error::quick_error;

const EMPTY_TREE: immutable::Tree<'static> = immutable::Tree::empty();

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        NotFound(oid: ObjectId) {
            display("The object {} referenced by the tree was not found in the database", oid)
        }
        Cancelled {
            display("The delegate cancelled the operation")
        }
    }
}

impl<'a> visit::Changes<'a> {
    /// Returns the changes that need to be applied to `self` to get `other`.
    pub fn to_obtain_tree<LocateFn>(
        &self,
        _other: &git_object::immutable::Tree<'_>,
        _state: &mut visit::State,
        _locate: LocateFn,
        _delegate: &mut impl visit::Record,
    ) -> Result<(), Error>
    where
        LocateFn: for<'b> FnMut(&oid, &'b mut Vec<u8>) -> Option<immutable::Object<'b>>,
    {
        let _this = *self.0.as_ref().unwrap_or(&&EMPTY_TREE);
        todo!("changes tree to tree")
    }
}
