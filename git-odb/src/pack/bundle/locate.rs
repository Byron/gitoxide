use crate::pack;
use git_object::borrowed;
use quick_error::quick_error;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Decode(err: pack::data::decode::Error) {
            display("Could not decode object")
        }
    }
}

impl pack::Bundle {
    /// `id` is a 20 byte SHA1 of the object to locate in the pack
    ///
    /// Note that ref deltas are automatically resolved within this pack only, which makes this implementation unusable
    /// for thin packs.
    /// For the latter, pack streams are required.
    pub fn locate<'a>(
        &self,
        id: borrowed::Id,
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
