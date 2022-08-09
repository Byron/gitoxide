//! A library implementing a URL for use in git with access to its special capabilities.
//! ## Feature Flags
#![cfg_attr(
feature = "document-features",
cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![forbid(unsafe_code)]
#![deny(rust_2018_idioms)]

use std::{
    convert::TryFrom,
    fmt::{self, Write},
};

use bstr::{BStr, ByteSlice};

///
pub mod parse;
#[doc(inline)]
pub use parse::parse;

///
pub mod expand_path;
#[doc(inline)]
pub use expand_path::expand_path;

/// A scheme for use in a [`Url`]
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum Scheme {
    File,
    Git,
    Ssh,
    Http,
    Https,
    // TODO: replace this with custom formats, maybe, get an idea how to do that.
    Radicle,
}

impl Scheme {
    /// Return ourselves parseable name.
    pub fn as_str(&self) -> &'static str {
        use Scheme::*;
        match self {
            File => "file",
            Git => "git",
            Ssh => "ssh",
            Http => "http",
            Https => "https",
            Radicle => "rad",
        }
    }
}

impl fmt::Display for Scheme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// A URL with support for specialized git related capabilities.
///
/// Additionally there is support for [deserialization][Url::from_bytes()] and serialization
/// (_see the `Display::fmt()` implementation_).
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Url {
    /// The URL scheme.
    pub scheme: Scheme,
    /// The user to impersonate on the remote.
    pub user: Option<String>,
    /// The host to which to connect. Localhost is implied if `None`.
    pub host: Option<String>,
    /// The port to use when connecting to a host. If `None`, standard ports depending on `scheme` will be used.
    pub port: Option<u16>,
    /// The path portion of the URL, usually the location of the git repository.
    pub path: bstr::BString,
}

impl Default for Url {
    fn default() -> Self {
        Url {
            scheme: Scheme::Ssh,
            user: None,
            host: None,
            port: None,
            path: bstr::BString::default(),
        }
    }
}

/// Serialization
impl Url {
    /// Transform ourselves into a binary string, losslessly, or `None` if user and host is strangely configured.
    pub fn to_bstring(&self) -> Option<bstr::BString> {
        let mut buf = Vec::with_capacity(
            (5 + 3)
                + self.user.as_ref().map(|n| n.len()).unwrap_or_default()
                + 1
                + self.host.as_ref().map(|h| h.len()).unwrap_or_default()
                + self.port.map(|_| 5).unwrap_or_default()
                + self.path.len(),
        );
        buf.extend_from_slice(self.scheme.as_str().as_bytes());
        buf.extend_from_slice(b"://");
        match (&self.user, &self.host) {
            (Some(user), Some(host)) => {
                buf.extend_from_slice(user.as_bytes());
                buf.push(b'@');
                buf.extend_from_slice(host.as_bytes());
            }
            (None, Some(host)) => {
                buf.extend_from_slice(host.as_bytes());
            }
            (None, None) => {}
            _ => return None,
        };
        if let Some(port) = &self.port {
            use std::io::Write;
            buf.push(b':');
            let mut numbers = [0u8; 5];
            write!(numbers.as_mut_slice(), "{}", port).expect("write succeeds as max number fits into buffer");
            buf.extend(numbers.iter().take_while(|b| **b != 0));
        }
        buf.extend_from_slice(&self.path);
        Some(buf.into())
    }
}
impl fmt::Display for Url {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.scheme.fmt(f)?;
        f.write_str("://")?;
        match (&self.user, &self.host) {
            (Some(user), Some(host)) => f.write_fmt(format_args!("{}@{}", user, host)),
            (None, Some(host)) => f.write_str(host),
            (None, None) => Ok(()),
            _ => return Err(fmt::Error),
        }?;
        if let Some(port) = &self.port {
            f.write_char(':')?;
            f.write_fmt(format_args!("{}", port))?;
        }
        f.write_str(self.path.to_str_lossy().as_ref())
    }
}

impl Url {
    /// Parse a URL from `bytes`
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, parse::Error> {
        parse(bytes)
    }
}

impl TryFrom<&BStr> for Url {
    type Error = parse::Error;

    fn try_from(value: &BStr) -> Result<Self, Self::Error> {
        Self::from_bytes(value)
    }
}

impl<'a> TryFrom<std::borrow::Cow<'a, BStr>> for Url {
    type Error = parse::Error;

    fn try_from(value: std::borrow::Cow<'a, BStr>) -> Result<Self, Self::Error> {
        Self::try_from(&*value)
    }
}
