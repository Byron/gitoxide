mod prefix {
    mod from_id {
        use git_hash::{Kind, ObjectId};
        use git_testtools::hex_to_id;

        #[test]
        fn various_valid_inputs() {
            let oid_hex = "abcdefabcdefabcdefabcdefabcdefabcdefabcd";
            let oid = hex_to_id(oid_hex);

            for hex_len in 4..oid.kind().len_in_hex() {
                let mut expected = String::from(&oid_hex[..hex_len]);
                let num_of_zeros = oid.kind().len_in_hex() - hex_len;
                expected.extend(std::iter::repeat('0').take(num_of_zeros));
                let prefix = git_hash::Prefix::new(oid, hex_len).unwrap();
                assert_eq!(prefix.as_oid().to_hex().to_string(), expected, "{}", hex_len);
                assert_eq!(prefix.hex_len(), hex_len);
            }
        }

        #[test]
        fn errors_if_hex_len_is_longer_than_oid_len_in_hex() {
            let kind = Kind::Sha1;
            assert!(matches!(
                git_hash::Prefix::new(ObjectId::null(kind), kind.len_in_hex() + 1),
                Err(git_hash::prefix::Error::TooLong { .. })
            ));
        }

        #[test]
        fn errors_if_hex_len_is_too_short() {
            let kind = Kind::Sha1;
            assert!(matches!(
                git_hash::Prefix::new(ObjectId::null(kind), 3),
                Err(git_hash::prefix::Error::TooShort { .. })
            ));
        }
    }
}

mod short_hex {
    #[test]
    fn display_entire_range_sha1() {
        let id_hex = "0123456789abcdef123456789abcdef123456789";
        let id = git_hash::ObjectId::from_hex(id_hex.as_bytes()).expect("valid input");
        for len in 0..=40 {
            assert_eq!(id.to_hex_with_len(len).to_string(), id_hex[..len]);
        }
        assert_eq!(
            id.to_hex_with_len(120).to_string(),
            id_hex,
            "values that are too long are truncated"
        );
    }
}
