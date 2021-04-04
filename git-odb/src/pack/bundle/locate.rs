use crate::pack;
use git_object::borrowed;

impl pack::Bundle {
    /// Find an object with the given [`id`][borrowed::Id] and place its data into `out`.
    ///
    /// [`cache`][pack::cache::DecodeEntry] is used to accelerate the lookup.
    ///
    /// **Note** that ref deltas are automatically resolved within this pack only, which makes this implementation unusable
    /// for thin packs, which by now are expected to be resolved already.
    pub fn locate<'a>(
        &self,
        id: borrowed::Id<'_>,
        out: &'a mut Vec<u8>,
        cache: &mut impl pack::cache::DecodeEntry,
    ) -> Result<Option<crate::borrowed::Object<'a>>, pack::data::decode::Error> {
        let idx = match self.index.lookup(id) {
            Some(idx) => idx,
            None => return Ok(None),
        };
        let ofs = self.index.pack_offset_at_index(idx);
        let pack_entry = self.pack.entry(ofs);
        self.pack
            .decode_entry(
                pack_entry,
                out,
                |id, _out| {
                    self.index.lookup(id).map(|idx| {
                        pack::data::decode::ResolvedBase::InPack(self.pack.entry(self.index.pack_offset_at_index(idx)))
                    })
                },
                cache,
            )
            .map(move |r| {
                Some(crate::borrowed::Object {
                    kind: r.kind,
                    data: out.as_slice(),
                })
            })
    }
}
