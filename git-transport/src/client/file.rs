use crate::client::git;
use quick_error::quick_error;
use std::{path::Path, process};

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Tbd {
            display("tbd")
        }
    }
}

pub fn connect(
    _path: &Path,
    _version: crate::Protocol,
) -> Result<git::Connection<process::ChildStdout, process::ChildStdin>, Error> {
    unimplemented!("file connection")
}
