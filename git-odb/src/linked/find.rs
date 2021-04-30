use crate::{compound, data, data::Object, linked, pack, PackEntry};
use git_hash::oid;

impl crate::Find for linked::Db {
    type Error = compound::find::Error;

    fn find<'a>(
        &self,
        id: impl AsRef<oid>,
        buffer: &'a mut Vec<u8>,
        pack_cache: &mut impl pack::cache::DecodeEntry,
    ) -> Result<Option<Object<'a>>, Self::Error> {
        let id = id.as_ref();
        for db in self.dbs.iter() {
            match db.internal_find(id) {
                Some(compound::find::PackLocation { pack_id, entry_index }) => {
                    return db
                        .internal_get_packed_object_by_index(pack_id, entry_index, buffer, pack_cache)
                        .map(Some)
                        .map_err(Into::into)
                }
                None => {
                    if db.loose.contains(id) {
                        return db.loose.find(id, buffer).map_err(Into::into);
                    }
                }
            }
        }
        Ok(None)
    }

    fn pack_entry(&self, object: &data::Object<'_>) -> Option<PackEntry<'_>> {
        object
            .pack_location
            .as_ref()
            .and_then(|l| {
                self.dbs
                    .iter()
                    .find_map(|db| db.packs.iter().find(|p| p.pack.id == l.pack_id))
                    .map(|b| (b, l))
            })
            .and_then(|(bundle, l)| {
                let crc32 = bundle.index.crc32_at_index(l.index_file_id);
                let pack_offset = bundle.index.pack_offset_at_index(l.index_file_id);
                bundle
                    .pack
                    .entry_slice(l.entry_range(pack_offset))
                    .map(|data| PackEntry {
                        data,
                        crc32,
                        version: bundle.pack.version(),
                    })
            })
    }
}

impl crate::Find for &linked::Db {
    type Error = compound::find::Error;

    fn find<'a>(
        &self,
        id: impl AsRef<oid>,
        buffer: &'a mut Vec<u8>,
        pack_cache: &mut impl pack::cache::DecodeEntry,
    ) -> Result<Option<Object<'a>>, Self::Error> {
        (*self).find(id, buffer, pack_cache)
    }

    fn pack_entry(&self, object: &data::Object<'_>) -> Option<PackEntry<'_>> {
        (*self).pack_entry(object)
    }
}
