//! A library implementing a URL for use in git with access to its special capabilities.
//! ## Feature Flags
#![cfg_attr(
feature = "document-features",
cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![forbid(unsafe_code)]
#![deny(rust_2018_idioms, missing_docs)]

use std::{
    convert::TryFrom,
    fmt::{self},
};

use bstr::{BStr, BString};

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
#[allow(missing_docs)]
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
    user: Option<String>,
    /// The host to which to connect. Localhost is implied if `None`.
    host: Option<String>,
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

/// Instantiation
impl Url {
    /// Create a new instance from the given parts, which will be validated by parsing them back.
    pub fn from_parts(
        scheme: Scheme,
        user: Option<String>,
        host: Option<String>,
        port: Option<u16>,
        path: BString,
    ) -> Result<Self, parse::Error> {
        parse(
            Url {
                scheme,
                user,
                host,
                port,
                path,
            }
            .to_bstring()
            .as_ref(),
        )
    }
}

/// Modification
impl Url {
    /// Set the given `user`, with `None` unsetting it. Returns the previous value.
    pub fn set_user(&mut self, user: Option<String>) -> Option<String> {
        let prev = self.user.take();
        self.user = user;
        prev
    }
}

/// Access
impl Url {
    /// Returns the user mentioned in the url, if present.
    pub fn user(&self) -> Option<&str> {
        self.user.as_deref()
    }
    /// Returns the host mentioned in the url, if present.
    pub fn host(&self) -> Option<&str> {
        self.host.as_deref()
    }
}

/// Serialization
impl Url {
    /// Write this URL losslessly to `out`, ready to be parsed again.
    pub fn write_to(&self, mut out: impl std::io::Write) -> std::io::Result<()> {
        out.write_all(self.scheme.as_str().as_bytes())?;
        out.write_all(b"://")?;
        match (&self.user, &self.host) {
            (Some(user), Some(host)) => {
                out.write_all(user.as_bytes())?;
                out.write_all(&[b'@'])?;
                out.write_all(host.as_bytes())?;
            }
            (None, Some(host)) => {
                out.write_all(host.as_bytes())?;
            }
            (None, None) => {}
            (Some(_user), None) => unreachable!("BUG: should not be possible to have a user but no host"),
        };
        if let Some(port) = &self.port {
            write!(&mut out, ":{}", port)?;
        }
        out.write_all(&self.path)?;
        Ok(())
    }

    /// Transform ourselves into a binary string, losslessly, or fail if the URL is malformed due to host or user parts being incorrect.
    pub fn to_bstring(&self) -> bstr::BString {
        let mut buf = Vec::with_capacity(
            (5 + 3)
                + self.user.as_ref().map(|n| n.len()).unwrap_or_default()
                + 1
                + self.host.as_ref().map(|h| h.len()).unwrap_or_default()
                + self.port.map(|_| 5).unwrap_or_default()
                + self.path.len(),
        );
        self.write_to(&mut buf).expect("io cannot fail in memory");
        buf.into()
    }
}

impl Url {
    /// Parse a URL from `bytes`
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, parse::Error> {
        parse(bytes)
    }
}

impl TryFrom<&str> for Url {
    type Error = parse::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_bytes(value.as_bytes())
    }
}

impl TryFrom<String> for Url {
    type Error = parse::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::from_bytes(value.as_bytes())
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
