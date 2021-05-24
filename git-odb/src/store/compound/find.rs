use crate::store::compound;
use crate::{data, pack, store::loose};

/// Returned by [`compound::Backend::find()`]
#[derive(thiserror::Error, Debug)]
#[allow(missing_docs)]
pub enum Error {
    #[error("An error occurred while obtaining an object from the loose object store")]
    Loose(#[from] loose::backend::find::Error),
    #[error("An error occurred while obtaining an object from the packed object store")]
    Pack(#[from] pack::data::decode_entry::Error),
}

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
pub(crate) struct PackLocation {
    pub bundle_index: usize,
    pub entry_index: u32,
}

impl compound::Backend {
    /// Find an object as identified by [`ObjectId`][git_hash::ObjectId] and store its data in full in the provided `buffer`.
    /// This will search the object in all contained object databases.
    /// Use a `pack_cache` to accelerate pack access by reducing the amount of work duplication, or [`pack::cache::Never`] to disable any caching.
    pub fn find<'a>(
        &self,
        id: impl AsRef<git_hash::oid>,
        buffer: &'a mut Vec<u8>,
        pack_cache: &mut impl pack::cache::DecodeEntry,
    ) -> Result<Option<data::Object<'a>>, Error> {
        let id = id.as_ref();
        for pack in &self.bundles {
            if let Some(idx) = pack.internal_find_pack_index(id) {
                let object = pack.internal_get_object_by_index(idx, buffer, pack_cache)?;
                return Ok(Some(object));
            }
        }
        if self.loose.contains(id) {
            return self.loose.find(id, buffer).map_err(Into::into);
        }
        Ok(None)
    }

    /// Internal-use function to look up a packed object index or loose object.
    /// Used to avoid double-lookups in linked::Db::locate.
    /// (The polonius borrow-checker would support this via the locate
    /// function, so this can be [simplified](https://github.com/Byron/gitoxide/blob/0c5f4043da4615820cb180804a81c2d4fe75fe5e/git-odb/src/compound/locate.rs#L47)
    /// once polonius is stable.)
    pub(crate) fn internal_find(&self, id: impl AsRef<git_hash::oid>) -> Option<PackLocation> {
        let id = id.as_ref();
        for (bundle_index, pack) in self.bundles.iter().enumerate() {
            if let Some(idx) = pack.internal_find_pack_index(id) {
                return Some(PackLocation {
                    bundle_index,
                    entry_index: idx,
                });
            }
        }
        None
    }

    pub(crate) fn internal_get_packed_object_by_index<'a>(
        &self,
        pack_index: usize,
        object_index: u32,
        buffer: &'a mut Vec<u8>,
        pack_cache: &mut impl pack::cache::DecodeEntry,
    ) -> Result<data::Object<'a>, pack::data::decode_entry::Error> {
        self.bundles[pack_index].internal_get_object_by_index(object_index, buffer, pack_cache)
    }
}
