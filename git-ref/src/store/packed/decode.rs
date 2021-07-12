use crate::{
    parse::newline,
    store::{packed, packed::Peeled},
};
use bstr::ByteSlice;
use nom::{
    bytes::complete::{tag, take_until, take_while},
    combinator::opt,
    error::ParseError,
    sequence::{delimited, tuple},
    IResult,
};

fn header<'a, E>(input: &'a [u8]) -> IResult<&'a [u8], packed::Header, E>
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

    Ok((rest, packed::Header { peeled, sorted }))
}

fn reference<'a, E: ParseError<&'a [u8]>>(input: &[u8]) -> IResult<&[u8], packed::Reference<'a>, E> {
    todo!("line parsing")
}

#[cfg(test)]
mod tests {
    type Result = std::result::Result<(), Box<dyn std::error::Error>>;

    mod reference {
        use crate::store::packed::decode;
        use bstr::ByteSlice;
        use git_testtools::hex_to_id;
        use nom::error::VerboseError;

        #[test]
        #[ignore]
        fn two_refs_in_a_row() {
            let input: &[u8] = b"d53c4b0f91f1b29769c9430f2d1c0bcab1170c75 refs/heads/alternates-after-packs-and-loose\n^e9cdc958e7ce2290e2d7958cdb5aa9323ef35d37\neaae9c1bc723209d793eb93f5587fa2604d5cd92 refs/heads/avoid-double-lookup\n";
            let (input, parsed) = decode::reference::<VerboseError<_>>(input).unwrap();
            assert_eq!(parsed.full_name, "refs/heads/alternates-after-packs-and-loose");
            assert_eq!(parsed.target(), hex_to_id("d53c4b0f91f1b29769c9430f2d1c0bcab1170c75"));
            assert_eq!(
                parsed.object,
                Some(b"e9cdc958e7ce2290e2d7958cdb5aa9323ef35d37".as_bstr())
            );
            assert_eq!(parsed.object(), hex_to_id("e9cdc958e7ce2290e2d7958cdb5aa9323ef35d37"));

            let (input, parsed) = decode::reference::<VerboseError<_>>(input).unwrap();
            assert!(input.is_empty(), "exhausted");
            assert_eq!(parsed.full_name, "refs/heads/avoid-double-lookup");
            assert_eq!(parsed.target, "e9cdc958e7ce2290e2d7958cdb5aa9323ef35d37");
            assert!(parsed.object.is_none());
        }
    }
    mod header {
        use super::Result;
        use crate::store::packed::{decode, Header, Peeled};
        use bstr::ByteSlice;
        use git_testtools::to_bstr_err;

        #[test]
        fn invalid() {
            assert!(
                decode::header::<()>(b"# some user comment").is_err(),
                "something the user put there"
            );
            assert!(decode::header::<()>(b"# pack-refs: ").is_err(), "looks right but isn't");
        }

        #[test]
        fn valid_fully_peeled_stored_with_leading_spaces() -> Result {
            let input: &[u8] = b"  # pack-refs with: peeled fully-peeled sorted  \nsomething else";
            let (rest, header) = decode::header::<nom::error::VerboseError<_>>(input).map_err(to_bstr_err)?;
            assert_eq!(rest.as_bstr(), "something else", "remainder starts after newline");
            assert_eq!(
                header,
                Header {
                    peeled: Peeled::Fully,
                    sorted: true
                }
            );
            Ok(())
        }

        #[test]
        fn valid_peeled_unsorted_with_leading_newlines() -> Result {
            let input: &[u8] = b"\n\n# pack-refs with: peeled\n";
            let (rest, header) = decode::header::<()>(input)?;
            assert!(rest.is_empty());
            assert_eq!(
                header,
                Header {
                    peeled: Peeled::Partial,
                    sorted: false
                }
            );
            Ok(())
        }

        #[test]
        fn valid_empty_without_leading_bytes() -> Result {
            let input: &[u8] = b"# pack-refs with: \n";
            let (rest, header) = decode::header::<()>(input)?;
            assert!(rest.is_empty());
            assert_eq!(
                header,
                Header {
                    peeled: Peeled::Unspecified,
                    sorted: false
                }
            );
            Ok(())
        }
    }
}
