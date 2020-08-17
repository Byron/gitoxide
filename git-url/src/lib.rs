#![forbid(unsafe_code)]

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum Protocol {
    Ssh,
    File,
}

pub mod owned {
    use crate::Protocol;

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
        pub path: String,
        pub expand_user: Option<UserExpansion>,
    }
}

#[doc(inline)]
pub use owned::Url as Owned;

pub mod parse;
#[doc(inline)]
pub use parse::parse;
