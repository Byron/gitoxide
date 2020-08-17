use crate::{owned, Protocol};
use quick_error::quick_error;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Utf8(err: std::str::Utf8Error) {
            display("Could not decode URL as UTF8")
            from()
            source(err)
        }
        Url(err: url::ParseError) {
            display("the URL could not be parsed")
            from()
            source(err)
        }
        UnsupportedProtocol(protocol: String) {
            display("Protocol '{}' is not supported", protocol)
        }
        EmptyPath {
            display("Paths cannot be empty")
        }
    }
}

fn str_to_protocol(s: &str) -> Result<Protocol, Error> {
    Ok(match s {
        "ssh" => Protocol::Ssh,
        _ => return Err(Error::UnsupportedProtocol(s.into())),
    })
}

pub fn parse(url: &[u8]) -> Result<owned::Url, Error> {
    let url = url::Url::parse(std::str::from_utf8(url)?)?;
    if url.path().is_empty() {
        return Err(Error::EmptyPath);
    }
    Ok(owned::Url {
        protocol: str_to_protocol(url.scheme())?,
        user: if url.username().is_empty() {
            None
        } else {
            Some(url.username().into())
        },
        host: url.host_str().map(Into::into),
        port: url.port(),
        path: url.path().into(),
        expand_user: None,
    })
}
