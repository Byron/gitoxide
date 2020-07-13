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
    pub fn locate<'data>(
        &self,
        id: &[u8],
        out: &'data mut Vec<u8>,
        cache: impl pack::cache::DecodeEntry,
    ) -> Option<Result<Object<'data>, Error>> {
        unimplemented!("locate object")
    }
}

pub struct Object<'data> {
    dummy: &'data [u8],
}
