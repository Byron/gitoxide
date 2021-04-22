use crate::{compound, data, data::Object, linked, pack, PackEntry};
use git_hash::oid;

impl crate::Locate for linked::Db {
    type Error = compound::locate::Error;

    fn locate<'a>(
        &self,
        id: impl AsRef<oid>,
        buffer: &'a mut Vec<u8>,
        pack_cache: &mut impl pack::cache::DecodeEntry,
    ) -> Result<Option<Object<'a>>, Self::Error> {
        let id = id.as_ref();
        for db in self.dbs.iter() {
            match db.internal_locate(id) {
                Some(compound::locate::PackLocation { pack_id, entry_index }) => {
                    return db
                        .internal_get_packed_object_by_index(pack_id, entry_index, buffer, pack_cache)
                        .map(Some)
                        .map_err(Into::into)
                }
                None => {
                    if db.loose.contains(id) {
                        return db.loose.locate(id, buffer).map_err(Into::into);
                    }
                }
            }
        }
        Ok(None)
    }

    fn pack_entry(&self, object: &data::Object<'_>) -> Option<PackEntry<'_>> {
        todo!("pack_entry()")
    }
}

impl crate::Locate for &linked::Db {
    type Error = compound::locate::Error;

    fn locate<'a>(
        &self,
        id: impl AsRef<oid>,
        buffer: &'a mut Vec<u8>,
        pack_cache: &mut impl pack::cache::DecodeEntry,
    ) -> Result<Option<Object<'a>>, Self::Error> {
        (*self).locate(id, buffer, pack_cache)
    }

    fn pack_entry(&self, object: &data::Object<'_>) -> Option<PackEntry<'_>> {
        (*self).pack_entry(object)
    }
}
