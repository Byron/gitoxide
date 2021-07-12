use crate::parse::hex_sha;
use crate::{
    parse::newline,
    store::{packed, packed::Peeled},
};
use bstr::{BStr, ByteSlice};
use nom::combinator::map;
use nom::sequence::{preceded, terminated};
use nom::{
    bytes::complete::{tag, take_while},
    combinator::opt,
    error::ParseError,
    sequence::{delimited, tuple},
    IResult,
};

fn until_newline<'a, E>(input: &'a [u8]) -> IResult<&'a [u8], &'a BStr, E>
where
    E: ParseError<&'a [u8]>,
{
    map(
        terminated(take_while(|b: u8| b != b'\r' && b != b'\n'), newline),
        |not_newline| not_newline.as_bstr(),
    )(input)
}

fn header<'a, E>(input: &'a [u8]) -> IResult<&'a [u8], packed::Header, E>
where
    E: ParseError<&'a [u8]>,
{
    let (rest, traits) = preceded(tag(b"# pack-refs with: "), until_newline)(input)?;

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

fn reference<'a, E: ParseError<&'a [u8]>>(
    input: &'a [u8],
    hash: git_hash::Kind,
) -> IResult<&'a [u8], packed::Reference<'a>, E> {
    let (input, (target, full_name)) = tuple((terminated(hex_sha(hash), tag(b" ")), until_newline))(input)?;
    let (rest, object) = opt(delimited(tag(b"^"), hex_sha(hash), newline))(input)?;
    Ok((
        rest,
        packed::Reference {
            full_name,
            target,
            object,
        },
    ))
}

#[cfg(test)]
mod tests {
    type Result = std::result::Result<(), Box<dyn std::error::Error>>;

    mod reference {
        use crate::store::packed::decode;
        use bstr::ByteSlice;
        use git_hash::Kind::Sha1;
        use git_testtools::hex_to_id;
        use nom::error::VerboseError;

        #[test]
        fn invalid() {
            assert!(decode::reference::<()>(b"# what looks like a comment", Sha1).is_err());
            assert!(
                decode::reference::<()>(b"^e9cdc958e7ce2290e2d7958cdb5aa9323ef35d37\n", Sha1).is_err(),
                "lonely peel"
            );
        }

        #[test]
        fn two_refs_in_a_row() {
            let input: &[u8] = b"d53c4b0f91f1b29769c9430f2d1c0bcab1170c75 refs/heads/alternates-after-packs-and-loose\n^e9cdc958e7ce2290e2d7958cdb5aa9323ef35d37\neaae9c1bc723209d793eb93f5587fa2604d5cd92 refs/heads/avoid-double-lookup\n";
            let (input, parsed) = decode::reference::<VerboseError<_>>(input, Sha1).unwrap();

            assert_eq!(parsed.full_name, "refs/heads/alternates-after-packs-and-loose");
            assert_eq!(parsed.target(), hex_to_id("d53c4b0f91f1b29769c9430f2d1c0bcab1170c75"));
            assert_eq!(
                parsed.object,
                Some(b"e9cdc958e7ce2290e2d7958cdb5aa9323ef35d37".as_bstr())
            );
            assert_eq!(parsed.object(), hex_to_id("e9cdc958e7ce2290e2d7958cdb5aa9323ef35d37"));

            let (input, parsed) = decode::reference::<VerboseError<_>>(input, Sha1).unwrap();
            assert!(input.is_empty(), "exhausted");
            assert_eq!(parsed.full_name, "refs/heads/avoid-double-lookup");
            assert_eq!(parsed.target, "eaae9c1bc723209d793eb93f5587fa2604d5cd92");
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
            assert!(
                decode::header::<()>(b" # pack-refs with: ").is_err(),
                "does not start with #"
            );
        }

        #[test]
        fn valid_fully_peeled_stored() -> Result {
            let input: &[u8] = b"# pack-refs with: peeled fully-peeled sorted  \nsomething else";
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
        fn valid_peeled_unsorted() -> Result {
            let input: &[u8] = b"# pack-refs with: peeled\n";
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
        fn valid_empty() -> Result {
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
