use std::borrow::Cow;

use bstr::ByteSlice;

use crate::Scheme;

/// The Error returned by [`parse()`]
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error("Could not decode URL as UTF8")]
    Utf8(#[from] std::str::Utf8Error),
    #[error(transparent)]
    Url(#[from] url::ParseError),
    #[error("Protocol {protocol:?} is not supported")]
    UnsupportedProtocol { protocol: String },
    #[error("Paths cannot be empty")]
    EmptyPath,
    #[error("Relative URLs are not permitted: {url:?}")]
    RelativeUrl { url: String },
}

fn str_to_protocol(s: &str) -> Result<Scheme, Error> {
    Ok(match s {
        "ssh" => Scheme::Ssh,
        "file" => Scheme::File,
        "git" => Scheme::Git,
        "http" => Scheme::Http,
        "https" => Scheme::Https,
        "rad" => Scheme::Radicle,
        _ => return Err(Error::UnsupportedProtocol { protocol: s.into() }),
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
        "ssh" => url.replacen(':', "/", 1).into(),
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

fn to_owned_url(url: url::Url) -> Result<crate::Url, Error> {
    Ok(crate::Url {
        scheme: str_to_protocol(url.scheme())?,
        user: if url.username().is_empty() {
            None
        } else {
            Some(url.username().into())
        },
        host: url.host_str().map(Into::into),
        port: url.port(),
        path: url.path().into(),
    })
}

/// Parse the given `bytes` as git url.
///
/// # Note
///
/// We cannot and should never have to deal with UTF-16 encoded windows strings, so bytes input is acceptable.
/// For file-paths, we don't expect UTF8 encoding either.
pub fn parse(bytes: &[u8]) -> Result<crate::Url, Error> {
    let guessed_protocol = guess_protocol(bytes);
    if possibly_strip_file_protocol(bytes) != bytes || (has_no_explicit_protocol(bytes) && guessed_protocol == "file") {
        return Ok(crate::Url {
            scheme: Scheme::File,
            path: possibly_strip_file_protocol(bytes).into(),
            ..Default::default()
        });
    }

    let url_str = std::str::from_utf8(bytes)?;
    let mut url = match url::Url::parse(url_str) {
        Ok(url) => url,
        Err(::url::ParseError::RelativeUrlWithoutBase) => {
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
    // SCP like URLs without user parse as 'something' with the scheme being the 'host'. Hosts always have dots.
    if url.scheme().find('.').is_some() {
        // try again with prefixed protocol
        url = url::Url::parse(&format!("ssh://{}", sanitize_for_protocol("ssh", url_str)))?;
    }
    if url.scheme() != "rad" && url.path().is_empty() {
        return Err(Error::EmptyPath);
    }
    if url.cannot_be_a_base() {
        return Err(Error::RelativeUrl { url: url.into() });
    }

    to_owned_url(url)
}
