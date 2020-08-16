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

pub mod parse {
    use crate::borrowed;
    use nom::error::ParseError;
    use quick_error::quick_error;

    quick_error! {
        #[derive(Debug)]
        pub enum Error {
            NomDetail(input: bstr::BString, msg: &'static str) {
                display("{}: '{}' could not be parsed", msg, input)
            }
        }
    }

    impl Error {
        fn set_parse_context(mut self, ctx: &'static str) -> Self {
            if let Error::NomDetail(_, ref mut message) = self {
                *message = ctx
            }
            self
        }

        pub(crate) fn context(msg: &'static str) -> impl Fn(nom::Err<Self>) -> nom::Err<Self> {
            move |e: nom::Err<Self>| e.map(|e| e.set_parse_context(msg))
        }
    }

    impl ParseError<&[u8]> for Error {
        fn from_error_kind(input: &[u8], _kind: nom::error::ErrorKind) -> Self {
            Error::NomDetail(input.into(), "parse error")
        }

        fn append(_: &[u8], _: nom::error::ErrorKind, other: Self) -> Self {
            other
        }
    }

    pub fn parse(_url: &[u8]) -> Result<borrowed::Url, Error> {
        unimplemented!("parse")
    }
}

#[doc(inline)]
pub use parse::parse;
