use std::{collections::HashSet, convert::TryInto, ops::Deref};

use git_hash::{oid, ObjectId};
use git_pack::{cache::DecodeEntry, data::entry::Location};

use crate::store::{handle, load_index};

mod error {
    use crate::{loose, pack};

    /// Returned by [`Handle::try_find()`][git_pack::Find::try_find()]
    #[derive(thiserror::Error, Debug)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error("An error occurred while obtaining an object from the loose object store")]
        Loose(#[from] loose::find::Error),
        #[error("An error occurred looking up a prefix which requires iteration")]
        LooseWalkDir(#[from] loose::iter::Error),
        #[error("An error occurred while obtaining an object from the packed object store")]
        Pack(#[from] pack::data::decode_entry::Error),
        #[error(transparent)]
        LoadIndex(#[from] crate::store::load_index::Error),
        #[error(transparent)]
        LoadPack(#[from] std::io::Error),
        #[error("Object {} referred to its base object {} by its id but it's not within a multi-index", .id, .base_id)]
        ThinPackAtRest {
            /// the id of the base object which lived outside of the multi-index
            base_id: git_hash::ObjectId,
            /// The original object to lookup
            id: git_hash::ObjectId,
        },
        #[error("Reached recursion limit of {} while resolving ref delta bases for {}", .max_depth, .id)]
        DeltaBaseRecursionLimit {
            /// the maximum recursion depth we encountered.
            max_depth: usize,
            /// The original object to lookup
            id: git_hash::ObjectId,
        },
        #[error("The base object {} could not be found but is required to decode {}", .base_id, .id)]
        DeltaBaseMissing {
            /// the id of the base object which failed to lookup
            base_id: git_hash::ObjectId,
            /// The original object to lookup
            id: git_hash::ObjectId,
        },
        #[error("An error occurred when looking up a ref delta base object {} to decode {}", .base_id, .id)]
        DeltaBaseLookup {
            #[source]
            err: Box<Self>,
            /// the id of the base object which failed to lookup
            base_id: git_hash::ObjectId,
            /// The original object to lookup
            id: git_hash::ObjectId,
        },
    }

    #[derive(Copy, Clone)]
    pub(crate) struct DeltaBaseRecursion<'a> {
        pub depth: usize,
        pub original_id: &'a git_hash::oid,
    }

    impl<'a> DeltaBaseRecursion<'a> {
        pub fn new(id: &'a git_hash::oid) -> Self {
            Self {
                original_id: id,
                depth: 0,
            }
        }
        pub fn inc_depth(mut self) -> Self {
            self.depth += 1;
            self
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn error_size() {
            assert_eq!(std::mem::size_of::<Error>(), 88, "should not grow without us noticing");
        }
    }
}
pub use error::Error;

use crate::{
    find::{PotentialPrefix, PrefixLookupResult},
    store::types::PackId,
    Find,
};

impl<S> super::Handle<S>
where
    S: Deref<Target = super::Store> + Clone,
{
    /// Return the exact number of packed objects after loading all currently available indices
    /// as last seen on disk.
    pub fn packed_object_count(&self) -> Result<u64, Error> {
        let mut count = self.packed_object_count.borrow_mut();
        match *count {
            Some(count) => Ok(count),
            None => {
                let mut snapshot = self.snapshot.borrow_mut();
                *snapshot = self.store.load_all_indices()?;
                let mut obj_count = 0;
                for index in &snapshot.indices {
                    obj_count += index.num_objects() as u64;
                }
                *count = Some(obj_count);
                Ok(obj_count)
            }
        }
    }

    /// Given a prefix `candidate` with an object id and an initial `hex_len`, check if it only matches a single
    /// object within the entire object database and increment its `hex_len` by one until it is unambiguous.
    /// Return `Ok(None)` if no object with that prefix exists.
    pub fn disambiguate_prefix(&self, mut candidate: PotentialPrefix) -> Result<Option<git_hash::Prefix>, Error> {
        let max_hex_len = candidate.id().kind().len_in_hex();
        if candidate.hex_len() == max_hex_len {
            return Ok(self.contains(candidate.id()).then(|| candidate.to_prefix()));
        }

        while candidate.hex_len() != max_hex_len {
            let res = self.lookup_prefix(candidate.to_prefix(), None)?;
            match res {
                Some(Ok(_id)) => return Ok(Some(candidate.to_prefix())),
                Some(Err(())) => {
                    candidate.inc_hex_len();
                    continue;
                }
                None => return Ok(None),
            }
        }
        Ok(Some(candidate.to_prefix()))
    }
    /// Find the only object matching `prefix` and return it as `Ok(Some(Ok(<ObjectId>)))`, or return `Ok(Some(Err(()))`
    /// if multiple different objects with the same prefix were found.
    ///
    /// Return `Ok(None)` if no object matched the `prefix`.
    ///
    /// Pass `candidates` to obtain the set of all object ids matching `prefix`, with the same return value as
    /// one would have received if it remained `None`.
    ///
    /// ### Performance Note
    ///
    /// - Unless the handles refresh mode is set to `Never`, each lookup will trigger a refresh of the object databases files
    ///   on disk if the prefix doesn't lead to ambiguous results.
    /// - Since all objects need to be examined to assure non-amiguous return values, after calling this method all indices will
    ///   be loaded.
    /// - If `candidates` is `Some(…)`, the traversal will continue to obtain all candidates, which takes more time
    ///   as there is no early abort.
    pub fn lookup_prefix(
        &self,
        prefix: git_hash::Prefix,
        mut candidates: Option<&mut HashSet<git_hash::ObjectId>>,
    ) -> Result<Option<PrefixLookupResult>, Error> {
        let mut candidate: Option<ObjectId> = None;
        loop {
            let snapshot = self.snapshot.borrow();
            for index in snapshot.indices.iter() {
                #[allow(clippy::needless_option_as_deref)] // needed as it's the equivalent of a reborrow.
                let lookup_result = index.lookup_prefix(prefix, candidates.as_deref_mut());
                if candidates.is_none() && !check_candidate(lookup_result, &mut candidate) {
                    return Ok(Some(Err(())));
                }
            }

            for lodb in snapshot.loose_dbs.iter() {
                #[allow(clippy::needless_option_as_deref)] // needed as it's the equivalent of a reborrow.
                let lookup_result = lodb.lookup_prefix(prefix, candidates.as_deref_mut())?;
                if candidates.is_none() && !check_candidate(lookup_result, &mut candidate) {
                    return Ok(Some(Err(())));
                }
            }

            match self.store.load_one_index(self.refresh, snapshot.marker)? {
                Some(new_snapshot) => {
                    drop(snapshot);
                    *self.snapshot.borrow_mut() = new_snapshot;
                }
                None => {
                    return match &candidates {
                        Some(candidates) => match candidates.len() {
                            0 => Ok(None),
                            1 => Ok(candidates.iter().cloned().next().map(Ok)),
                            _ => Ok(Some(Err(()))),
                        },
                        None => Ok(candidate.map(Ok)),
                    };
                }
            }
        }

        fn check_candidate(lookup_result: Option<PrefixLookupResult>, candidate: &mut Option<ObjectId>) -> bool {
            match (lookup_result, &*candidate) {
                (Some(Ok(oid)), Some(candidate)) if *candidate != oid => false,
                (Some(Ok(_)), Some(_)) | (None, None) | (None, Some(_)) => true,
                (Some(Err(())), _) => false,
                (Some(Ok(oid)), None) => {
                    *candidate = Some(oid);
                    true
                }
            }
        }
    }

    fn try_find_cached_inner<'a, 'b>(
        &'b self,
        mut id: &'b oid,
        buffer: &'a mut Vec<u8>,
        pack_cache: &mut impl DecodeEntry,
        snapshot: &mut load_index::Snapshot,
        recursion: Option<error::DeltaBaseRecursion<'_>>,
    ) -> Result<Option<(git_object::Data<'a>, Option<Location>)>, Error> {
        if let Some(r) = recursion {
            if r.depth >= self.max_recursion_depth {
                return Err(Error::DeltaBaseRecursionLimit {
                    max_depth: self.max_recursion_depth,
                    id: r.original_id.to_owned(),
                });
            }
        } else if !self.ignore_replacements {
            if let Ok(pos) = self
                .store
                .replacements
                .binary_search_by(|(map_this, _)| map_this.as_ref().cmp(id))
            {
                id = self.store.replacements[pos].1.as_ref();
            }
        }

        'outer: loop {
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
                                    match self.store.load_one_index(self.refresh, snapshot.marker)? {
                                        Some(new_snapshot) => {
                                            *snapshot = new_snapshot;
                                            self.clear_cache();
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
                        let res = match pack.decode_entry(
                            entry,
                            buffer,
                            |id, _out| {
                                index_file
                                    .pack_offset_by_id(id)
                                    .map(|pack_offset| git_pack::data::ResolvedBase::InPack(pack.entry(pack_offset)))
                            },
                            pack_cache,
                        ) {
                            Ok(r) => Ok((
                                git_object::Data {
                                    kind: r.kind,
                                    data: buffer.as_slice(),
                                },
                                Some(git_pack::data::entry::Location {
                                    pack_id: pack.id,
                                    pack_offset,
                                    entry_size: r.compressed_size + header_size,
                                }),
                            )),
                            Err(git_pack::data::decode_entry::Error::DeltaBaseUnresolved(base_id)) => {
                                // Only with multi-pack indices it's allowed to jump to refer to other packs within this
                                // multi-pack. Otherwise this would constitute a thin pack which is only allowed in transit.
                                // However, if we somehow end up with that, we will resolve it safely, even though we could
                                // avoid handling this case and error instead.

                                // Since this is a special case, we just allocate here to make it work. It's an actual delta-ref object
                                // which is sent by some servers that points to an object outside of the pack we are looking
                                // at right now. With the complexities of loading packs, we go into recursion here. Git itself
                                // doesn't do a cycle check, and we won't either but limit the recursive depth.
                                // The whole ordeal isn't as efficient as it could be due to memory allocation and
                                // later mem-copying when trying again.
                                let mut buf = Vec::new();
                                let obj_kind = self
                                    .try_find_cached_inner(
                                        &base_id,
                                        &mut buf,
                                        pack_cache,
                                        snapshot,
                                        recursion
                                            .map(|r| r.inc_depth())
                                            .or_else(|| error::DeltaBaseRecursion::new(id).into()),
                                    )
                                    .map_err(|err| Error::DeltaBaseLookup {
                                        err: Box::new(err),
                                        base_id,
                                        id: id.to_owned(),
                                    })?
                                    .ok_or_else(|| Error::DeltaBaseMissing {
                                        base_id,
                                        id: id.to_owned(),
                                    })?
                                    .0
                                    .kind;
                                let index = &mut snapshot.indices[idx];
                                let handle::index_lookup::Outcome {
                                    object_index:
                                        handle::IndexForObjectInPack {
                                            pack_id: _,
                                            pack_offset,
                                        },
                                    index_file,
                                    pack: possibly_pack,
                                } = index.lookup(id).expect("to find the object again in snapshot");
                                let pack = possibly_pack
                                    .as_ref()
                                    .expect("pack to still be available like just now");
                                let entry = pack.entry(pack_offset);
                                let header_size = entry.header_size();
                                pack.decode_entry(
                                    entry,
                                    buffer,
                                    |id, out| {
                                        index_file
                                            .pack_offset_by_id(id)
                                            .map(|pack_offset| {
                                                git_pack::data::ResolvedBase::InPack(pack.entry(pack_offset))
                                            })
                                            .or_else(|| {
                                                (id == base_id).then(|| {
                                                    out.resize(buf.len(), 0);
                                                    out.copy_from_slice(buf.as_slice());
                                                    git_pack::data::ResolvedBase::OutOfPack {
                                                        kind: obj_kind,
                                                        end: out.len(),
                                                    }
                                                })
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
                                })
                            }
                            Err(err) => Err(err),
                        }?;

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

            match self.store.load_one_index(self.refresh, snapshot.marker)? {
                Some(new_snapshot) => {
                    *snapshot = new_snapshot;
                    self.clear_cache();
                }
                None => return Ok(None),
            }
        }
    }

    fn clear_cache(&self) {
        self.packed_object_count.borrow_mut().take();
    }
}

impl<S> git_pack::Find for super::Handle<S>
where
    S: Deref<Target = super::Store> + Clone,
{
    type Error = Error;

    // TODO: probably make this method fallible, but that would mean its own error type.
    fn contains(&self, id: impl AsRef<oid>) -> bool {
        let id = id.as_ref();
        let mut snapshot = self.snapshot.borrow_mut();
        loop {
            for (idx, index) in snapshot.indices.iter().enumerate() {
                if index.contains(id) {
                    if idx != 0 {
                        snapshot.indices.swap(0, idx);
                    }
                    return true;
                }
            }

            for lodb in snapshot.loose_dbs.iter() {
                if lodb.contains(id) {
                    return true;
                }
            }

            match self.store.load_one_index(self.refresh, snapshot.marker) {
                Ok(Some(new_snapshot)) => {
                    *snapshot = new_snapshot;
                    self.clear_cache();
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
    ) -> Result<Option<(git_object::Data<'a>, Option<Location>)>, Self::Error> {
        let id = id.as_ref();
        let mut snapshot = self.snapshot.borrow_mut();
        self.try_find_cached_inner(id, buffer, pack_cache, &mut snapshot, None)
    }

    fn location_by_oid(&self, id: impl AsRef<oid>, buf: &mut Vec<u8>) -> Option<Location> {
        assert!(
            matches!(self.token.as_ref(), Some(handle::Mode::KeepDeletedPacksAvailable)),
            "BUG: handle must be configured to `prevent_pack_unload()` before using this method"
        );

        assert!(self.store_ref().replacements.is_empty() || self.ignore_replacements, "Everything related to packing must not use replacements. These are not used here, but it should be turned off for good measure.");

        let id = id.as_ref();
        let mut snapshot = self.snapshot.borrow_mut();
        'outer: loop {
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
                                    match self.store.load_one_index(self.refresh, snapshot.marker).ok()? {
                                        Some(new_snapshot) => {
                                            *snapshot = new_snapshot;
                                            self.clear_cache();
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

            match self.store.load_one_index(self.refresh, snapshot.marker).ok()? {
                Some(new_snapshot) => {
                    *snapshot = new_snapshot;
                    self.clear_cache();
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
        let pack_id = PackId::from_intrinsic_pack_id(pack_id);
        loop {
            let snapshot = self.snapshot.borrow();
            {
                for index in snapshot.indices.iter() {
                    if let Some(iter) = index.iter(pack_id) {
                        return Some(iter.map(|e| (e.pack_offset, e.oid)).collect());
                    }
                }
            }

            match self.store.load_one_index(self.refresh, snapshot.marker).ok()? {
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
        let pack_id = PackId::from_intrinsic_pack_id(location.pack_id);
        let mut snapshot = self.snapshot.borrow_mut();
        let marker = snapshot.marker;
        loop {
            {
                for index in snapshot.indices.iter_mut() {
                    if let Some(possibly_pack) = index.pack(pack_id) {
                        let pack = match possibly_pack {
                            Some(pack) => pack,
                            None => {
                                let pack = self.store.load_pack(pack_id, marker).ok()?.expect(
                                "BUG: pack must exist from previous call to location_by_oid() and must not be unloaded",
                            );
                                *possibly_pack = Some(pack);
                                possibly_pack.as_deref().expect("just put it in")
                            }
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

            snapshot.indices.insert(
                0,
                self.store
                    .index_by_id(pack_id, marker)
                    .expect("BUG: index must always be present, must not be unloaded or overwritten"),
            );
        }
    }
}

impl<S> crate::Find for super::Handle<S>
where
    S: Deref<Target = super::Store> + Clone,
    Self: git_pack::Find,
{
    type Error = <Self as git_pack::Find>::Error;

    fn contains(&self, id: impl AsRef<oid>) -> bool {
        git_pack::Find::contains(self, id)
    }

    fn try_find<'a>(
        &self,
        id: impl AsRef<oid>,
        buffer: &'a mut Vec<u8>,
    ) -> Result<Option<git_object::Data<'a>>, Self::Error> {
        git_pack::Find::try_find(self, id, buffer).map(|t| t.map(|t| t.0))
    }
}
