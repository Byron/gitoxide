use crate::client::SetServiceResponse;
use crate::{Protocol, Service};
use quick_error::quick_error;
use std::io::BufRead;
use std::{borrow::Cow, convert::Infallible, io};

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Detail(description: String) {
            display("{}", description)
        }
        PostBody(err: io::Error) {
            display("An IO error occurred while uploading the body of a POST request")
            from()
            source(err)
        }
    }
}

#[cfg(feature = "http-client-curl")]
pub(crate) mod curl;

pub struct GetResponse<H, B> {
    headers: H,
    body: B,
}

pub struct PostResponse<H, B, PB> {
    /// **Note**: Implementations should drop the handle to avoid deadlocks
    post_body: PB,
    headers: H,
    body: B,
}

impl<A, B, C> From<PostResponse<A, B, C>> for GetResponse<A, B> {
    fn from(v: PostResponse<A, B, C>) -> Self {
        GetResponse {
            headers: v.headers,
            body: v.body,
        }
    }
}

trait Http {
    type Headers: io::BufRead;
    type ResponseBody: io::BufRead;
    type PostBody: io::Write;

    fn get(
        &mut self,
        url: &str,
        headers: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Result<GetResponse<Self::Headers, Self::ResponseBody>, Error>;
    fn post(
        &mut self,
        url: &str,
        headers: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Result<PostResponse<Self::Headers, Self::ResponseBody, Self::PostBody>, Error>;
}

#[cfg(feature = "http-client-curl")]
type HttpImpl = curl::Curl;

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
        let GetResponse { headers, body } = self.http.get(&url, static_headers.iter().chain(&dynamic_headers))?;
        eprintln!("HEADERS");
        for header in headers.lines() {
            let header = header?;
            eprintln!("{}", header);
        }
        eprintln!("BODY");
        for line in body.lines() {
            let line = line?;
            eprintln!("{}", line);
        }
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
