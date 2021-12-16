use std::ops::Deref;

use crate::general::{handle, load_index};
use git_hash::oid;
use git_object::Data;
use git_pack::{cache::DecodeEntry, data::entry::Location, index::Entry};

mod error {
    use crate::{loose, pack};

    /// Returned by [`compound::Store::try_find()`]
    #[derive(thiserror::Error, Debug)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error("An error occurred while obtaining an object from the loose object store")]
        Loose(#[from] loose::find::Error),
        #[error("An error occurred while obtaining an object from the packed object store")]
        Pack(#[from] pack::data::decode_entry::Error),
        #[error(transparent)]
        LoadIndex(#[from] crate::general::load_index::Error),
        #[error(transparent)]
        LoadPack(#[from] std::io::Error),
    }
}
pub use error::Error;

impl<S> crate::pack::Find for super::Handle<S>
where
    S: Deref<Target = super::Store> + Clone,
{
    type Error = Error;

    // TODO: probably make this method fallible, but that would mean its own error type.
    fn contains(&self, id: impl AsRef<oid>) -> bool {
        let id = id.as_ref();
        loop {
            let mut snapshot = self.snapshot.borrow_mut();
            {
                for (idx, index) in snapshot.indices.iter().enumerate() {
                    if index.contains(id) {
                        if idx != 0 {
                            snapshot.indices.swap(0, idx);
                        }
                        return true;
                    }
                }
            }

            for lodb in snapshot.loose_dbs.iter() {
                if lodb.contains(id) {
                    return true;
                }
            }

            match self.store.load_one_index(self.refresh_mode, snapshot.marker) {
                Ok(Some(load_index::Outcome::Replace(new_snapshot))) => {
                    drop(snapshot);
                    *self.snapshot.borrow_mut() = new_snapshot;
                }
                Ok(None) => return false, // nothing more to load, or our refresh mode doesn't allow disk refreshes
                Err(_) => return false, // something went wrong, nothing we can communicate here with this trait. TODO: Maybe that should change?
            }
        }
    }

    fn try_find_cached<'a>(
        &self,
        id: impl AsRef<oid>,
        buffer: &'a mut Vec<u8>,
        pack_cache: &mut impl DecodeEntry,
    ) -> Result<Option<(Data<'a>, Option<Location>)>, Self::Error> {
        let id = id.as_ref();
        'outer: loop {
            let mut snapshot = self.snapshot.borrow_mut();
            {
                let marker = snapshot.marker;
                for (idx, index) in snapshot.indices.iter_mut().enumerate() {
                    if let Some((handle::IndexForObjectInPack { pack_id, pack_offset }, index_file, possibly_pack)) =
                        index.lookup(id)
                    {
                        let pack = match possibly_pack {
                            Some(pack) => pack,
                            None => match self.store.load_pack(pack_id, marker)? {
                                Some(pack) => {
                                    *possibly_pack = Some(pack);
                                    possibly_pack.as_deref().expect("just put it in")
                                }
                                None => {
                                    // The pack wasn't available anymore so we are supposed to try another round with a fresh index
                                    match self.store.load_one_index(self.refresh_mode, snapshot.marker)? {
                                        Some(load_index::Outcome::Replace(new_snapshot)) => {
                                            drop(snapshot);
                                            *self.snapshot.borrow_mut() = new_snapshot;
                                            continue 'outer;
                                        }
                                        None => {
                                            // nothing new in the index, kind of unexpected to not have a pack but to also
                                            // to have no new index yet. We set the new index before removing any slots, so
                                            // this should be observable.
                                            return Ok(None);
                                        }
                                    }
                                }
                            },
                        };
                        let entry = pack.entry(pack_offset);
                        let header_size = entry.header_size();
                        let res = pack
                            .decode_entry(
                                entry,
                                buffer,
                                |id, _out| {
                                    index_file.lookup(id).map(|idx| {
                                        git_pack::data::ResolvedBase::InPack(
                                            pack.entry(index_file.pack_offset_at_index(idx)),
                                        )
                                    })
                                },
                                pack_cache,
                            )
                            .map(move |r| {
                                (
                                    git_object::Data {
                                        kind: r.kind,
                                        data: buffer.as_slice(),
                                    },
                                    Some(git_pack::data::entry::Location {
                                        pack_id: pack.id,
                                        pack_offset,
                                        entry_size: r.compressed_size + header_size,
                                    }),
                                )
                            })?;

                        if idx != 0 {
                            snapshot.indices.swap(0, idx);
                        }
                        return Ok(Some(res));
                    }
                }
            }

            for lodb in snapshot.loose_dbs.iter() {
                // TODO: remove this double-lookup once the borrow checker allows it.
                if lodb.contains(id) {
                    return lodb
                        .try_find(id, buffer)
                        .map(|obj| obj.map(|obj| (obj, None)))
                        .map_err(Into::into);
                }
            }

            match self.store.load_one_index(self.refresh_mode, snapshot.marker)? {
                Some(load_index::Outcome::Replace(new_snapshot)) => {
                    drop(snapshot);
                    *self.snapshot.borrow_mut() = new_snapshot;
                }
                None => return Ok(None),
            }
        }
    }

    fn location_by_oid(&self, id: impl AsRef<oid>, buf: &mut Vec<u8>) -> Option<Location> {
        todo!("location by oid")
    }

    fn index_iter_by_pack_id(&self, pack_id: u32) -> Option<Box<dyn Iterator<Item = Entry> + '_>> {
        todo!("index iter by pack id")
    }

    fn entry_by_location(&self, location: &Location) -> Option<git_pack::find::Entry<'_>> {
        todo!("entry by location")
    }
}
