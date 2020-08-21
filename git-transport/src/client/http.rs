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
pub(crate) mod pipe {
    use std::io;

    pub struct PipeWriter;
    pub struct PipeReader;

    impl io::Read for PipeReader {
        fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
            unimplemented!()
        }
    }

    impl io::Write for PipeWriter {
        fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
            unimplemented!()
        }

        fn flush(&mut self) -> io::Result<()> {
            unimplemented!()
        }
    }

    pub fn _unidirectional() -> (PipeWriter, PipeReader) {
        unimplemented!("unidirectional pipe")
    }
}

#[cfg(feature = "http-client-curl")]
pub(crate) mod curl {
    use crate::client::http::{pipe::PipeReader, Error, Http};
    use curl::easy::Easy2;
    use std::io::Read;

    struct Handler;

    impl curl::easy::Handler for Handler {}

    pub struct CurlHttp {
        _handle: Easy2<Handler>,
    }

    impl Http for CurlHttp {
        type Response = PipeReader;

        fn get(_url: &str, _headers: impl Iterator<Item = impl AsRef<str>>) -> Result<Self::Response, Error> {
            unimplemented!()
        }

        fn post(
            _url: &str,
            _headers: impl Iterator<Item = impl AsRef<str>>,
            _body: impl Read,
        ) -> Result<Self::Response, Error> {
            unimplemented!()
        }
    }
}

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
