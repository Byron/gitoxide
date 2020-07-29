use crate::pack;
use git_object::borrowed;
use quick_error::quick_error;
use std::{
    convert::TryFrom,
    path::{Path, PathBuf},
};

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        InvalidPath(path: PathBuf) {
            display("An 'idx' extension is expected of an index file: '{}'", path.display())
        }
        Pack(err: pack::data::parse::Error) {
            display("Could not instantiate pack")
            from()
            source(err)
        }
        Index(err: pack::index::init::Error) {
            display("Could not instantiate pack index")
            from()
            source(err)
        }
        Decode(err: pack::data::decode::Error) {
            display("Could not decode object")
        }
    }
}

/// A packfile with an index
pub struct Bundle {
    pub pack: pack::data::File,
    pub index: pack::index::File,
}

pub mod write {
    use crate::pack;
    use git_features::progress::Progress;
    use git_object::owned;
    use quick_error::quick_error;
    use std::{io, path::Path};
    use tempfile::NamedTempFile;

    quick_error! {
        #[derive(Debug)]
        pub enum Error {
            Io(err: io::Error) {
                display("An IO error occurred when reading the pack or creating a temporary file")
                from()
                source(err)
            }
            HeaderDecode(err: pack::data::parse::Error) {
                display("The pack header could not be parsed when starting to write the index")
                from()
                source(err)
            }
            PeristError(err: tempfile::PersistError) {
                display("Could not move a temporary file into its desired place")
                from()
                source(err)
            }
            EmptyIndex {
                display("Empty indices are not allowed - at least one pack entry is required")
            }
            IndexWrite(err: pack::index::write::Error) {
                display("The index file could not be written")
                from()
                source(err)
            }
        }
    }

    #[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
    #[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
    pub struct Outcome {
        index_kind: pack::index::Kind,
        index_hash: owned::Id,
        num_objects: u32,

        pack_kind: pack::data::Kind,
        pack_hash: owned::Id,
    }

    impl pack::Bundle {
        pub fn write_to_directory(
            path: impl AsRef<Path>,
            pack: impl io::Read,
            _progress: impl Progress,
            kind: pack::index::Kind,
        ) -> Result<pack::index::write::Outcome, Error> {
            let path = path.as_ref();

            let (_kind, num_objects, iter) = pack::data::Iter::new_from_header(
                io::BufReader::new(pack),
                pack::data::iter::Mode::KeepDecompressedBytes,
            )??;
            if num_objects == 0 {
                return Err(Error::EmptyIndex);
            }

            let mut tempfile = io::BufWriter::with_capacity(4096 * 8, NamedTempFile::new_in(path)?);
            let outcome = pack::index::File::write_to_stream(
                iter.map(|e| {
                    e.map(|e| pack::index::write::Entry {
                        header: e.header,
                        header_size: e.header_size,
                        pack_offset: e.pack_offset,
                        bytes: vec![], // TODO
                        decompressed: e.decompressed.expect("iteration while keeping decompression result"),
                    })
                }),
                &mut tempfile,
                kind,
            )?;

            let index_path = path.join(format!("{}.idx", outcome.index_hash.to_sha1_hex_string()));
            tempfile
                .into_inner()
                .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?
                .persist(index_path)?;
            unimplemented!("pack writing and resolution")
        }
    }
}

impl Bundle {
    /// `path` is either a pack file or an index file
    pub fn at(path: impl AsRef<Path>) -> Result<Self, Error> {
        Self::try_from(path.as_ref())
    }

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
    ) -> Option<Result<pack::Object<'a>, Error>> {
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
            .map(move |r| pack::Object {
                kind: r.kind,
                data: out.as_slice(),
            })
            .into()
    }
}

impl TryFrom<&Path> for Bundle {
    type Error = Error;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .ok_or_else(|| Error::InvalidPath(path.to_owned()))?;
        Ok(match ext {
            "idx" => Self {
                index: pack::index::File::at(path)?,
                pack: pack::data::File::at(path.with_extension("pack"))?,
            },
            "pack" => Self {
                pack: pack::data::File::at(path)?,
                index: pack::index::File::at(path.with_extension("idx"))?,
            },
            _ => return Err(Error::InvalidPath(path.to_owned())),
        })
    }
}
