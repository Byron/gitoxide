type Result = std::result::Result<(), Box<dyn std::error::Error>>;

mod reference {
    use super::Result;
    use crate::store::{packed, packed::decode};
    use crate::FullName;
    use git_testtools::hex_to_id;
    use nom::error::VerboseError;

    #[test]
    fn invalid() {
        assert!(decode::reference::<()>(b"# what looks like a comment",).is_err());
        assert!(
            decode::reference::<()>(b"^e9cdc958e7ce2290e2d7958cdb5aa9323ef35d37\n",).is_err(),
            "lonely peel"
        );
    }

    #[test]
    fn two_refs_in_a_row() -> Result {
        let input: &[u8] = b"d53c4b0f91f1b29769c9430f2d1c0bcab1170c75 refs/heads/alternates-after-packs-and-loose
^e9cdc958e7ce2290e2d7958cdb5aa9323ef35d37\neaae9c1bc723209d793eb93f5587fa2604d5cd92 refs/heads/avoid-double-lookup\n";
        let (input, parsed) = decode::reference::<VerboseError<_>>(input)?;

        assert_eq!(
            parsed,
            packed::Reference {
                name: FullName("refs/heads/alternates-after-packs-and-loose".into()),
                target: "d53c4b0f91f1b29769c9430f2d1c0bcab1170c75".into(),
                object: Some("e9cdc958e7ce2290e2d7958cdb5aa9323ef35d37".into())
            }
        );
        assert_eq!(parsed.target(), hex_to_id("d53c4b0f91f1b29769c9430f2d1c0bcab1170c75"));
        assert_eq!(parsed.object(), hex_to_id("e9cdc958e7ce2290e2d7958cdb5aa9323ef35d37"));

        let (input, parsed) = decode::reference::<VerboseError<_>>(input)?;
        assert!(input.is_empty(), "exhausted");
        assert_eq!(parsed.name, FullName("refs/heads/avoid-double-lookup".into()));
        assert_eq!(parsed.target, "eaae9c1bc723209d793eb93f5587fa2604d5cd92");
        assert!(parsed.object.is_none());
        Ok(())
    }
}

mod header {
    use super::Result;
    use crate::store::packed::{
        decode,
        decode::{Header, Peeled},
    };
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
