use gix_hash::ObjectId;

use crate::{Error, Negotiator};

pub(crate) struct Noop;

impl Negotiator for Noop {
    fn known_common(&mut self, _id: ObjectId, _graph: &mut crate::Graph<'_, '_>) -> Result<(), Error> {
        Ok(())
    }

    fn add_tip(&mut self, _id: ObjectId, _graph: &mut crate::Graph<'_, '_>) -> Result<(), Error> {
        Ok(())
    }

    fn next_have(&mut self, _graph: &mut crate::Graph<'_, '_>) -> Option<Result<ObjectId, Error>> {
        None
    }

    fn in_common_with_remote(&mut self, _id: ObjectId, _graph: &mut crate::Graph<'_, '_>) -> Result<bool, Error> {
        Ok(false)
    }
}
