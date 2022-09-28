pub struct Remote {
    /// A worker thread which performs the actual request.
    handle: Option<std::thread::JoinHandle<Result<(), reqwest::Error>>>,
    /// A channel to send requests (work) to the worker thread.
    request: std::sync::mpsc::SyncSender<remote::Request>,
    /// A channel to receive the result of the prior request.
    response: std::sync::mpsc::Receiver<remote::Response>,
}

mod remote {
    use crate::client::http;
    use crate::client::http::reqwest::Remote;
    use git_features::io::pipe;
    use std::convert::TryFrom;
    use std::io::Write;
    use std::str::FromStr;

    impl Default for Remote {
        fn default() -> Self {
            let (req_send, req_recv) = std::sync::mpsc::sync_channel(0);
            let (res_send, res_recv) = std::sync::mpsc::sync_channel(0);
            let handle = std::thread::spawn(move || -> Result<(), reqwest::Error> {
                for Request { url, headers, upload } in req_recv {
                    // We may error while configuring, which is expected as part of the internal protocol. The error will be
                    // received and the sender of the request might restart us.
                    let client = reqwest::blocking::ClientBuilder::new()
                        .connect_timeout(std::time::Duration::from_secs(20))
                        .build()?;
                    let mut req = if upload { client.post(url) } else { client.get(url) }.headers(headers);
                    let (post_body_tx, post_body_rx) = pipe::unidirectional(0);
                    if upload {
                        req = req.body(reqwest::blocking::Body::new(post_body_rx));
                    }
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
                    let mut res = match req.send() {
                        Ok(res) => res,
                        Err(err) => {
                            let err = Err(std::io::Error::new(std::io::ErrorKind::Other, err));
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
            }
        }
    }

    /// utilities
    impl Remote {
        fn make_request(
            &mut self,
            url: &str,
            headers: impl IntoIterator<Item = impl AsRef<str>>,
            upload: bool,
        ) -> Result<http::PostResponse<pipe::Reader, pipe::Reader, pipe::Writer>, http::Error> {
            let mut header_map = reqwest::header::HeaderMap::new();
            for header_line in headers {
                let header_line = header_line.as_ref();
                let colon_pos = header_line
                    .find(':')
                    .expect("header line must contain a colon to separate key and value");
                let (header_name, value) = header_line.split_at(colon_pos);

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
            headers: impl IntoIterator<Item = impl AsRef<str>>,
        ) -> Result<http::GetResponse<Self::Headers, Self::ResponseBody>, http::Error> {
            self.make_request(url, headers, false).map(Into::into)
        }

        fn post(
            &mut self,
            url: &str,
            headers: impl IntoIterator<Item = impl AsRef<str>>,
        ) -> Result<http::PostResponse<Self::Headers, Self::ResponseBody, Self::PostBody>, http::Error> {
            self.make_request(url, headers, true)
        }
    }

    pub struct Request {
        pub url: String,
        pub headers: reqwest::header::HeaderMap,
        pub upload: bool,
    }

    /// A link to a thread who provides data for the contained readers.
    /// The expected order is:
    /// - write `upload_body`
    /// - read `headers` to end
    /// - read `body` to hend
    pub struct Response {
        pub headers: pipe::Reader,
        pub body: pipe::Reader,
        pub upload_body: pipe::Writer,
    }
}
