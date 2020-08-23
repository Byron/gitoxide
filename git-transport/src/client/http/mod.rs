use crate::{
    client::{git, SetServiceResponse},
    Protocol, Service,
};
use git_features::pipe;
use quick_error::quick_error;
use std::{
    borrow::Cow,
    convert::Infallible,
    io::{self, Read},
};

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

#[allow(clippy::type_complexity)]
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
    line_reader: git_packetline::Reader<pipe::Reader>,
}

impl Transport {
    pub fn new(url: &str, version: crate::Protocol) -> Self {
        Transport {
            url: url.to_owned(),
            user_agent_header: concat!("User-Agent: git/oxide-", env!("CARGO_PKG_VERSION")),
            version,
            http: HttpImpl::new(),
            line_reader: git_packetline::Reader::new(pipe::unidirectional(0).1, None),
        }
    }
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
    fn set_service(&mut self, service: Service) -> Result<SetServiceResponse, crate::client::Error> {
        let url = append_url(&self.url, format!("info/refs?service={}", service.as_str()));
        let static_headers = [Cow::Borrowed(self.user_agent_header)];
        let mut dynamic_headers = Vec::<Cow<str>>::new();
        if self.version != Protocol::V1 {
            dynamic_headers.push(Cow::Owned(format!("Git-Protocol: version={}", self.version as usize)));
        }
        let GetResponse { mut headers, body } = self.http.get(&url, static_headers.iter().chain(&dynamic_headers))?;
        // TODO: check for Content-Type: application/x-git-upload-pack-advertisement
        io::copy(&mut headers, &mut io::sink())?;

        self.line_reader.replace(body);

        let mut announced_service = String::new();
        self.line_reader.as_read().read_to_string(&mut announced_service)?;
        let expected_service_announcement = format!("# service={}", service.as_str());
        if announced_service.trim() != expected_service_announcement {
            return Err(crate::client::Error::Http(Error::Detail(format!(
                "Expected to see {:?}, but got {:?}",
                expected_service_announcement,
                announced_service.trim()
            ))));
        }

        let (capabilities, refs) = git::recv::capabilties_and_possibly_refs(&mut self.line_reader)?;
        Ok(SetServiceResponse {
            actual_protocol: Protocol::V1, // TODO
            capabilities,
            refs,
        })
    }
}

pub fn connect(url: &str, version: crate::Protocol) -> Result<Transport, Infallible> {
    Ok(Transport::new(url, version))
}
