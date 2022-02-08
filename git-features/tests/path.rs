mod bytes {
    use bstr::ByteSlice;
    use git_features::path;

    #[test]
    fn backslash_to_slash() {
        assert_eq!(
            path::bytes::backslash_to_slash(b"no-backslash".as_ref()).as_bstr(),
            "no-backslash"
        );

        assert_eq!(
            path::bytes::backslash_to_slash(b"\\a\\b\\\\".as_ref()).as_bstr(),
            "/a/b//"
        );
    }
}
