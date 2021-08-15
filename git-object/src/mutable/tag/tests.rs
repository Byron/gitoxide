mod validated_name {
    mod invalid {
        use bstr::ByteSlice;

        use super::super::super::*;

        #[test]
        fn only_dash() {
            assert!(validated_name(b"-".as_bstr()).is_err())
        }
        #[test]
        fn leading_dash() {
            assert!(validated_name(b"-hello".as_bstr()).is_err())
        }
    }

    mod valid {
        use bstr::ByteSlice;

        use super::super::super::*;

        #[test]
        fn version() {
            for version in &["v1.0.0", "0.2.1", "0-alpha1"] {
                assert!(validated_name(version.as_bytes().as_bstr()).is_ok())
            }
        }
    }
}
