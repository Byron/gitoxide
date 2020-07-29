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
