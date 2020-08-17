use crate::{owned, Protocol};
use bstr::ByteSlice;
use quick_error::quick_error;
use std::borrow::Cow;

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
        "file" => Protocol::File,
        _ => return Err(Error::UnsupportedProtocol(s.into())),
    })
}

fn guess_protocol(url: &[u8]) -> &str {
    match url.find_byte(b':') {
        Some(colon_pos) => {
            if url[..colon_pos].find_byte(b'.').is_some() {
                "ssh"
            } else {
                "file"
            }
        }
        None => "file",
    }
}

fn sanitize_for_protocol<'a>(protocol: &str, url: &'a str) -> Cow<'a, str> {
    match protocol {
        "ssh" => url.replacen(":", "/", 1).into(),
        _ => url.into(),
    }
}

fn has_no_explicit_protocol(url: &[u8]) -> bool {
    url.find(b"://").is_none()
}

fn possibly_strip_file_protocol(url: &[u8]) -> &[u8] {
    if url.starts_with(b"file://") {
        &url[b"file://".len()..]
    } else {
        url
    }
}

/// Note: We cannot and should never have to deal with UTF-16 encoded windows strings, so bytes input is acceptable.
/// For file-paths, we don't expect UTF8 encoding either.
pub fn parse(url: &[u8]) -> Result<owned::Url, Error> {
    let guessed_protocol = guess_protocol(url);
    if possibly_strip_file_protocol(url) != url || (has_no_explicit_protocol(url) && guessed_protocol == "file") {
        return Ok(owned::Url {
            protocol: Protocol::File,
            path: possibly_strip_file_protocol(url).into(),
            ..Default::default()
        });
    }

    let url_str = std::str::from_utf8(url)?;
    let mut url = match url::Url::parse(url_str) {
        Ok(url) => url,
        Err(url::ParseError::RelativeUrlWithoutBase) => {
            // happens with bare paths as well as scp like paths. The latter contain a ':' past the host portion,
            // which we are trying to detect.
            url::Url::parse(&format!(
                "{}://{}",
                guessed_protocol,
                sanitize_for_protocol(guessed_protocol, url_str)
            ))?
        }
        Err(err) => return Err(err.into()),
    };
    // SCP like URLs without user parse as 'something' with the scheme being the host
    if url.scheme().find('.').is_some() {
        // try again with prefixed protocol
        url = url::Url::parse(&format!("ssh://{}", sanitize_for_protocol("ssh", url_str)))?;
    }
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
