#![forbid(unsafe_code)]

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum Protocol {
    File,
    Git,
    Ssh,
    Http,
    Https,
}

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum UserExpansion {
    Current,
    Name(String),
}

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Url {
    pub protocol: Protocol,
    pub user: Option<String>,
    pub host: Option<String>,
    pub port: Option<u16>,
    pub path: bstr::BString,
    pub expansion: Option<UserExpansion>,
}

impl Default for Url {
    fn default() -> Self {
        Url {
            protocol: Protocol::Ssh,
            user: None,
            host: None,
            port: None,
            path: bstr::BString::default(),
            expansion: None,
        }
    }
}

pub mod expand_path;

pub mod parse;
#[doc(inline)]
pub use parse::parse;
