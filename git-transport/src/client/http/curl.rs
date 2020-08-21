use crate::client::http::{Error, Http};
use curl::easy::Easy2;
use git_features::pipe;
use std::io::Read;

struct Handler;

impl curl::easy::Handler for Handler {}

pub struct CurlHttp {
    _handle: Easy2<Handler>,
}

impl Http for CurlHttp {
    type Response = pipe::Reader;

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
