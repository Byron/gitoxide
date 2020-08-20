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
    _host: &str,
    _path: &Path,
    _version: crate::Protocol,
    _user: Option<&str>,
    _port: Option<u16>,
) -> Result<git::Connection<process::ChildStdout, process::ChildStdin>, Error> {
    unimplemented!("file connection")
}
