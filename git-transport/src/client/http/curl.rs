use crate::client::http;
use curl::easy::Easy2;
use git_features::pipe;
use std::{
    io,
    io::Read,
    sync::mpsc::{sync_channel, Receiver, SyncSender, TrySendError},
};

#[derive(Default)]
struct Handler {
    send_header: Option<pipe::Writer>,
    send_data: Option<pipe::Writer>,
    receive_body: Option<pipe::Reader>,
}

impl curl::easy::Handler for Handler {}

pub struct Curl {
    handle: Easy2<Handler>,
}

struct Request {
    url: String,
    headers: curl::easy::List,
}

struct Response {
    headers: pipe::Reader,
    body: pipe::Reader,
    upload_body: pipe::Writer,
}

fn new_remote_curl() -> (
    std::thread::JoinHandle<Result<(), curl::Error>>,
    SyncSender<Request>,
    Receiver<Response>,
) {
    let (req_send, req_recv) = sync_channel(0);
    let (res_send, res_recv) = sync_channel(0);
    let handle = std::thread::spawn(move || -> Result<(), curl::Error> {
        let mut handle = Easy2::new(Handler::default());
        for Request { url, headers } in req_recv {
            handle.url(&url)?;
            handle.http_headers(headers)?;

            let (receive_data, receive_headers, send_body) = {
                let handler = handle.get_mut();
                let (send, receive_data) = pipe::unidirectional(1);
                handler.send_data = Some(send);
                let (send, receive_headers) = pipe::unidirectional(1);
                handler.send_header = Some(send);
                let (send_body, receive_body) = pipe::unidirectional(None);
                handler.receive_body = Some(receive_body);
                (receive_data, receive_headers, send_body)
            };

            if let Err(err) = handle.perform() {
                let handler = handle.get_mut();
                let err = Err(io::Error::new(io::ErrorKind::Other, err));
                handler.receive_body.take();
                match (handler.send_header.take(), handler.send_data.take()) {
                    (Some(header), mut data) => {
                        if let Err(TrySendError::Disconnected(err)) | Err(TrySendError::Full(err)) =
                            header.channel.try_send(err)
                        {
                            if let Some(body) = data.take() {
                                body.channel.try_send(err).ok();
                            }
                        }
                    }
                    (None, Some(body)) => {
                        body.channel.try_send(err).ok();
                    }
                    (None, None) => {}
                };
            } else {
                let handler = handle.get_mut();
                handler.receive_body.take();
                handler.send_header.take();
                handler.send_data.take();
            }

            if res_send
                .send(Response {
                    headers: receive_headers,
                    body: receive_data,
                    upload_body: send_body,
                })
                .is_err()
            {
                break;
            }
        }
        Ok(())
    });
    (handle, req_send, res_recv)
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
