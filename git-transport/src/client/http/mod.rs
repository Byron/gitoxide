use crate::{
    client::{self, capabilities, ExtendedBufRead, HandleProgress, RequestWriter},
    Protocol, Service,
};
use git_packetline::PacketLine;
use std::{
    borrow::Cow,
    convert::Infallible,
    io::{self, BufRead, Read},
};

#[cfg(feature = "http-client-curl")]
pub(crate) mod curl;

mod traits;
pub use traits::{Error, GetResponse, Http, PostResponse};

#[cfg(feature = "http-client-curl")]
pub type Impl = curl::Curl;

pub struct Transport<H: Http> {
    url: String,
    user_agent_header: &'static str,
    desired_version: crate::Protocol,
    actual_version: crate::Protocol,
    http: H,
    service: Option<Service>,
    line_provider: Option<git_packetline::Provider<H::ResponseBody>>,
    identity: Option<client::Identity>,
}

impl Transport<Impl> {
    pub fn new(url: &str, version: crate::Protocol) -> Self {
        Transport {
            url: url.to_owned(),
            user_agent_header: concat!("User-Agent: git/oxide-", env!("CARGO_PKG_VERSION")),
            desired_version: version,
            actual_version: version,
            service: None,
            http: Impl::default(),
            line_provider: None,
            identity: None,
        }
    }
}

impl<H: Http> Transport<H> {
    fn check_content_type(service: Service, kind: &str, headers: <H as Http>::Headers) -> Result<(), client::Error> {
        let wanted_content_type = format!("Content-Type: application/x-{}-{}", service.as_str(), kind);
        if !headers
            .lines()
            .collect::<Result<Vec<_>, _>>()?
            .iter()
            .any(|l| l == &wanted_content_type)
        {
            return Err(client::Error::Http(Error::Detail(format!(
                "Didn't find '{}' header to indicate 'smart' protocol, and 'dumb' protocol is not supported.",
                wanted_content_type
            ))));
        }
        Ok(())
    }

    fn add_basic_auth_if_present(&self, headers: &mut Vec<Cow<str>>) -> Result<(), client::Error> {
        if let Some(identity) = &self.identity {
            match identity {
                client::Identity::Account { username, password } => {
                    #[cfg(not(debug_assertions))]
                    if self.url.starts_with("http://") {
                        return Err(client::Error::AuthenticationRefused(
                            "Will not send credentials in clear text over http",
                        ));
                    }
                    headers.push(Cow::Owned(format!(
                        "Authorization: Basic {}",
                        base64::encode(format!("{}:{}", username, password))
                    )))
                }
            }
        }
        Ok(())
    }
}

fn append_url(base: &str, suffix: &str) -> String {
    if base.ends_with('/') {
        format!("{}{}", base, suffix)
    } else {
        format!("{}/{}", base, suffix)
    }
}

impl<H: Http> client::Transport for Transport<H> {
    fn handshake(&mut self, service: Service) -> Result<client::SetServiceResponse, client::Error> {
        let url = append_url(&self.url, &format!("info/refs?service={}", service.as_str()));
        let static_headers = [Cow::Borrowed(self.user_agent_header)];
        let mut dynamic_headers = Vec::<Cow<str>>::new();
        if self.desired_version != Protocol::V1 {
            dynamic_headers.push(Cow::Owned(format!(
                "Git-Protocol: version={}",
                self.desired_version as usize
            )));
        }
        self.add_basic_auth_if_present(&mut dynamic_headers)?;
        let GetResponse { headers, body } = self.http.get(&url, static_headers.iter().chain(&dynamic_headers))?;
        <Transport<H>>::check_content_type(service, "advertisement", headers)?;

        let line_reader = self
            .line_provider
            .get_or_insert_with(|| git_packetline::Provider::new(body, PacketLine::Flush));

        let mut announced_service = String::new();
        line_reader.as_read().read_to_string(&mut announced_service)?;
        let expected_service_announcement = format!("# service={}", service.as_str());
        if announced_service.trim() != expected_service_announcement {
            return Err(client::Error::Http(Error::Detail(format!(
                "Expected to see {:?}, but got {:?}",
                expected_service_announcement,
                announced_service.trim()
            ))));
        }

        let capabilities::recv::Outcome {
            capabilities,
            refs,
            protocol: actual_protocol,
        } = capabilities::recv::v1_or_v2_as_detected(line_reader)?;
        self.actual_version = actual_protocol;
        self.service = Some(service);
        Ok(client::SetServiceResponse {
            actual_protocol,
            capabilities,
            refs,
        })
    }

