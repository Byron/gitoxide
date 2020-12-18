//! A library implementing a URL for use in git with access to its special capabilities.
#![forbid(unsafe_code)]
#![deny(rust_2018_idioms)]

use bstr::ByteSlice;
use std::{
    convert::TryFrom,
    fmt::{self, Write},
};

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
}

impl fmt::Display for Scheme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Scheme::*;
        f.write_str(match self {
            File => "file",
            Git => "git",
            Ssh => "ssh",
            Http => "http",
            Https => "https",
        })
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

impl TryFrom<&[u8]> for Url {
    type Error = parse::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Self::from_bytes(value)
    }
}
