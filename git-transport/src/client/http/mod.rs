use crate::client::SetServiceResponse;
use crate::Service;
use quick_error::quick_error;
use std::convert::Infallible;
use std::io;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Tbd {
            display("tbd")
        }
    }
}
#[cfg(feature = "http-client-curl")]
pub(crate) mod curl;

trait Http {
    type Headers: Iterator<Item = Vec<u8>>;
    type ResponseBody: io::Read;

    fn get(
        &mut self,
        url: &str,
        headers: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Result<(Self::Headers, Self::ResponseBody), Error>;
    fn post(
        &mut self,
        url: &str,
        headers: impl IntoIterator<Item = impl AsRef<str>>,
        body: impl io::Read,
    ) -> Result<(Self::Headers, Self::ResponseBody), Error>;
}

#[cfg(feature = "http-client-curl")]
type HttpImpl = curl::CurlHttp;

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
        self.http.get(&url, &[self.user_agent_header])?;
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
