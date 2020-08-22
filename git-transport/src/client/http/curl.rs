use crate::client::http;
use curl::easy::Easy2;
use git_features::pipe;
use std::io::Read;

#[derive(Default)]
struct Handler {
    send_header: Option<pipe::Writer>,
    send_data: Option<pipe::Writer>,
}

impl curl::easy::Handler for Handler {}

pub struct Curl {
    handle: Easy2<Handler>,
}

impl Curl {
    pub fn new() -> Self {
        Curl {
            handle: Easy2::new(Handler::default()),
        }
    }
}

impl From<curl::Error> for http::Error {
    fn from(err: curl::Error) -> Self {
        http::Error::Detail(err.to_string())
    }
}

impl crate::client::http::Http for Curl {
    type Headers = pipe::Reader;
    type ResponseBody = pipe::Reader;

    fn get(
        &mut self,
        url: &str,
        headers: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Result<(Self::Headers, Self::ResponseBody), http::Error> {
        self.handle.url(url)?;
        let mut list = curl::easy::List::new();
        for header in headers {
            list.append(header.as_ref())?;
        }
        self.handle.http_headers(list)?;

        let (send, receive_data) = pipe::unidirectional(1);
        self.handle.get_mut().send_data = Some(send);
        let (send, receive_headers) = pipe::unidirectional(1);
        self.handle.get_mut().send_header = Some(send);

        Ok((receive_headers, receive_data))
    }

    fn post(
        &mut self,
        _url: &str,
        _headers: impl IntoIterator<Item = impl AsRef<str>>,
        _body: impl Read,
    ) -> Result<(Self::Headers, Self::ResponseBody), http::Error> {
        unimplemented!()
    }
}
