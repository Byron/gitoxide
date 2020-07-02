use nom::error::ParseError;
use quick_error::quick_error;
quick_error! {
    #[derive(Debug)]
    pub enum Error {
        ParseIntegerError(msg: &'static str, number: crate::BytesOwned, err: btoi::ParseIntegerError) {
            display("{}: {:?}", msg, number)
            cause(err)
        }
        Nom(err_msg: String) {
            display("{}", err_msg)
        }
        NomDetail(input: crate::BytesOwned, msg: &'static str) {
            display("{}: '{}' could not be parsed", msg, input)
        }
        ParseKindError(err: crate::types::Error) {
            display("{}", err)
            cause(err)
        }
        ObjectKind(err: crate::Error) {
            from()
            cause(err)
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
