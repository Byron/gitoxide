use crate::pack;
use git_features::progress::Progress;
use quick_error::quick_error;
use std::{io, time::SystemTime};

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Io(err: io::Error) {
            display("An IO error occurred when reading the pack")
            from()
            source(err)
        }
        HeaderDecode(err: pack::data::parse::Error) {
            display("The pack header could not be parsed when starting to write the index")
            from()
            source(err)
        }
        EmptyIndex {
            display("Empty indices are not allowed - at least one pack entry is required")
        }
    }
}

/// Various ways of writing an index file from pack entries
impl pack::index::File {
    /// `pack` is pack including the header and all entries, with or without trailer
    pub fn write_to_stream(
        pack: impl io::BufRead,
        _out: impl io::Write,
        mut progress: impl Progress,
    ) -> Result<(), Error> {
        let (_kind, num_objects, iter) =
            pack::data::Iter::new_from_header(pack, pack::data::iter::Mode::KeepDecompressedBytes)??;
        if num_objects == 0 {
            return Err(Error::EmptyIndex);
        }

        progress.init(Some(num_objects), Some("objects"));

        let then = SystemTime::now();
        for entry in iter {
            progress.inc();
        }

        let elapsed = then.elapsed().expect("system time to work").as_secs_f32();
        progress.done(format!(
            "done {} objects in {:.02}s ({}/s)",
            num_objects,
            elapsed,
            num_objects as f32 / elapsed
        ));
        Ok(())
    }
}
