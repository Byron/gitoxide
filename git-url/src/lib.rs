#![forbid(unsafe_code)]

use std::{convert::TryFrom, fmt};

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum Protocol {
    File,
    Git,
    Ssh,
    Http,
    Https,
}

impl fmt::Display for Protocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Protocol::*;
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
    pub protocol: Protocol,
    pub user: Option<String>,
    pub host: Option<String>,
    pub port: Option<u16>,
    pub path: bstr::BString,
}

impl Default for Url {
    fn default() -> Self {
        Url {
            protocol: Protocol::Ssh,
            user: None,
            host: None,
            port: None,
            path: bstr::BString::default(),
        }
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
pub use expand_path::doit as expand_path;

pub mod parse;
#[doc(inline)]
pub use parse::parse;
