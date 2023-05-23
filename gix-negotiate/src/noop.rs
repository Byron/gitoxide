use crate::{Error, Negotiator};
use gix_hash::ObjectId;

pub(crate) struct Noop;

impl Negotiator for Noop {
    fn known_common(&mut self, _id: ObjectId) -> Result<(), Error> {
        Ok(())
    }

    fn add_tip(&mut self, _id: ObjectId) -> Result<(), Error> {
        Ok(())
    }

    fn next_have(&mut self) -> Option<Result<ObjectId, Error>> {
        None
    }

    fn in_common_with_remote(&mut self, _id: ObjectId) -> Result<bool, Error> {
        Ok(false)
    }
}
