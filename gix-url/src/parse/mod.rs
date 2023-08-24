use std::convert::Infallible;

use bstr::{BStr, BString, ByteSlice};

#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error("{} \"{url}\" is not valid UTF-8", .kind.get_error_str())]
    Utf8 {
        url: BString,
        kind: UrlKind,
        source: std::str::Utf8Error,
    },
    #[error("{} {url:?} can not be parsed as valid URL", .kind.get_error_str())]
    Url {
        url: String,
        kind: UrlKind,
        source: url::ParseError,
    },
    #[error("{} \"{url}\" does not specify a path to a repository", .kind.get_error_str())]
    MissingRepositoryPath { url: BString, kind: UrlKind },
    #[error("URL {url:?} is relative which is not allowed in this context")]
    RelativeUrl { url: String },
}

impl From<Infallible> for Error {
    fn from(_: Infallible) -> Self {
        unreachable!("Cannot actually happen, but it seems there can't be a blanket impl for this")
    }
}

///
#[derive(Debug)]
pub enum UrlKind {
    ///
    Url,
    ///
    Scp,
    ///
    Local,
}

impl UrlKind {
    fn get_error_str(&self) -> &'static str {
        match self {
            UrlKind::Url => "URL",
            UrlKind::Scp => "SCP-like target",
            UrlKind::Local => "local path",
        }
    }
}

enum InputScheme {
    Url { protocol_end: usize },
    Scp { colon: usize },
    Local,
}

fn find_scheme(input: &BStr) -> InputScheme {
    // TODO: url's may only contain `:/`, we should additionally check if the characters used for
    // protocol are all valid
    if let Some(protocol_end) = input.find("://") {
        return InputScheme::Url { protocol_end };
    }

    if let Some(colon) = input.find_byte(b':') {
        // allow user to select files containing a `:` by passing them as absolute or relative path
        // this is behavior explicitly mentioned by the scp and git manuals
        if !&input[..colon].contains(&b'/') {
            // TODO: implement windows specific checks
            return InputScheme::Scp { colon };
        }
    }

    InputScheme::Local
}

/// Parse the given `bytes` as a [git url](crate::Url).
///
/// # Note
///
/// We cannot and should never have to deal with UTF-16 encoded windows strings, so bytes input is acceptable.
/// For file-paths, we don't expect UTF8 encoding either.
pub fn parse(input: &BStr) -> Result<crate::Url, Error> {
    match find_scheme(input) {
        InputScheme::Local => parse_local(input, false),
        InputScheme::Url { protocol_end } if input[..protocol_end].eq_ignore_ascii_case(b"file") => {
            // strip the protocol part
            parse_local(&input[protocol_end + 3..], true)
        }
        InputScheme::Url { .. } => parse_url(input),
        InputScheme::Scp { colon } => parse_scp(input, colon),
    }
}

fn parse_url(input: &BStr) -> Result<crate::Url, Error> {
    let input = std::str::from_utf8(input).map_err(|source| Error::Utf8 {
        url: input.to_owned(),
        kind: UrlKind::Url,
        source,
    })?;
    let url = url::Url::parse(input).map_err(|source| Error::Url {
        url: input.to_owned(),
        kind: UrlKind::Url,
        source,
    })?;

    let scheme = url.scheme().into();

    if matches!(scheme, crate::Scheme::Git | crate::Scheme::Ssh) && url.path().is_empty() {
        return Err(Error::MissingRepositoryPath {
            url: input.to_owned().into(),
            kind: UrlKind::Url,
        });
    }

    if url.cannot_be_a_base() {
        return Err(Error::RelativeUrl { url: input.to_owned() });
    }

    Ok(crate::Url {
        serialize_alternative_form: false,
        scheme: url.scheme().into(),
        password: url.password().map(Into::into),
        user: if url.username().is_empty() && url.password().is_none() {
            None
        } else {
            Some(url.username().into())
        },
        host: url.host_str().map(Into::into),
        port: url.port(),
        path: url.path().into(),
    })
}

fn parse_scp(input: &BStr, colon: usize) -> Result<crate::Url, Error> {
    let input = std::str::from_utf8(input).map_err(|source| Error::Utf8 {
        url: input.to_owned(),
        kind: UrlKind::Scp,
        source,
    })?;

    // TODO: this incorrectly splits at IPv6 addresses, check for `[]` before splitting
    let (host, path) = input.split_at(colon);
    let path = &path[1..]; // remove leading `:`

    if path.is_empty() {
        return Err(Error::MissingRepositoryPath {
            url: input.to_owned().into(),
            kind: UrlKind::Scp,
        });
    }

    // The path returned by the parsed url often has the wrong number of leading `/` characters but
    // should never differ in any other way (ssh URLs should not contain a query or fragment part).
    // To avoid the various off-by-one errors caused by the `/` characters, we keep using the path
    // determined above and can therefore skip parsing it here as well.
    let url = url::Url::parse(&format!("ssh://{host}")).map_err(|source| Error::Url {
        url: input.to_owned(),
        kind: UrlKind::Scp,
        source,
    })?;

    Ok(crate::Url {
        serialize_alternative_form: true,
        scheme: url.scheme().into(),
        password: url.password().map(Into::into),
        user: if url.username().is_empty() && url.password().is_none() {
            None
        } else {
            Some(url.username().into())
        },
        host: url.host_str().map(Into::into),
        port: url.port(),
        path: path.into(),
    })
}

fn parse_local(input: &BStr, was_in_url_format: bool) -> Result<crate::Url, Error> {
    if input.is_empty() {
        return Err(Error::MissingRepositoryPath {
            url: input.to_owned(),
            kind: UrlKind::Local,
        });
    }

    // TODO: handle relative paths, Git does weird stuff

    Ok(crate::Url {
        serialize_alternative_form: !was_in_url_format,
        scheme: crate::scheme::Scheme::File,
        password: None,
        user: None,
        host: None,
        port: None,
        path: input.into(),
    })
}
