use std::convert::Infallible;

use bstr::{BStr, BString, ByteSlice};

#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error("Could not decode URL as UTF8")]
    Utf8(#[from] std::str::Utf8Error),
    #[error(transparent)]
    Url(#[from] url::ParseError),
    #[error("URLs need to specify the path to the repository")]
    MissingResourceLocation,
    #[error("file URLs require an absolute or relative path to the repository")]
    MissingRepositoryPath,
    #[error("\"{url}\" is not a valid local path")]
    NotALocalFile { url: BString },
    #[error("Relative URLs are not permitted: {url:?}")]
    RelativeUrl { url: String },
}

impl From<Infallible> for Error {
    fn from(_: Infallible) -> Self {
        unreachable!("Cannot actually happen, but it seems there can't be a blanket impl for this")
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
        InputScheme::Url { .. } => parse_url(input),
        InputScheme::Scp { colon } => parse_scp(input, colon),
        InputScheme::Local => parse_local(input),
    }
}

fn parse_url(input: &BStr) -> Result<crate::Url, Error> {
    let url = url::Url::parse(std::str::from_utf8(input)?)?;

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
    let input = std::str::from_utf8(input)?;

    // TODO: this incorrectly splits at IPv6 addresses, check for `[]` before splitting
    let (host, path) = input.split_at(colon);
    let path = &path[1..]; // remove leading `:`

    // The path returned by the parsed url often has the wrong number of leading `/` characters but
    // should never differ in any other way (ssh URLs should not contain a query or fragment part).
    // To avoid the various off-by-one errors caused by the `/` characters, we keep using the path
    // determined above and can therefore skip parsing it here as well.
    let url = url::Url::parse(&format!("ssh://{host}"))?;

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

fn parse_local(input: &BStr) -> Result<crate::Url, Error> {
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
