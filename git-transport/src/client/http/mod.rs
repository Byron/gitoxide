use quick_error::quick_error;
use std::{io, path::Path};

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Tbd {
            display("tbd")
        }
    }
}
#[cfg(feature = "http-client-curl")]
pub(crate) mod curl;
#[cfg(feature = "http-client-curl")]
pub(crate) mod pipe;

trait Http {
    type Response: io::Read;

    fn get(url: &str, headers: impl Iterator<Item = impl AsRef<str>>) -> Result<Self::Response, Error>;
    fn post(
        url: &str,
        headers: impl Iterator<Item = impl AsRef<str>>,
        body: impl io::Read,
    ) -> Result<Self::Response, Error>;
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
