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

trait Http {
    type Headers: Iterator<Item = Vec<u8>>;
    type ResponseBody: io::Read;

    fn get(
        &mut self,
        url: &str,
        headers: impl Iterator<Item = impl AsRef<str>>,
    ) -> Result<(Self::Headers, Self::ResponseBody), Error>;
    fn post(
        &mut self,
        url: &str,
        headers: impl Iterator<Item = impl AsRef<str>>,
        body: impl io::Read,
    ) -> Result<(Self::Headers, Self::ResponseBody), Error>;
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
