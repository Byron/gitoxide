use crate::client::http::{Error, Http};
use curl::easy::Easy2;
use git_features::pipe;
use std::io::Read;

struct Handler;

impl curl::easy::Handler for Handler {}

pub struct CurlHttp {
    handle: Option<Easy2<Handler>>,
}

impl CurlHttp {
    pub fn new() -> Self {
        CurlHttp {
            handle: Some(Easy2::new(Handler)),
        }
    }
}

impl Http for CurlHttp {
    type Headers = pipe::Iter<Vec<u8>>;
    type ResponseBody = pipe::Reader;

    fn get(
        &mut self,
        _url: &str,
        _headers: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Result<(Self::Headers, Self::ResponseBody), Error> {
        unimplemented!()
    }

    fn post(
        &mut self,
        _url: &str,
        _headers: impl IntoIterator<Item = impl AsRef<str>>,
        _body: impl Read,
    ) -> Result<(Self::Headers, Self::ResponseBody), Error> {
        unimplemented!()
    }
}
