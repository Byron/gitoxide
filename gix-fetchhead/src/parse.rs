use std::{path::Path, fs::File, io::Read};

use crate::FetchHead;

//TODO: should probably take in a repository object and find the FETCH_HEAD file from there.
pub fn parse(path: impl AsRef<Path>) -> Result<FetchHead, std::io::Error> {
    let mut source = match File::open(&path) {
        Ok(it) => it,
        Err(err) => return Err(err),
    };
    let mut contents = String::new();
    if source.read_to_string(&mut contents).is_err() {
         panic!("couldn't read open file at path");
    }

    todo!();
}
