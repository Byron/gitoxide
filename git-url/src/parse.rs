use std::{borrow::Cow, convert::Infallible};

pub use bstr;
use bstr::{BStr, ByteSlice};

use crate::Scheme;

/// The Error returned by [`parse()`]
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error("Could not decode URL as UTF8")]
    Utf8(#[from] std::str::Utf8Error),
    #[error(transparent)]
    Url(#[from] url::ParseError),
    #[error("Paths cannot be empty")]
    EmptyPath,
    #[error("Relative URLs are not permitted: {url:?}")]
    RelativeUrl { url: String },
}

impl From<Infallible> for Error {
    fn from(_: Infallible) -> Self {
        unreachable!("Cannot actually happen, but it seems there can't be a blanket impl for this")
    }
}

fn str_to_protocol(s: &str) -> Scheme {
    Scheme::from(s)
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

fn try_strip_file_protocol(url: &[u8]) -> Option<&[u8]> {
    url.strip_prefix(b"file://")
}

fn to_owned_url(url: url::Url) -> Result<crate::Url, Error> {
    Ok(crate::Url {
        serialize_alternative_form: false,
        scheme: str_to_protocol(url.scheme()),
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
pub fn parse(input: &BStr) -> Result<crate::Url, Error> {
    let guessed_protocol = guess_protocol(input);
    let path_without_protocol = try_strip_file_protocol(input);
    if path_without_protocol.is_some() || (has_no_explicit_protocol(input) && guessed_protocol == "file") {
        return Ok(crate::Url {
            scheme: Scheme::File,
            path: path_without_protocol.unwrap_or(input).into(),
            serialize_alternative_form: !input.starts_with(b"file://"),
            ..Default::default()
        });
    }

    let url_str = std::str::from_utf8(input)?;
    let (mut url, mut sanitized_scp) = match url::Url::parse(url_str) {
        Ok(url) => (url, false),
        Err(url::ParseError::RelativeUrlWithoutBase) => {
            // happens with bare paths as well as scp like paths. The latter contain a ':' past the host portion,
            // which we are trying to detect.
            (
                url::Url::parse(&format!(
                    "{}://{}",
                    guessed_protocol,
                    sanitize_for_protocol(guessed_protocol, url_str)
                ))?,
                true,
            )
        }
        Err(err) => return Err(err.into()),
    };
    // SCP like URLs without user parse as 'something' with the scheme being the 'host'. Hosts always have dots.
    if url.scheme().find('.').is_some() {
        // try again with prefixed protocol
        url = url::Url::parse(&format!("ssh://{}", sanitize_for_protocol("ssh", url_str)))?;
        sanitized_scp = true;
    }
    if url.scheme() != "rad" && url.path().is_empty() {
        return Err(Error::EmptyPath);
    }
    if url.cannot_be_a_base() {
        return Err(Error::RelativeUrl { url: url.into() });
    }

    to_owned_url(url).map(|url| url.serialize_alternate_form(sanitized_scp))
}
