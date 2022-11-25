use std::{any::Any, convert::TryFrom, io::Write, str::FromStr};

use git_features::io::pipe;

use crate::client::{http, http::reqwest::Remote};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error("Request configuration failed")]
    ConfigureRequest(#[from] Box<dyn std::error::Error + Send + Sync + 'static>),
}

impl Default for Remote {
    fn default() -> Self {
        let (req_send, req_recv) = std::sync::mpsc::sync_channel(0);
        let (res_send, res_recv) = std::sync::mpsc::sync_channel(0);
        let handle = std::thread::spawn(move || -> Result<(), Error> {
            for Request {
                url,
                headers,
                upload,
                config,
            } in req_recv
            {
                // We may error while configuring, which is expected as part of the internal protocol. The error will be
                // received and the sender of the request might restart us.
                let client = reqwest::blocking::ClientBuilder::new()
                    .connect_timeout(std::time::Duration::from_secs(20))
                    .build()?;
                let mut req_builder = if upload { client.post(url) } else { client.get(url) }.headers(headers);
                let (post_body_tx, post_body_rx) = pipe::unidirectional(0);
                if upload {
                    req_builder = req_builder.body(reqwest::blocking::Body::new(post_body_rx));
                }
                let mut req = req_builder.build()?;
                let (mut response_body_tx, response_body_rx) = pipe::unidirectional(0);
                let (mut headers_tx, headers_rx) = pipe::unidirectional(0);
                if res_send
                    .send(Response {
                        headers: headers_rx,
                        body: response_body_rx,
                        upload_body: post_body_tx,
                    })
                    .is_err()
                {
                    // This means our internal protocol is violated as the one who sent the request isn't listening anymore.
                    // Shut down as something is off.
                    break;
                }
                if let Some(ref mut request_options) = config.backend.as_ref().and_then(|backend| backend.lock().ok()) {
                    if let Some(options) = request_options.downcast_mut::<super::Options>() {
                        if let Some(configure_request) = &mut options.configure_request {
                            configure_request(&mut req)?;
                        }
                    }
                }
                let mut res = match client.execute(req).and_then(|res| res.error_for_status()) {
                    Ok(res) => res,
                    Err(err) => {
                        let (kind, err) = match err.status() {
                            Some(status) => {
                                let kind = if status == reqwest::StatusCode::UNAUTHORIZED {
                                    std::io::ErrorKind::PermissionDenied
                                } else {
                                    std::io::ErrorKind::Other
                                };
                                (kind, format!("Received HTTP status {}", status.as_str()))
                            }
                            None => (std::io::ErrorKind::Other, err.to_string()),
                        };
                        let err = Err(std::io::Error::new(kind, err));
                        headers_tx.channel.send(err).ok();
                        continue;
                    }
                };

                let send_headers = {
                    let headers = res.headers();
                    move || -> std::io::Result<()> {
                        for (name, value) in headers {
                            headers_tx.write_all(name.as_str().as_bytes())?;
                            headers_tx.write_all(b":")?;
                            headers_tx.write_all(value.as_bytes())?;
                            headers_tx.write_all(b"\n")?;
                        }
                        // Make sure this is an FnOnce closure to signal the remote reader we are done.
                        drop(headers_tx);
                        Ok(())
                    }
                };

                // We don't have to care if anybody is receiving the header, as a matter of fact we cannot fail sending them.
                // Thus an error means the receiver failed somehow, but might also have decided not to read headers at all. Fine with us.
                send_headers().ok();

                // reading the response body is streaming and may fail for many reasons. If so, we send the error over the response
                // body channel and that's all we can do.
                if let Err(err) = std::io::copy(&mut res, &mut response_body_tx) {
                    response_body_tx.channel.send(Err(err)).ok();
                }
            }
            Ok(())
        });

        Remote {
            handle: Some(handle),
            request: req_send,
            response: res_recv,
            config: http::Options::default(),
        }
    }
}

/// utilities
impl Remote {
    fn make_request(
        &mut self,
        url: &str,
        _base_url: &str,
        headers: impl IntoIterator<Item = impl AsRef<str>>,
        upload: bool,
    ) -> Result<http::PostResponse<pipe::Reader, pipe::Reader, pipe::Writer>, http::Error> {
        let mut header_map = reqwest::header::HeaderMap::new();
        for header_line in headers {
            let header_line = header_line.as_ref();
            let colon_pos = header_line
                .find(':')
                .expect("header line must contain a colon to separate key and value");
            let header_name = &header_line[..colon_pos];
            let value = &header_line[colon_pos + 1..];

            match reqwest::header::HeaderName::from_str(header_name)
                .ok()
                .zip(reqwest::header::HeaderValue::try_from(value.trim()).ok())
            {
                Some((key, val)) => header_map.insert(key, val),
                None => continue,
            };
        }
        self.request
            .send(Request {
                url: url.to_owned(),
                headers: header_map,
                upload,
                config: self.config.clone(),
            })
            .expect("the remote cannot be down at this point");

        let Response {
            headers,
            body,
            upload_body,
        } = match self.response.recv() {
            Ok(res) => res,
            Err(_) => {
                let err = self
                    .handle
                    .take()
                    .expect("always present")
                    .join()
                    .expect("no panic")
                    .expect_err("no receiver means thread is down with init error");
                *self = Self::default();
                return Err(http::Error::InitHttpClient { source: Box::new(err) });
            }
        };

        Ok(http::PostResponse {
            post_body: upload_body,
            headers,
            body,
        })
    }
}

impl http::Http for Remote {
    type Headers = pipe::Reader;
    type ResponseBody = pipe::Reader;
    type PostBody = pipe::Writer;

    fn get(
        &mut self,
        url: &str,
        base_url: &str,
        headers: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Result<http::GetResponse<Self::Headers, Self::ResponseBody>, http::Error> {
        self.make_request(url, base_url, headers, false).map(Into::into)
    }

    fn post(
        &mut self,
        url: &str,
        base_url: &str,
        headers: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Result<http::PostResponse<Self::Headers, Self::ResponseBody, Self::PostBody>, http::Error> {
        self.make_request(url, base_url, headers, true)
    }

    fn configure(&mut self, config: &dyn Any) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        if let Some(config) = config.downcast_ref::<http::Options>() {
            self.config = config.clone();
        }
        Ok(())
    }
}

pub(crate) struct Request {
    pub url: String,
    pub headers: reqwest::header::HeaderMap,
    pub upload: bool,
    pub config: http::Options,
}

/// A link to a thread who provides data for the contained readers.
/// The expected order is:
/// - write `upload_body`
/// - read `headers` to end
/// - read `body` to hend
pub(crate) struct Response {
    pub headers: pipe::Reader,
    pub body: pipe::Reader,
    pub upload_body: pipe::Writer,
}
