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

pub(crate) enum LooseOrPack {
    Loose(Box<loose::Object>),
    Packed(usize, u32),
}

impl compound::Db {
    /// Find an object as identified by [`ObjectId`][git_hash::ObjectId] and store its data in full in the provided `buffer`.
    /// This will search the object in all contained object databases.
    /// Use a `pack_cache` to accelerate pack access by reducing the amount of work duplication, or [`pack::cache::Noop`] to disable any caching.
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

    /// Internal-use function to look up a packed object index or loose object.
    /// Used to avoid double-lookups in linked::Db::locate.
    /// (The polonius borrow-checker would support this via the locate
    /// function, so this can be [simplified](https://github.com/Byron/gitoxide/blob/0c5f4043da4615820cb180804a81c2d4fe75fe5e/git-odb/src/compound/locate.rs#L47)
    /// once polonius is stable.)
    pub(crate) fn internal_locate(&self, id: impl AsRef<git_hash::oid>) -> Result<Option<LooseOrPack>, Error> {
        let id = id.as_ref();
        for (pack_idx, pack) in self.packs.iter().enumerate() {
            if let Some(idx) = pack.internal_locate_index(id) {
                return Ok(Some(LooseOrPack::Packed(pack_idx, idx)));
            }
        }
        if let Some(object) = self.loose.locate(id)? {
            return Ok(Some(LooseOrPack::Loose(Box::new(object))));
        }
        Ok(None)
    }

    pub(crate) fn internal_get_packed_object_by_index<'a>(
        &self,
        pack_index: usize,
        object_index: u32,
        buffer: &'a mut Vec<u8>,
        pack_cache: &mut impl pack::cache::DecodeEntry,
    ) -> Result<crate::borrowed::Object<'a>, pack::data::decode::Error> {
        self.packs[pack_index].internal_get_object_by_index(object_index, buffer, pack_cache)
    }
}
