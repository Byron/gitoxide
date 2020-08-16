#![forbid(unsafe_code)]

/// For convenience to allow using `bstr` without adding it to own cargo manifest
pub use bstr;

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum Protocol {
    Ssh,
}

pub mod borrowed {
    use crate::Protocol;
    use bstr::BStr;

    #[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
    #[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
    pub enum UserExpansion<'a> {
        Current,
        #[cfg_attr(feature = "serde1", serde(borrow))]
        Name(&'a BStr),
    }

    #[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
    #[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
    pub struct Url<'a> {
        pub protocol: Protocol,
        #[cfg_attr(feature = "serde1", serde(borrow))]
        pub user: Option<&'a BStr>,
        pub host: Option<&'a BStr>,
        pub port: Option<u32>,
        pub path: &'a BStr,
        pub expand_user: Option<UserExpansion<'a>>,
    }
}
#[doc(inline)]
pub use borrowed::Url as Borrowed;
pub mod parse;

#[doc(inline)]
pub use parse::parse;
