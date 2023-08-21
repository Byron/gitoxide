type Result = std::result::Result<(), Box<dyn std::error::Error>>;

mod reference {
    use winnow::{error::TreeError, prelude::*};

    use super::Result;
    use crate::{
        store_impl::{packed, packed::decode},
        FullNameRef,
    };

    /// Convert a hexadecimal hash into its corresponding `ObjectId` or _panic_.
    fn hex_to_id(hex: &str) -> gix_hash::ObjectId {
        gix_hash::ObjectId::from_hex(hex.as_bytes()).expect("40 bytes hex")
    }

    #[test]
    fn invalid() {
        assert!(decode::reference::<()>
            .parse_peek(b"# what looks like a comment",)
            .is_err());
        assert!(
            decode::reference::<()>
                .parse_peek(b"^e9cdc958e7ce2290e2d7958cdb5aa9323ef35d37\n",)
                .is_err(),
            "lonely peel"
        );
    }

    #[test]
    fn two_refs_in_a_row() -> Result {
        let input: &[u8] = b"d53c4b0f91f1b29769c9430f2d1c0bcab1170c75 refs/heads/alternates-after-packs-and-loose
^e9cdc958e7ce2290e2d7958cdb5aa9323ef35d37\neaae9c1bc723209d793eb93f5587fa2604d5cd92 refs/heads/avoid-double-lookup\n";
        let (input, parsed) = decode::reference::<TreeError<_>>.parse_peek(input).unwrap();

        assert_eq!(
            parsed,
            packed::Reference {
                name: FullNameRef::new_unchecked("refs/heads/alternates-after-packs-and-loose".into()),
                target: "d53c4b0f91f1b29769c9430f2d1c0bcab1170c75".into(),
                object: Some("e9cdc958e7ce2290e2d7958cdb5aa9323ef35d37".into())
            }
        );
        assert_eq!(parsed.target(), hex_to_id("d53c4b0f91f1b29769c9430f2d1c0bcab1170c75"));
        assert_eq!(parsed.object(), hex_to_id("e9cdc958e7ce2290e2d7958cdb5aa9323ef35d37"));

        let (input, parsed) = decode::reference::<TreeError<_>>.parse_peek(input).unwrap();
        assert!(input.is_empty(), "exhausted");
        assert_eq!(
            parsed.name,
            FullNameRef::new_unchecked("refs/heads/avoid-double-lookup".into())
        );
        assert_eq!(parsed.target, "eaae9c1bc723209d793eb93f5587fa2604d5cd92");
        assert!(parsed.object.is_none());
        Ok(())
    }
}

mod header {
    use gix_object::bstr::ByteSlice;
    use gix_testtools::to_bstr_err;
    use winnow::prelude::*;

    use super::Result;
    use crate::store_impl::packed::{
        decode,
        decode::{Header, Peeled},
    };

    #[test]
    fn invalid() {
        assert!(
            decode::header::<()>.parse_peek(b"# some user comment").is_err(),
            "something the user put there"
        );
        assert!(
            decode::header::<()>.parse_peek(b"# pack-refs: ").is_err(),
            "looks right but isn't"
        );
        assert!(
            decode::header::<()>.parse_peek(b" # pack-refs with: ").is_err(),
            "does not start with #"
        );
    }

    #[test]
    fn valid_fully_peeled_stored() -> Result {
        let input: &[u8] = b"# pack-refs with: peeled fully-peeled sorted  \nsomething else";
        let (rest, header) = decode::header::<winnow::error::TreeError<_, _>>
            .parse_peek(input)
            .map_err(to_bstr_err)?;

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
        let (rest, header) = decode::header::<()>.parse_peek(input).unwrap();

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
        let (rest, header) = decode::header::<()>.parse_peek(input).unwrap();

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
