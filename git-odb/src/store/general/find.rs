use std::ops::Deref;

use git_hash::oid;
use git_object::Data;
use git_pack::{cache::DecodeEntry, data::entry::Location, index::Entry};

impl<S> crate::pack::Find for super::Handle<S>
where
    S: Deref<Target = super::Store> + Clone,
{
    type Error = crate::compound::find::Error;

    fn contains(&self, id: impl AsRef<oid>) -> bool {
        todo!("contains")
    }

    fn try_find_cached<'a>(
        &self,
        id: impl AsRef<oid>,
        buffer: &'a mut Vec<u8>,
        pack_cache: &mut impl DecodeEntry,
    ) -> Result<Option<(Data<'a>, Option<Location>)>, Self::Error> {
        todo!("try find cached")
    }

    fn location_by_oid(&self, id: impl AsRef<oid>, buf: &mut Vec<u8>) -> Option<Location> {
        todo!("location by oid")
    }

    fn index_iter_by_pack_id(&self, pack_id: u32) -> Option<Box<dyn Iterator<Item = Entry> + '_>> {
        todo!("index iter by pack id")
    }

    fn entry_by_location(&self, location: &Location) -> Option<git_pack::find::Entry<'_>> {
        todo!("entry by location")
    }
}
