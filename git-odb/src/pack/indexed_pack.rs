use crate::pack;
use quick_error::quick_error;
use std::path::{Path, PathBuf};

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        InvalidPath(path: PathBuf) {
            display("An 'idx' extension is expected of an index file: '{}'", path.display())
        }
        Pack(err: pack::Error) {
            display("Could not instantiate pack")
            from()
            cause(err)
        }
        Index(err: pack::index::Error) {
            display("Could not instantiate pack index")
            from()
            cause(err)
        }
        Decode(err: pack::Error) {
            display("Could not decode object")
        }
    }
}

/// A packfile with an index
pub struct Bundle {
    pack: pack::File,
    index: pack::index::File,
}

impl Bundle {
    pub fn at(path: impl AsRef<Path>) -> Result<Self, Error> {
        let path = path.as_ref();
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .ok_or_else(|| Error::InvalidPath(path.to_owned()))?;
        Ok(match ext {
            "idx" => Self {
                index: pack::index::File::at(path)?,
                pack: pack::File::at(path.with_extension("pack"))?,
            },
            "pack" => Self {
                pack: pack::File::at(path)?,
                index: pack::index::File::at(path.with_extension("idx"))?,
            },
            _ => return Err(Error::InvalidPath(path.to_owned())),
        })
    }

    /// `id` is a 20 byte SHA1 of the object to locate in the pack
    pub fn locate<'a>(
        &self,
        id: &[u8],
        out: &'a mut Vec<u8>,
        cache: &mut impl pack::cache::DecodeEntry,
    ) -> Option<Result<Object<'a>, Error>> {
        let idx = self.index.lookup_index(id)?;
        let ofs = self.index.pack_offset_at_index(idx);
        let entry = self.pack.entry(ofs);
        Some(
            self.pack
                .decode_entry(entry, out, |_id, _out| None, cache)
                .map_err(Error::Decode)
                .map(move |_| Object { dummy: out.as_slice() }),
        )
    }
}

pub struct Object<'data> {
    pub dummy: &'data [u8],
}
