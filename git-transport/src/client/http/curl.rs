use crate::client::http;
use curl::easy::Easy2;
use git_features::pipe;
use std::io::Read;

#[derive(Default)]
struct Handler {
    send_header: Option<std::sync::mpsc::SyncSender<Vec<u8>>>,
    send_data: Option<pipe::Writer>,
}

impl curl::easy::Handler for Handler {}

pub struct Http {
    handle: Easy2<Handler>,
}

impl Http {
    pub fn new() -> Self {
        Http {
            handle: Easy2::new(Handler::default()),
        }
    }
}

impl From<curl::Error> for http::Error {
    fn from(err: curl::Error) -> Self {
        http::Error::Detail(err.to_string())
    }
}

pub struct Joiner {
    result: std::sync::mpsc::Receiver<Result<(), http::Error>>,
}

impl crate::client::http::Joiner for Joiner {
    fn join(self) -> Result<(), http::Error> {
        self.result
            .recv()
            .map_err(|_| http::Error::Detail("receive on closed channel, must be a bug".into()))?
    }
}

impl crate::client::http::Http for Http {
    type Headers = pipe::Iter<Vec<u8>>;
    type ResponseBody = pipe::Reader;
    type Handle = Joiner;

    fn get(
        &mut self,
        url: &str,
        headers: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Result<(Self::Handle, Self::Headers, Self::ResponseBody), http::Error> {
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
        let (_send, recv_result) = std::sync::mpsc::sync_channel(0); // TODO: must be static in the remote handle

        Ok((Joiner { result: recv_result }, receive_headers, receive_data))
    }

    fn post(
        &mut self,
        _url: &str,
        _headers: impl IntoIterator<Item = impl AsRef<str>>,
        _body: impl Read,
    ) -> Result<(Self::Handle, Self::Headers, Self::ResponseBody), http::Error> {
        unimplemented!()
    }
}
