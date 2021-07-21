use git_hash::oid;
use std::convert::TryInto;

use crate::{
    pack,
    pack::bundle::Location,
    store::{compound, linked},
};
use git_pack::{data::Object, find::Entry};

impl linked::Store {
    /// Return true if the given object `id` is contained in the store.
    pub fn contains(&self, id: impl AsRef<oid>) -> bool {
        let id = id.as_ref();
        for db in self.dbs.iter() {
            if db.internal_find_packed(id).is_some() || db.loose.contains(id) {
                return true;
            }
        }
        false
    }
}

impl crate::Find for linked::Store {
    type Error = compound::find::Error;

    fn find<'a>(
        &self,
        id: impl AsRef<oid>,
        buffer: &'a mut Vec<u8>,
        pack_cache: &mut impl pack::cache::DecodeEntry,
    ) -> Result<Option<Object<'a>>, Self::Error> {
        let id = id.as_ref();
        for db in self.dbs.iter() {
            match db.internal_find_packed(id) {
                Some(compound::find::PackLocation {
                    bundle_index: pack_id,
                    entry_index,
                }) => {
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

    fn location_by_id(&self, id: impl AsRef<oid>, buf: &mut Vec<u8>) -> Option<pack::bundle::Location> {
        let id = id.as_ref();
        for db in self.dbs.iter() {
            if let Some(compound::find::PackLocation {
                bundle_index,
                entry_index,
            }) = db.internal_find_packed(id)
            {
                let bundle = &db.bundles[bundle_index];
                let pack_offset = bundle.index.pack_offset_at_index(entry_index);
                let entry = bundle.pack.entry(pack_offset);

                buf.resize(entry.decompressed_size.try_into().expect("representable size"), 0);
                return bundle
                    .pack
                    .decompress_entry(&entry, buf)
                    .ok()
                    .map(|entry_size_past_header| pack::bundle::Location {
                        pack_id: bundle.pack.id,
                        pack_offset,
                        index_file_id: entry_index,
                        entry_size: entry.header_size() + entry_size_past_header,
                    });
            }
        }
        None
    }

    fn entry_by_location(&self, location: &pack::bundle::Location) -> Option<Entry<'_>> {
        self.dbs
            .iter()
            .find_map(|db| db.bundles.iter().find(|p| p.pack.id == location.pack_id))
            .map(|b| (b, location))
            .and_then(|(bundle, l)| {
                let crc32 = bundle.index.crc32_at_index(l.index_file_id);
                let pack_offset = bundle.index.pack_offset_at_index(l.index_file_id);
                bundle.pack.entry_slice(l.entry_range(pack_offset)).map(|data| Entry {
                    data,
                    crc32,
                    version: bundle.pack.version(),
                })
            })
    }
}

impl crate::Find for &linked::Store {
    type Error = compound::find::Error;

    fn find<'a>(
        &self,
        id: impl AsRef<oid>,
        buffer: &'a mut Vec<u8>,
        pack_cache: &mut impl pack::cache::DecodeEntry,
    ) -> Result<Option<Object<'a>>, Self::Error> {
        (*self).find(id, buffer, pack_cache)
    }

    fn location_by_id(&self, id: impl AsRef<oid>, buf: &mut Vec<u8>) -> Option<Location> {
        (*self).location_by_id(id, buf)
    }

    fn entry_by_location(&self, location: &pack::bundle::Location) -> Option<Entry<'_>> {
        (*self).entry_by_location(location)
    }
}
