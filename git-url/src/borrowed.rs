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
