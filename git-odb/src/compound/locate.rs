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
    pub fn locate<'a>(
        &self,
        id: impl AsRef<git_hash::oid>,
        buffer: &'a mut Vec<u8>,
    ) -> Result<Option<compound::Object<'a>>, Error> {
        let id = id.as_ref();
        for pack in &self.packs {
            if let Some(idx) = pack.internal_locate_index(id) {
                let object = pack.internal_get_object_by_index(idx, buffer, &mut pack::cache::Noop)?;
                return Ok(Some(compound::Object::Borrowed(object)));
            }
        }
        if let Some(object) = self.loose.locate(id)? {
            return Ok(Some(compound::Object::Loose(Box::new(object))));
        }
        for alternate in &self.alternates {
            // See 8c5bd095539042d7db0e611460803cdbf172beb0 for a commit that adds polonius and makes the proper version compile.
            // See https://stackoverflow.com/questions/63906425/nll-limitation-how-to-work-around-cannot-borrow-buf-as-mutable-more-than?noredirect=1#comment113007288_63906425
            // More see below! Of course we don't want to do the lookup twice… but have to until this is fixed or we compile nightly.
            #[cfg(not(feature = "polonius"))]
            if alternate.locate(id, buffer)?.is_some() {
                return alternate.locate(id, buffer);
            }
            #[cfg(feature = "polonius")]
            if let Some(object) = alternate.locate(id, buffer)? {
                return Ok(Some(object));
            }
        }
        Ok(None)
    }

    /// Find an object as identified by [`ObjectId`][git_hash::ObjectId] and store its data in full in the provided `buffer`.
    /// This will search the object in all contained object databases.
    /// Use a cache to accelerate pack access.
    pub fn locate_with_cache<'a>(
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
        for alternate in &self.alternates {
            // See 8c5bd095539042d7db0e611460803cdbf172beb0 for a commit that adds polonius and makes the proper version compile.
            // See https://stackoverflow.com/questions/63906425/nll-limitation-how-to-work-around-cannot-borrow-buf-as-mutable-more-than?noredirect=1#comment113007288_63906425
            // More see below! Of course we don't want to do the lookup twice… but have to until this is fixed or we compile nightly.
            #[cfg(not(feature = "polonius"))]
            if alternate.locate_with_cache(id, buffer, pack_cache)?.is_some() {
                return alternate.locate_with_cache(id, buffer, pack_cache);
            }
            #[cfg(feature = "polonius")]
            if let Some(object) = alternate.locate_with_cache(id, buffer, pack_cache)? {
                return Ok(Some(object));
            }
        }
        Ok(None)
    }
}
