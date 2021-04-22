use crate::{compound, data, linked, pack, PackEntry};

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

    /// Return the [`PackEntry`] for `object` if it is backed by a pack.
    ///
    /// Note that this is only in the interest of avoiding duplicate work during pack generation
    /// as the input for this is an already decoded [`data::Object`] that is fully known.
    ///
    /// # Notes
    ///
    /// Custom implementations might be interested in providing their own meta-data with `object`,
    /// which currently isn't possible as the `Locate` trait requires GATs to work like that.
    pub fn pack_entry(&self, object: &data::Object<'_>) -> Option<PackEntry<'_>> {
        todo!("pack_entry()")
    }
}

mod traits {
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
            linked::Db::locate(self, id, buffer, pack_cache)
        }

        fn pack_entry(&self, object: &data::Object<'_>) -> Option<PackEntry<'_>> {
            linked::Db::pack_entry(self, object)
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
            linked::Db::locate(self, id, buffer, pack_cache)
        }

        fn pack_entry(&self, object: &data::Object<'_>) -> Option<PackEntry<'_>> {
            linked::Db::pack_entry(self, object)
        }
    }
}
