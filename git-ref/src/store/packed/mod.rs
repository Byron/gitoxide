#![allow(missing_docs, dead_code)]

pub mod iter {}

#[derive(Debug, PartialEq, Eq)]
enum Peeled {
    Unspecified,
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
    use crate::{
        parse::newline,
        store::{packed::Header, packed::Peeled},
    };
    use bstr::ByteSlice;
    use nom::{
        bytes::complete::{tag, take_until, take_while},
        combinator::opt,
        error::ParseError,
        sequence::{delimited, tuple},
        IResult,
    };

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

        let mut peeled = Peeled::Unspecified;
        let mut sorted = false;
        for token in traits.as_bstr().split_str(b" ") {
            if token == b"fully-peeled" {
                peeled = Peeled::Fully;
            } else if token == b"peeled" {
                peeled = Peeled::Partial;
            } else if token == b"sorted" {
                sorted = true;
            }
        }

        Ok((rest, Header { peeled, sorted }))
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::store::packed::Peeled;
        use bstr::ByteSlice;
        use git_testtools::to_bstr_err;

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

        #[test]
        fn valid_header_empty() {
            let input: &[u8] = b"\n\n# pack-refs with: \n";
            let (rest, header) = header::<()>(input).unwrap();
            assert!(rest.is_empty());
            assert_eq!(
                header,
                Header {
                    peeled: Peeled::Unspecified,
                    sorted: false
                }
            );
        }
    }
}
