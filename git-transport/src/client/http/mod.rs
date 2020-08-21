use crate::client::SetServiceResponse;
use crate::Service;
use quick_error::quick_error;
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
        headers: impl Iterator<Item = impl AsRef<str>>,
    ) -> Result<(Self::Headers, Self::ResponseBody), Error>;
    fn post(
        &mut self,
        url: &str,
        headers: impl Iterator<Item = impl AsRef<str>>,
        body: impl io::Read,
    ) -> Result<(Self::Headers, Self::ResponseBody), Error>;
}

#[cfg(feature = "http-client-curl")]
type HttpImpl = curl::CurlHttp;

pub struct Transport {
    url: String,
    version: crate::Protocol,
    http: HttpImpl,
}

impl crate::client::Transport for Transport {}

impl crate::client::TransportSketch for Transport {
    fn set_service(&mut self, service: Service) -> Result<SetServiceResponse<'static>, crate::client::Error> {
        unimplemented!("set service http")
    }
}

pub fn connect(url: &str, version: crate::Protocol) -> Result<Transport, Error> {
    Ok(Transport {
        url: url.to_owned(),
        version,
        http: HttpImpl::new(),
    })
}
