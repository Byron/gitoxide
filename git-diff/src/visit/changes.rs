use crate::visit;
use crate::visit::record::{Change, PathComponent, PathComponentUpdateMode};
use git_hash::{oid, ObjectId};
use git_object::immutable;
use quick_error::quick_error;

static EMPTY_TREE: immutable::Tree<'static> = immutable::Tree::empty();

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
        other: &git_object::immutable::Tree<'_>,
        _state: &mut visit::State,
        _locate: LocateFn,
        delegate: &mut impl visit::Record,
    ) -> Result<(), Error>
    where
        LocateFn: for<'b> FnMut(&oid, &'b mut Vec<u8>) -> Option<immutable::Object<'b>>,
    {
        let lhs = *self.0.as_ref().unwrap_or(&&EMPTY_TREE);
        let rhs = other;

        let mut path_id = 0;
        let mut lhs_entries = lhs.entries.iter();
        let mut rhs_entries = rhs.entries.iter();

        loop {
            match (lhs_entries.next(), rhs_entries.next()) {
                (None, None) => break Ok(()),
                (Some(lhs), None) => {
                    delegate.update_path_component(
                        PathComponent::new(lhs.filename, &mut path_id),
                        PathComponentUpdateMode::Replace,
                    );
                    if delegate
                        .record(Change::Deletion {
                            previous_entry_mode: lhs.mode,
                            previous_oid: lhs.oid.to_owned(),
                            path_id: path_id,
                        })
                        .cancelled()
                    {
                        break Err(Error::Cancelled);
                    }
                }
                (None, Some(rhs)) => {
                    delegate.update_path_component(
                        PathComponent::new(rhs.filename, &mut path_id),
                        PathComponentUpdateMode::Replace,
                    );
                    if delegate
                        .record(Change::Addition {
                            entry_mode: rhs.mode,
                            oid: rhs.oid.to_owned(),
                            path_id: path_id,
                        })
                        .cancelled()
                    {
                        break Err(Error::Cancelled);
                    }
                }
                _ => todo!("all branches"),
            }
        }
    }
}
