use crate::{compound, loose, pack};

/// Returned by [`compound::Db::locate()`]
#[derive(thiserror::Error, Debug)]
#[allow(missing_docs)]
pub enum Error {
    #[error("An error occurred while obtaining an object from the loose object store")]
    Loose(#[from] loose::db::locate::Error),
    #[error("An error occurred while obtaining an object from the packed object store")]
    Pack(#[from] pack::data::decode::Error),
}

impl compound::Db {
    /// Find an object as identified by [`ObjectId`][git_hash::ObjectId] and store its data in full in the provided `buffer`.
    /// This will search the object in all contained object databases.
    /// Use a cache to accelerate pack access, or [`pack::cache::Noop`] to disable any caching.
    pub fn locate<'a>(
        &self,
        id: impl AsRef<git_hash::oid>,
        buffer: &'a mut Vec<u8>,
        pack_cache: &mut impl pack::cache::DecodeEntry,
    ) -> Result<Option<compound::Object<'a>>, Error> {
        let id = id.as_ref();
        for pack in &self.packs {
            if let Some(idx) = pack.internal_locate_index(id) {
                let object = pack.internal_get_object_by_index(idx, buffer, pack_cache)?;
                return Ok(Some(compound::Object::Borrowed(object)));
            }
        }
        if let Some(object) = self.loose.locate(id)? {
            return Ok(Some(compound::Object::Loose(Box::new(object))));
        }
        Ok(None)
    }
}
