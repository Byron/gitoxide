mod bytes {
    use bstr::ByteSlice;
    use git_features::path;

    #[test]
    fn assure_unix_separators() {
        assert_eq!(
            path::convert::to_unix_separators(b"no-backslash".as_ref()).as_bstr(),
            "no-backslash"
        );

        assert_eq!(
            path::convert::to_unix_separators(b"\\a\\b\\\\".as_ref()).as_bstr(),
            "/a/b//"
        );
    }

    #[test]
    fn assure_windows_separators() {
        assert_eq!(
            path::convert::to_windows_separators(b"no-backslash".as_ref()).as_bstr(),
            "no-backslash"
        );

        assert_eq!(
            path::convert::to_windows_separators(b"/a/b//".as_ref()).as_bstr(),
            "\\a\\b\\\\"
        );
    }
}
