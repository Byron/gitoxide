use std::{convert::TryInto, ops::Deref};

use git_hash::oid;
use git_object::Data;
use git_pack::{cache::DecodeEntry, data::entry::Location};

use crate::general::handle;

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

use crate::general;

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
                Ok(Some(new_snapshot)) => {
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
                    if let Some(handle::index_lookup::Outcome {
                        object_index: handle::IndexForObjectInPack { pack_id, pack_offset },
                        index_file,
                        pack: possibly_pack,
                    }) = index.lookup(id)
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
                                        Some(new_snapshot) => {
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
                Some(new_snapshot) => {
                    drop(snapshot);
                    *self.snapshot.borrow_mut() = new_snapshot;
                }
                None => return Ok(None),
            }
        }
    }

    fn location_by_oid(&self, id: impl AsRef<oid>, buf: &mut Vec<u8>) -> Option<Location> {
        assert!(
            matches!(self.token.as_ref(), Some(handle::Mode::KeepDeletedPacksAvailable)),
            "BUG: handle must be configured to `prevent_pack_unload()` before using this method"
        );
        let id = id.as_ref();
        'outer: loop {
            let mut snapshot = self.snapshot.borrow_mut();
            {
                let marker = snapshot.marker;
                for (idx, index) in snapshot.indices.iter_mut().enumerate() {
                    if let Some(handle::index_lookup::Outcome {
                        object_index: handle::IndexForObjectInPack { pack_id, pack_offset },
                        index_file: _,
                        pack: possibly_pack,
                    }) = index.lookup(id)
                    {
                        let pack = match possibly_pack {
                            Some(pack) => pack,
                            None => match self.store.load_pack(pack_id, marker).ok()? {
                                Some(pack) => {
                                    *possibly_pack = Some(pack);
                                    possibly_pack.as_deref().expect("just put it in")
                                }
                                None => {
                                    // The pack wasn't available anymore so we are supposed to try another round with a fresh index
                                    match self.store.load_one_index(self.refresh_mode, snapshot.marker).ok()? {
                                        Some(new_snapshot) => {
                                            drop(snapshot);
                                            *self.snapshot.borrow_mut() = new_snapshot;
                                            continue 'outer;
                                        }
                                        None => {
                                            // nothing new in the index, kind of unexpected to not have a pack but to also
                                            // to have no new index yet. We set the new index before removing any slots, so
                                            // this should be observable.
                                            return None;
                                        }
                                    }
                                }
                            },
                        };
                        let entry = pack.entry(pack_offset);

                        buf.resize(entry.decompressed_size.try_into().expect("representable size"), 0);
                        assert_eq!(pack.id, pack_id.to_intrinsic_pack_id(), "both ids must always match");

                        let res = pack.decompress_entry(&entry, buf).ok().map(|entry_size_past_header| {
                            git_pack::data::entry::Location {
                                pack_id: pack.id,
                                pack_offset,
                                entry_size: entry.header_size() + entry_size_past_header,
                            }
                        });

                        if idx != 0 {
                            snapshot.indices.swap(0, idx);
                        }
                        return res;
                    }
                }
            }

            match self.store.load_one_index(self.refresh_mode, snapshot.marker).ok()? {
                Some(new_snapshot) => {
                    drop(snapshot);
                    *self.snapshot.borrow_mut() = new_snapshot;
                }
                None => return None,
            }
        }
    }

    fn pack_offsets_and_oid(&self, pack_id: u32) -> Option<Vec<(u64, git_hash::ObjectId)>> {
        assert!(
            matches!(self.token.as_ref(), Some(handle::Mode::KeepDeletedPacksAvailable)),
            "BUG: handle must be configured to `prevent_pack_unload()` before using this method"
        );
        let pack_id = general::store::PackId::from_intrinsic_pack_id(pack_id);
        loop {
            let snapshot = self.snapshot.borrow();
            {
                for index in snapshot.indices.iter() {
                    if let Some(iter) = index.iter(pack_id) {
                        return Some(iter.map(|e| (e.pack_offset, e.oid)).collect());
                    }
                }
            }

            match self.store.load_one_index(self.refresh_mode, snapshot.marker).ok()? {
                Some(new_snapshot) => {
                    drop(snapshot);
                    *self.snapshot.borrow_mut() = new_snapshot;
                }
                None => return None,
            }
        }
    }

    fn entry_by_location(&self, location: &Location) -> Option<git_pack::find::Entry> {
        assert!(
            matches!(self.token.as_ref(), Some(handle::Mode::KeepDeletedPacksAvailable)),
            "BUG: handle must be configured to `prevent_pack_unload()` before using this method"
        );
        let pack_id = general::store::PackId::from_intrinsic_pack_id(location.pack_id);
        'outer: loop {
            let mut snapshot = self.snapshot.borrow_mut();
            {
                let marker = snapshot.marker;
                for index in snapshot.indices.iter_mut() {
                    if let Some(possibly_pack) = index.pack(pack_id) {
                        let pack = match possibly_pack {
                            Some(pack) => pack,
                            None => match self.store.load_pack(pack_id, marker).ok()? {
                                Some(pack) => {
                                    *possibly_pack = Some(pack);
                                    possibly_pack.as_deref().expect("just put it in")
                                }
                                None => {
                                    // The pack wasn't available anymore so we are supposed to try another round with a fresh index
                                    match self.store.load_one_index(self.refresh_mode, snapshot.marker).ok()? {
                                        Some(new_snapshot) => {
                                            drop(snapshot);
                                            *self.snapshot.borrow_mut() = new_snapshot;
                                            continue 'outer;
                                        }
                                        None => {
                                            // nothing new in the index, kind of unexpected to not have a pack but to also
                                            // to have no new index yet. We set the new index before removing any slots, so
                                            // this should be observable.
                                            return None;
                                        }
                                    }
                                }
                            },
                        };
                        return pack
                            .entry_slice(location.entry_range(location.pack_offset))
                            .map(|data| git_pack::find::Entry {
                                data: data.to_owned(),
                                version: pack.version(),
                            });
                    }
                }
            }

            match self.store.load_one_index(self.refresh_mode, snapshot.marker).ok()? {
                Some(new_snapshot) => {
                    drop(snapshot);
                    *self.snapshot.borrow_mut() = new_snapshot;
                }
                None => return None,
            }
        }
    }
}
