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
#[derive(Debug, Clone)]
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
        let explicitly_local = &input[..colon].contains(&b'/');
        let dos_driver_letter = cfg!(windows) && input[..colon].len() == 1;

        if !explicitly_local && !dos_driver_letter {
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
        InputScheme::Local => parse_local(input),
        InputScheme::Url { protocol_end } if input[..protocol_end].eq_ignore_ascii_case(b"file") => {
            parse_file_url(input, protocol_end)
        }
        InputScheme::Url { .. } => parse_url(input),
        InputScheme::Scp { colon } => parse_scp(input, colon),
    }
}

fn parse_url(input: &BStr) -> Result<crate::Url, Error> {
    let (input, url) = input_to_utf8_and_url(input, UrlKind::Url)?;

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
    let input = input_to_utf8(input, UrlKind::Scp)?;

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

fn parse_file_url(input: &BStr, protocol_colon: usize) -> Result<crate::Url, Error> {
    let input = input_to_utf8(input, UrlKind::Url)?;
    let input_after_protocol = &input[protocol_colon + 3..];

    let Some(first_slash) = input_after_protocol.find('/') else {
        return Err(Error::MissingRepositoryPath {
            url: input.to_owned().into(),
            kind: UrlKind::Url,
        });
    };

    // We can not use the url crate to parse host and path because it special cases Windows
    // driver letters. With the url crate an input of `file://x:/path/to/git` is parsed as empty
    // host and with `x:/path/to/git` as path. This behavior is wrong for Git which parses the
    // `x:` as the host.
    // TODO: this behavior is most likely different on Windows
    let host = if first_slash == 0 {
        // file:///path/to/git
        None
    } else {
        // file://host/path/to/git
        Some(&input_after_protocol[..first_slash])
    };
    // path includes the slash character
    let path = &input_after_protocol[first_slash..];

    Ok(crate::Url {
        serialize_alternative_form: false,
        host: host.map(Into::into),
        ..parse_local(path.into())?
    })
}

fn parse_local(input: &BStr) -> Result<crate::Url, Error> {
    if input.is_empty() {
        return Err(Error::MissingRepositoryPath {
            url: input.to_owned(),
            kind: UrlKind::Local,
        });
    }

    Ok(crate::Url {
        serialize_alternative_form: true,
        scheme: crate::scheme::Scheme::File,
        password: None,
        user: None,
        host: None,
        port: None,
        path: input.into(),
    })
}

/// Helper function to turn a BStr into an str. The kind is only used for the construction of the
/// error variant.
fn input_to_utf8(input: &BStr, kind: UrlKind) -> Result<&str, Error> {
    std::str::from_utf8(input).map_err(|source| Error::Utf8 {
        url: input.to_owned(),
        kind,
        source,
    })
}

/// Helper function to turn a BStr into an Url. The kind is only used for the construction of the
/// error variant.
fn input_to_utf8_and_url(input: &BStr, kind: UrlKind) -> Result<(&str, url::Url), Error> {
    let input = input_to_utf8(input, kind.clone())?;
    url::Url::parse(input)
        .map(|url| (input, url))
        .map_err(|source| Error::Url {
            url: input.to_owned(),
            kind,
            source,
        })
}
