use quick_error::quick_error;
use std::path::Path;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Tbd {
            display("tbd")
        }
    }
}

pub struct Transport {}

impl crate::client::Transport for Transport {}

pub fn connect(
    _host: &str,
    _path: &Path,
    _version: crate::Protocol,
    _user: Option<&str>,
    _port: Option<u16>,
    _secure: bool,
) -> Result<Transport, Error> {
    unimplemented!("file connection")
}
