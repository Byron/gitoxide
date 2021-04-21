use crate::{compound, data, linked, pack};

impl linked::Db {
    /// Find an object as identified by [`ObjectId`][git_hash::ObjectId] and store its data in full in the provided `buffer`.
    /// This will search the object in all contained compound object databases.
    /// Use a `pack_cache` to accelerate pack access by reducing the amount of work duplication, or [`pack::cache::Noop`] to disable any caching.
    pub fn locate<'a>(
        &self,
        id: impl AsRef<git_hash::oid>,
        buffer: &'a mut Vec<u8>,
        pack_cache: &mut impl pack::cache::DecodeEntry,
    ) -> Result<Option<data::Object<'a>>, compound::locate::Error> {
        use compound::locate::PackInfo;
        let id = id.as_ref();
        for db in self.dbs.iter() {
            match db.internal_locate(id) {
                Some(PackInfo { pack_id, entry_index }) => {
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
}

mod traits {
    use crate::data::Object;
    use crate::pack::cache::DecodeEntry;
    use crate::{compound, linked};
    use git_hash::oid;

    impl crate::Locate for linked::Db {
        type Error = compound::locate::Error;

        fn locate<'a>(
            &self,
            id: impl AsRef<oid>,
            buffer: &'a mut Vec<u8>,
            pack_cache: &mut impl DecodeEntry,
        ) -> Result<Option<Object<'a>>, Self::Error> {
            linked::Db::locate(self, id, buffer, pack_cache)
        }
    }
    impl crate::Locate for &linked::Db {
        type Error = compound::locate::Error;

        fn locate<'a>(
            &self,
            id: impl AsRef<oid>,
            buffer: &'a mut Vec<u8>,
            pack_cache: &mut impl DecodeEntry,
        ) -> Result<Option<Object<'a>>, Self::Error> {
            linked::Db::locate(self, id, buffer, pack_cache)
        }
    }
}
