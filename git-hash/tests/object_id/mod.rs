mod from_hex {

    mod valid {
        use git_hash::ObjectId;

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
        use git_hash::{decode, ObjectId};

        #[test]
        fn non_hex_characters() {
            assert!(matches!(
                ObjectId::from_hex(b"zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz").unwrap_err(),
                decode::Error::Invalid { index: 0, c: 'z' }
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
