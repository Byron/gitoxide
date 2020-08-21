use crate::client::http::{Error, Http};
use curl::easy::Easy2;
use git_features::pipe;
use std::io::Read;

#[derive(Default)]
struct Handler {
    send_header: Option<std::sync::mpsc::SyncSender<Vec<u8>>>,
    send_data: Option<pipe::Writer>,
}

impl curl::easy::Handler for Handler {}

pub struct CurlHttp {
    handle: Easy2<Handler>,
}

impl CurlHttp {
    pub fn new() -> Self {
        CurlHttp {
            handle: Easy2::new(Handler::default()),
        }
    }
}

impl From<curl::Error> for Error {
    fn from(err: curl::Error) -> Self {
        Error::Detail(err.to_string())
    }
}

struct Joiner<'a> {
    parent: &'a mut CurlHttp,
}

impl<'a> crate::client::http::Joiner for Joiner<'a> {
    fn join(self) -> Result<(), Error> {
        unimplemented!("join on joiner")
    }
}

impl Http for CurlHttp {
    type Headers = pipe::Iter<Vec<u8>>;
    type ResponseBody = pipe::Reader;
    type Handle = Joiner<'static>;

    fn get(
        &mut self,
        url: &str,
        headers: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Result<(Self::Handle, Self::Headers, Self::ResponseBody), Error> {
        self.handle.url(url)?;
        let mut list = curl::easy::List::new();
        for header in headers {
            list.append(header.as_ref())?;
        }
        self.handle.http_headers(list)?;

        let (send, receive_data) = pipe::unidirectional(1);
        self.handle.get_mut().send_data = Some(send);
        let (send, receive_headers) = pipe::iter(1);
        self.handle.get_mut().send_header = Some(send);

        Ok((Joiner { parent: self }, receive_headers, receive_data))
    }

    fn post(
        &mut self,
        _url: &str,
        _headers: impl IntoIterator<Item = impl AsRef<str>>,
        _body: impl Read,
    ) -> Result<(Self::Handle, Self::Headers, Self::ResponseBody), Error> {
        unimplemented!()
    }
}
