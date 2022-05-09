mod convert {
    use bstr::ByteSlice;
    use git_path::{to_unix_separators, to_windows_separators};

    #[test]
    fn assure_unix_separators() {
        assert_eq!(to_unix_separators(b"no-backslash".as_bstr()).as_bstr(), "no-backslash");

        assert_eq!(to_unix_separators(b"\\a\\b\\\\".as_bstr()).as_bstr(), "/a/b//");
    }

    #[test]
    fn assure_windows_separators() {
        assert_eq!(
            to_windows_separators(b"no-backslash".as_bstr()).as_bstr(),
            "no-backslash"
        );

        assert_eq!(to_windows_separators(b"/a/b//".as_bstr()).as_bstr(), "\\a\\b\\\\");
    }

    mod absolutize_trailing_components {
        use git_path::absolutize_components;
        use std::borrow::Cow;
        use std::path::Path;

        fn p(input: &str) -> &Path {
            Path::new(input)
        }

        #[test]
        fn no_change_if_there_are_no_trailing_relative_components() {
            for input in ["./a/b/c/d", "/absolute/path", "C:\\hello\\world"] {
                let path = p(input);
                assert_eq!(absolutize_components(path), path);
            }
        }

        #[test]
        fn trailing_relative_components_are_resolved() {
            for (input, expected) in [
                ("./a/b/./c/../d/..", "./a/b"),
                ("/a/b/c/.././../.", "/a"),
                ("/a/b/../../..", "/"),
                ("/a/./b/c/.././../.", "/a"),
                ("/a/././c/.././../.", "/"),
                ("/a/b/../c/../..", "/"),
                ("C:/hello/../a", "C:/a"),
                ("./a/../b/..", "./"),
                ("/a/../b", "/b"),
            ] {
                let path = p(input);
                assert_eq!(
                    absolutize_components(path),
                    Cow::Borrowed(p(expected)),
                    "'{}' got an unexpected result",
                    input
                );
            }
        }
    }
}
