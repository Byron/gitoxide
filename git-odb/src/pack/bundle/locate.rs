use crate::pack;
use git_object::borrowed;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Decode(pack::data::decode::Error),
}

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
    ) -> Option<Result<crate::borrowed::Object<'a>, Error>> {
        let idx = self.index.lookup(id)?;
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
            .map_err(Error::Decode)
            .map(move |r| crate::borrowed::Object {
                kind: r.kind,
                data: out.as_slice(),
            })
            .into()
    }
}
