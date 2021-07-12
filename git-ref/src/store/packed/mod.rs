#![allow(missing_docs, dead_code)]

pub mod iter {}

#[derive(Debug, PartialEq, Eq)]
enum Peeled {
    Partial,
    Fully,
}

/// Information parsed from the header of a packed ref file
#[derive(Debug, PartialEq, Eq)]
struct Header {
    peeled: Peeled,
    sorted: bool,
}

mod decode {
    use crate::parse::newline;
    use crate::store::packed::Header;
    use bstr::{BStr, ByteSlice};
    use nom::bytes::complete::{tag, take_until, take_while};
    use nom::combinator::opt;
    use nom::error::VerboseError;
    use nom::sequence::{delimited, preceded, terminated, tuple};
    use nom::{error::ParseError, IResult};

    fn header<'a, E>(input: &'a [u8]) -> IResult<&'a [u8], Header, E>
    where
        E: ParseError<&'a [u8]>,
    {
        let (rest, traits) = delimited(
            tuple((
                opt(take_while(|c: u8| c.is_ascii_whitespace())),
                tag(b"# pack-refs with: "),
            )),
            take_until("\n"),
            newline,
        )(input)?;
        dbg!(traits.as_bstr());
        todo!("parse a header line")
    }

    fn to_bstr_err(err: nom::Err<VerboseError<&[u8]>>) -> VerboseError<&BStr> {
        let err = match err {
            nom::Err::Error(err) | nom::Err::Failure(err) => err,
            nom::Err::Incomplete(_) => unreachable!("not a streaming parser"),
        };
        VerboseError {
            errors: err.errors.into_iter().map(|(i, v)| (i.as_bstr(), v)).collect(),
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::store::packed::Peeled;
        use bstr::ByteSlice;

        #[test]
        fn valid_header_fully_peeled_stored() {
            let input: &[u8] = b"  # pack-refs with: peeled fully-peeled sorted  \nsomething else";
            let (rest, header) = header::<nom::error::VerboseError<_>>(input)
                .map_err(to_bstr_err)
                .unwrap();
            assert_eq!(rest.as_bstr(), "something else", "remainder starts after newline");
            assert_eq!(
                header,
                Header {
                    peeled: Peeled::Fully,
                    sorted: true
                }
            );
        }

        #[test]
        fn valid_header_peeled_unsorted() {
            let input: &[u8] = b"\n\n# pack-refs with: peeled\n";
            let (rest, header) = header::<()>(input).unwrap();
            assert!(rest.is_empty());
            assert_eq!(
                header,
                Header {
                    peeled: Peeled::Partial,
                    sorted: false
                }
            );
        }
    }
}
