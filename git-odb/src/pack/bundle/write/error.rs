use crate::pack;
use quick_error::quick_error;
use std::io;

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
        IndexWrite(err: pack::index::write::Error) {
            display("The index file could not be written")
            from()
            source(err)
        }
    }
}
