#![forbid(unsafe_code)]

use std::{convert::TryFrom, fmt};

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

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Url {
    pub scheme: Scheme,
    pub user: Option<String>,
    pub host: Option<String>,
    pub port: Option<u16>,
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
    pub fn from_bytes(url: &[u8]) -> Result<Self, parse::Error> {
        parse(url)
    }
}

impl TryFrom<&[u8]> for Url {
    type Error = parse::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Self::from_bytes(value)
    }
}

pub mod expand_path;
#[doc(inline)]
pub use expand_path::expand_path;

pub mod parse;
use bstr::ByteSlice;
#[doc(inline)]
pub use parse::parse;
use std::fmt::Write;
