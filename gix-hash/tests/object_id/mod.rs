mod from_hex {

    mod valid {
        use gix_hash::ObjectId;

        #[test]
        fn twenty_hex_chars_lowercase() {
            assert!(ObjectId::from_hex(b"1234567890abcdefaaaaaaaaaaaaaaaaaaaaaaaa").is_ok());
        }

        #[test]
        fn twenty_hex_chars_uppercase() {
            assert!(ObjectId::from_hex(b"1234567890ABCDEFAAAAAAAAAAAAAAAAAAAAAAAA").is_ok());
        }
    }

    mod invalid {
        use gix_hash::{decode, ObjectId};

        #[test]
        fn non_hex_characters() {
            assert!(matches!(
                ObjectId::from_hex(b"zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz").unwrap_err(),
                decode::Error::Invalid
            ));
        }

        #[test]
        fn too_short() {
            assert!(matches!(
                ObjectId::from_hex(b"abcd").unwrap_err(),
                decode::Error::InvalidHexEncodingLength(4)
            ));
        }
        #[test]
        fn too_long() {
            assert!(matches!(
                ObjectId::from_hex(b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaf").unwrap_err(),
                decode::Error::InvalidHexEncodingLength(41)
            ));
        }
    }
}

mod empty {
    use gix_features::hash::hasher;
    use gix_hash::{Kind, ObjectId};

    fn hash_contents(s: &[u8]) -> ObjectId {
        let mut hasher = hasher(Kind::Sha1);
        hasher.update(s);
        ObjectId::Sha1(hasher.digest())
    }

    #[test]
    fn blob() {
        assert_eq!(ObjectId::empty_blob(Kind::Sha1), hash_contents(b"blob 0\0"));
    }

    #[test]
    fn tree() {
        assert_eq!(ObjectId::empty_tree(Kind::Sha1), hash_contents(b"tree 0\0"));
    }
}
