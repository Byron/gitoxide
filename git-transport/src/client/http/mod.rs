use crate::client::SetServiceResponse;
use crate::{Protocol, Service};
use quick_error::quick_error;
use std::{borrow::Cow, convert::Infallible, io};

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Detail(description: String) {
            display("{}", description)
        }
        Status(status: u16, method: &'static str, url: String) {
            display("{}:{} saw HTTP status {}", method, url, status)
        }
    }
}

#[cfg(feature = "http-client-curl")]
pub(crate) mod curl;

#[must_use = "`join()` should be called to handle error conditions."]
trait Joiner {
    fn join(self) -> Result<(), Error>;
}

trait Http {
    type Headers: Iterator<Item = Vec<u8>>;
    type ResponseBody: io::Read;
    type Handle: Joiner;

    fn get(
        &mut self,
        url: &str,
        headers: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Result<(Self::Handle, Self::Headers, Self::ResponseBody), Error>;
    fn post(
        &mut self,
        url: &str,
        headers: impl IntoIterator<Item = impl AsRef<str>>,
        body: impl io::Read,
    ) -> Result<(Self::Handle, Self::Headers, Self::ResponseBody), Error>;
}

#[cfg(feature = "http-client-curl")]
type HttpImpl = curl::Http;

pub struct Transport {
    url: String,
    user_agent_header: &'static str,
    version: crate::Protocol,
    http: HttpImpl,
}

impl crate::client::Transport for Transport {}

fn append_url(base: &str, suffix: String) -> String {
    if base.ends_with('/') {
        format!("{}{}", base, suffix)
    } else {
        format!("{}/{}", base, suffix)
    }
}

impl crate::client::TransportSketch for Transport {
    fn set_service(&mut self, service: Service) -> Result<SetServiceResponse<'static>, crate::client::Error> {
        let url = append_url(&self.url, format!("info/refs?service={}", service.as_str()));
        let static_headers = [Cow::Borrowed(self.user_agent_header)];
        let mut dynamic_headers = Vec::<Cow<str>>::new();
        if self.version != Protocol::V1 {
            dynamic_headers.push(Cow::Owned(format!("Git-Protocol: version={}", self.version as usize)));
        }
        self.http
            .get(&url, static_headers.iter().chain(&dynamic_headers))
            .map_err(|err| Box::new(err) as Box<dyn std::error::Error>)?;
        unimplemented!("set service http")
    }
}

pub fn connect(url: &str, version: crate::Protocol) -> Result<Transport, Infallible> {
    Ok(Transport {
        url: url.to_owned(),
        user_agent_header: concat!("User-Agent: git/oxide-", env!("CARGO_PKG_VERSION")),
        version,
        http: HttpImpl::new(),
    })
}
