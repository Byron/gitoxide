impl crate::Bundle {
    /// Find an object with the given [`ObjectId`][git_hash::ObjectId] and place its data into `out`.
    ///
    /// [`cache`][crate::cache::DecodeEntry] is used to accelerate the lookup.
    ///
    /// **Note** that ref deltas are automatically resolved within this pack only, which makes this implementation unusable
    /// for thin packs, which by now are expected to be resolved already.
    pub fn find<'a>(
        &self,
        id: impl AsRef<git_hash::oid>,
        out: &'a mut Vec<u8>,
        cache: &mut impl crate::cache::DecodeEntry,
    ) -> Result<Option<(git_object::Data<'a>, crate::data::entry::Location)>, crate::data::decode::Error> {
        let idx = match self.index.lookup(id) {
            Some(idx) => idx,
            None => return Ok(None),
        };
        self.get_object_by_index(idx, out, cache).map(Some)
    }

    /// Special-use function to get an object given an index previously returned from
    /// internal_find_pack_index.
    ///
    /// # Panics
    ///
    /// If `index` is out of bounds.
    pub fn get_object_by_index<'a>(
        &self,
        idx: u32,
        out: &'a mut Vec<u8>,
        cache: &mut impl crate::cache::DecodeEntry,
    ) -> Result<(git_object::Data<'a>, crate::data::entry::Location), crate::data::decode::Error> {
        let ofs = self.index.pack_offset_at_index(idx);
        let pack_entry = self.pack.entry(ofs);
        let header_size = pack_entry.header_size();
        self.pack
            .decode_entry(
                pack_entry,
                out,
                |id, _out| {
                    self.index.lookup(id).map(|idx| {
                        crate::data::decode::entry::ResolvedBase::InPack(
                            self.pack.entry(self.index.pack_offset_at_index(idx)),
                        )
                    })
                },
                cache,
            )
            .map(move |r| {
                (
                    git_object::Data {
                        kind: r.kind,
                        data: out.as_slice(),
                    },
                    crate::data::entry::Location {
                        pack_id: self.pack.id,
                        pack_offset: ofs,
                        entry_size: r.compressed_size + header_size,
                    },
                )
            })
    }
}
