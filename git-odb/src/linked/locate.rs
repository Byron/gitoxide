use crate::{compound, linked, pack};

impl linked::Db {
    pub fn locate<'a>(
        &self,
        id: impl AsRef<git_hash::oid>,
        buffer: &'a mut Vec<u8>,
    ) -> Result<Option<compound::Object<'a>>, compound::locate::Error> {
        let id = id.as_ref();
        for db in self.dbs.iter() {
            match db.internal_locate(id)? {
                Some(compound::locate::LooseOrPack::Loose(object)) => return Ok(Some(compound::Object::Loose(object))),
                Some(compound::locate::LooseOrPack::Packed(pack_index, object_index)) => {
                    return db
                        .internal_get_packed_object_by_index(pack_index, object_index, buffer, &mut pack::cache::Noop)
                        .map(|object| Some(compound::Object::Borrowed(object)))
                        .map_err(Into::into)
                }
                None => continue,
            }
        }
        Ok(None)
    }
}