    fn set_identity(&mut self, identity: client::Identity) -> Result<(), client::Error> {
        self.identity = Some(identity);
        Ok(())
    }

    fn request(
        &mut self,
        write_mode: client::WriteMode,
        on_into_read: client::MessageKind,
    ) -> Result<client::RequestWriter, client::Error> {
        let service = self.service.expect("handshake() must have been called first");
        let url = append_url(&self.url, service.as_str());
        let static_headers = &[
            Cow::Borrowed(self.user_agent_header),
            Cow::Owned(format!("Content-Type: application/x-{}-request", service.as_str())),
            format!("Accept: application/x-{}-result", service.as_str()).into(),
            "Expect:".into(), // needed to avoid sending Expect: 100-continue, which adds another response and only CURL wants that
        ];
        let mut dynamic_headers = Vec::new();
        self.add_basic_auth_if_present(&mut dynamic_headers)?;
        if self.actual_version != Protocol::V1 {
            dynamic_headers.push(Cow::Owned(format!(
                "Git-Protocol: version={}",
                self.actual_version as usize
            )));
        }

        let PostResponse {
            headers,
            body,
            post_body,
        } = self.http.post(&url, static_headers.iter().chain(&dynamic_headers))?;
        let line_provider = self
            .line_provider
            .as_mut()
            .expect("handshake to have been called first");
        line_provider.replace(body);
        Ok(RequestWriter::new_from_bufread(
            post_body,
            Box::new(HeadersThenBody::<H, _> {
                service,
                headers: Some(headers),
                body: line_provider.as_read_without_sidebands(),
            }),
            write_mode,
            on_into_read,
        ))
    }

    fn close(&mut self) -> Result<(), client::Error> {
        Ok(())
    }

    fn to_url(&self) -> String {
        self.url.to_owned()
    }

    fn desired_protocol_version(&self) -> Protocol {
        self.desired_version
    }

    fn is_stateful(&self) -> bool {
        false
    }
}

struct HeadersThenBody<H: Http, B> {
    service: Service,
    headers: Option<H::Headers>,
    body: B,
}

impl<H: Http, B> HeadersThenBody<H, B> {
    fn handle_headers(&mut self) -> io::Result<()> {
        if let Some(headers) = self.headers.take() {
            <Transport<H>>::check_content_type(self.service, "result", headers)
                .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?
        }
        Ok(())
    }
}

impl<H: Http, B: ExtendedBufRead> io::Read for HeadersThenBody<H, B> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.handle_headers()?;
        self.body.read(buf)
    }
}

impl<H: Http, B: ExtendedBufRead> io::BufRead for HeadersThenBody<H, B> {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        self.handle_headers()?;
        self.body.fill_buf()
    }

    fn consume(&mut self, amt: usize) {
        self.body.consume(amt)
    }
}

impl<H: Http, B: ExtendedBufRead> ExtendedBufRead for HeadersThenBody<H, B> {
    fn set_progress_handler(&mut self, handle_progress: Option<HandleProgress>) {
        self.body.set_progress_handler(handle_progress)
    }

    fn peek_data_line(&mut self) -> Option<io::Result<Result<&[u8], git_packetline::decode::Error>>> {
        self.body.peek_data_line()
    }
}

pub fn connect(url: &str, version: crate::Protocol) -> Result<Transport<Impl>, Infallible> {
    Ok(Transport::new(url, version))
}
