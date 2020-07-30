use crate::{pack, pack::data::iter::Mode};
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
        PackIter(err: pack::data::iter::Error) {
            display("Pack iteration failed")
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

        let iter_with_thinpack_resolver_tbd =
            pack::data::Iter::new_from_header(io::BufReader::new(pack), Mode::Verify)?;
        if iter_with_thinpack_resolver_tbd.len() == 0 {
            return Err(Error::EmptyIndex);
        }

        let mut tempfile = io::BufWriter::with_capacity(4096 * 8, NamedTempFile::new_in(path)?);
        let outcome = pack::index::File::write_to_stream(iter_with_thinpack_resolver_tbd, &mut tempfile, kind)?;

        let index_path = path.join(format!("{}.idx", outcome.index_hash.to_sha1_hex_string()));
        tempfile
            .into_inner()
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?
            .persist(index_path)?;
        // Consider thin packs resolution in data pack itself as Iterator - input iter::Entry, output resolved Entries
        // These can then be written to an output stream (Write) which can also be in the data pack.
        // This method just coordinates the pieces
        unimplemented!("pack writing and thin pack resolution")
    }
}
