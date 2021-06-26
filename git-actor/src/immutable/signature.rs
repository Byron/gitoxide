///
pub mod decode {
    use bstr::{BString, ByteSlice};
    use nom::error::{ContextError, ParseError};
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
        }
    }

    impl ParseError<&[u8]> for Error {
        fn from_error_kind(input: &[u8], kind: nom::error::ErrorKind) -> Self {
            Error::Nom(format!("{:?} failed at: {}", input.to_str_lossy(), kind.description()))
        }

        fn append(_: &[u8], _: nom::error::ErrorKind, other: Self) -> Self {
            other
        }
    }

    impl ContextError<&[u8]> for Error {
        fn add_context(input: &[u8], ctx: &'static str, _other_usually_internal_ignored: Self) -> Self {
            Error::Nom(format!("{:?} did not match '{}'", input.to_str_lossy(), ctx))
        }
    }

    impl From<nom::Err<Error>> for Error {
        fn from(e: nom::Err<Error>) -> Self {
            match e {
                nom::Err::Error(err) | nom::Err::Failure(err) => err,
                nom::Err::Incomplete(_) => unreachable!("we do not implement streaming parsers"),
            }
        }
    }
}
