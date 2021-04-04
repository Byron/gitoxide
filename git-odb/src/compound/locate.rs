use crate::{compound, loose, pack};
use git_object::borrowed;

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
    /// Find an object as identified by [`id`][borrowed::Id] and store its data in full in the provided `buffer`.
    /// This will search the object in all contained object databases.
    pub fn locate2<'a>(
        &self,
        id: borrowed::Id<'_>,
        buffer: &'a mut Vec<u8>,
    ) -> Result<Option<compound::Object<'a>>, Error> {
        for alternate in &self.alternates {
            // See 8c5bd095539042d7db0e611460803cdbf172beb0 for a commit that adds polonius and makes the proper version compile.
            // See https://stackoverflow.com/questions/63906425/nll-limitation-how-to-work-around-cannot-borrow-buf-as-mutable-more-than?noredirect=1#comment113007288_63906425
            // More see below! Of course we don't want to do the lookup twice… but have to until this is fixed or we compile nightly.
            #[cfg(not(feature = "polonius"))]
            if alternate.locate2(id, buffer)?.is_some() {
                return alternate.locate2(id, buffer);
            }
            #[cfg(feature = "polonius")]
            if let Some(object) = alternate.locate2(id, buffer)? {
                return Ok(Some(object));
            }
        }
        for pack in &self.packs {
            // See 8c5bd095539042d7db0e611460803cdbf172beb0 for a commit that adds polonius and makes the proper version compile.
            // See https://stackoverflow.com/questions/63906425/nll-limitation-how-to-work-around-cannot-borrow-buf-as-mutable-more-than?noredirect=1#comment113007288_63906425
            // The underlying issue is described here https://github.com/rust-lang/rust/issues/45402,
            // Once Polonius becomes a thing AND is not too slow, we must remove this double-lookup to become something like this:
            // if let Some(object) = if pack.locate(id, buffer, &mut pack::cache::DecodeEntryNoop) {…}
            #[cfg(not(feature = "polonius"))]
            if pack.locate2(id, buffer, &mut pack::cache::Noop)?.is_some() {
                let object = pack.locate2(id, buffer, &mut pack::cache::Noop)?;
                return Ok(object.map(compound::Object::Borrowed));
            }
            #[cfg(feature = "polonius")]
            if let Some(object) = pack.locate2(id, buffer, &mut pack::cache::Noop)? {
                return Ok(Some(compound::Object::Borrowed(object)));
            }
        }
        Ok(self.loose.locate2(id)?.map(|object| compound::Object::Loose(object)))
    }
    /// Find an object as identified by [`id`][borrowed::Id] and store its data in full in the provided `buffer`.
    /// This will search the object in all contained object databases.
    pub fn locate<'a>(
        &self,
        id: borrowed::Id<'_>,
        buffer: &'a mut Vec<u8>,
    ) -> Option<Result<compound::Object<'a>, Error>> {
        for alternate in &self.alternates {
            // See 8c5bd095539042d7db0e611460803cdbf172beb0 for a commit that adds polonius and makes the proper version compile.
            // See https://stackoverflow.com/questions/63906425/nll-limitation-how-to-work-around-cannot-borrow-buf-as-mutable-more-than?noredirect=1#comment113007288_63906425
            // More see below! Of course we don't want to do the lookup twice… but have to until this is fixed or we compile nightly.
            #[cfg(not(feature = "polonius"))]
            if alternate.locate(id, buffer).is_some() {
                return alternate.locate(id, buffer);
            }
            #[cfg(feature = "polonius")]
            if let Some(object) = alternate.locate(id, buffer) {
                return Some(object);
            }
        }
        for pack in &self.packs {
            // See 8c5bd095539042d7db0e611460803cdbf172beb0 for a commit that adds polonius and makes the proper version compile.
            // See https://stackoverflow.com/questions/63906425/nll-limitation-how-to-work-around-cannot-borrow-buf-as-mutable-more-than?noredirect=1#comment113007288_63906425
            // The underlying issue is described here https://github.com/rust-lang/rust/issues/45402,
            // Once Polonius becomes a thing AND is not too slow, we must remove this double-lookup to become something like this:
            // if let Some(object) = if pack.locate(id, buffer, &mut pack::cache::DecodeEntryNoop) {…}
            #[cfg(not(feature = "polonius"))]
            if pack.locate(id, buffer, &mut pack::cache::Noop).is_some() {
                let object = pack.locate(id, buffer, &mut pack::cache::Noop).unwrap();
                return Some(object.map(compound::Object::Borrowed).map_err(Into::into));
            }
            #[cfg(feature = "polonius")]
            if let Some(object) = pack.locate(id, buffer, &mut pack::cache::Noop) {
                return Some(object.map(compound::Object::Borrowed).map_err(Into::into));
            }
        }
        self.loose
            .locate(id)
            .map(|object| object.map(compound::Object::Loose).map_err(Into::into))
    }
}
