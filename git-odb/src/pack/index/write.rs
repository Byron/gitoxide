use crate::pack;
use git_features::progress::Progress;
use git_object::owned;
use quick_error::quick_error;
use std::{io, path::Path, time::SystemTime};
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
        Unsupported(kind: pack::index::Kind) {
            display("Indices of type {} cannot be written, only {} are supported", *kind as usize, pack::index::Kind::default() as usize)
        }
    }
}

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Outcome {
    pack_kind: pack::data::Kind,
    pack_hash: owned::Id,

    index_kind: pack::index::Kind,
    index_hash: owned::Id,
    num_objects: u32,
}

/// Various ways of writing an index file from pack entries
impl pack::index::File {
    /// `pack` is pack including the header and all entries, with or without trailer
    pub fn write_to_stream(
        pack: impl io::BufRead,
        _out: impl io::Write,
        mut progress: impl Progress,
        kind: pack::index::Kind,
    ) -> Result<Outcome, Error> {
        if kind != pack::index::Kind::default() {
            return Err(Error::Unsupported(kind));
        }
        let (_kind, num_objects, iter) =
            pack::data::Iter::new_from_header(pack, pack::data::iter::Mode::KeepDecompressedBytes)??;
        if num_objects == 0 {
            return Err(Error::EmptyIndex);
        }

        progress.init(Some(num_objects), Some("objects"));

        let then = SystemTime::now();
        for _entry in iter {
            progress.inc();
        }

        let elapsed = then.elapsed().expect("system time to work").as_secs_f32();
        progress.done(format!(
            "done {} objects in {:.02}s ({}/s)",
            num_objects,
            elapsed,
            num_objects as f32 / elapsed
        ));
        unimplemented!("todo stream");
    }

    pub fn write_to_directory(
        path: impl AsRef<Path>,
        pack: impl io::Read,
        progress: impl Progress,
        kind: pack::index::Kind,
    ) -> Result<Outcome, Error> {
        let path = path.as_ref();

        let mut tempfile = io::BufWriter::with_capacity(4096 * 8, NamedTempFile::new_in(path)?);
        let outcome = Self::write_to_stream(io::BufReader::new(pack), &mut tempfile, progress, kind)?;

        let index_path = path.join(format!("{}.idx", outcome.index_hash.to_sha1_hex_string()));
        tempfile
            .into_inner()
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?
            .persist(index_path)?;
        unimplemented!("pack writing")
    }
}
