use std::ops::Deref;

use crate::general::load_index;
use git_hash::oid;
use git_object::Data;
use git_pack::{cache::DecodeEntry, data::entry::Location, index::Entry};

impl<S> crate::pack::Find for super::Handle<S>
where
    S: Deref<Target = super::Store> + Clone,
{
    type Error = crate::compound::find::Error;

    // TODO: probably make this method fallible, but that would mean its own error type.
    fn contains(&self, id: impl AsRef<oid>) -> bool {
        let id = id.as_ref();
        loop {
            {
                let mut indices = self.indices.borrow_mut();
                let mut iter = indices.iter();
                let mut idx = 0;
                while let Some(index) = iter.next() {
                    if index.contains(id) {
                        if idx != 0 {
                            indices.swap(0, idx);
                        }
                        return true;
                    }
                    idx += 1;
                }
            }

            let snapshot = self.snapshot.borrow();
            for lodb in snapshot.loose_dbs.iter() {
                if lodb.contains(id) {
                    return true;
                }
            }

            match self.store.load_one_index(self.refresh_mode, &snapshot.marker) {
                Ok(Some(load_index::Outcome::Replace(mut new_snapshot))) => {
                    drop(snapshot);
                    *self.indices.borrow_mut() = std::mem::take(&mut new_snapshot.indices);
                    *self.snapshot.borrow_mut() = new_snapshot;
                }
                Ok(None) => return false, // nothing more to load, or our refresh mode doesn't allow disk refreshes
                Err(_) => return false, // something went wrong, nothing we can communicate here with this trait. TODO: Maybe that should change?
            }
        }
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
