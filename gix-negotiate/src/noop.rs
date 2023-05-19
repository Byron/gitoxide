use crate::Negotiator;
use gix_hash::{oid, ObjectId};

pub(crate) struct Noop;

impl Negotiator for Noop {
    fn known_common(&mut self, _id: &oid) {}

    fn add_tip(&mut self, _id: &oid) {}

    fn next_have(&mut self) -> Option<ObjectId> {
        None
    }

    fn in_common_with_remote(&mut self, _id: &oid) -> bool {
        false
    }
}
