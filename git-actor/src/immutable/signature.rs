use bstr::BStr;

use crate::{immutable::parse, Time};

/// A signature is created by an actor at a certain time.
///
/// Note that this is not a cryptographical signature.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Signature<'a> {
    /// The actor's name.
    #[cfg_attr(feature = "serde1", serde(borrow))]
    pub name: &'a BStr,
    /// The actor's email.
    pub email: &'a BStr,
    /// The time stamp at which the signature was performed.
    pub time: Time,
}

impl<'a> Signature<'a> {
    /// Deserialize a signature from the given `data`.
    pub fn from_bytes(data: &'a [u8]) -> Result<Signature<'a>, decode::Error> {
        parse::signature(data).map(|(_, t)| t).map_err(decode::Error::from)
    }
}

///
pub mod decode {
    use bstr::BString;
    use nom::error::ParseError;
    use quick_error::quick_error;

    quick_error! {
        /// An error returned by various [`Commit`][crate::immutable::Commit] and [`Signature`][crate::immutable::Signature] methods
        /// when they couldn't be decoded.
        #[derive(Debug, Clone)]
        #[allow(missing_docs)]
        pub enum Error {
            ParseIntegerError(msg: &'static str, number: BString, err: btoi::ParseIntegerError) {
                display("{}: {:?}", msg, number)
                source(err)
            }
            Nom(err_msg: String) {
                display("{}", err_msg)
            }
            NomDetail(input: BString, msg: &'static str) {
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

    impl From<nom::Err<Error>> for Error {
        fn from(e: nom::Err<Error>) -> Self {
            match e {
                nom::Err::Error(err) | nom::Err::Failure(err) => Error::Nom(err.to_string()),
                nom::Err::Incomplete(_) => unreachable!("we do not implement streaming parsers"),
            }
        }
    }
}
