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

    mod absolutize {
        use std::{borrow::Cow, path::Path};

        use git_path::absolutize;

        fn p(input: &str) -> &Path {
            Path::new(input)
        }

        #[test]
        fn no_change_if_there_are_no_trailing_relative_components() {
            for input in ["./a/b/c/d", "/absolute/path", "C:\\hello\\world"] {
                let path = p(input);
                assert_eq!(absolutize(path, None::<&Path>), path);
            }
        }

        #[test]
        fn special_cases_around_cwd() {
            let cwd = std::env::current_dir().unwrap();
            assert_eq!(
                absolutize(p("a/.."), None::<&Path>),
                p("."),
                "empty paths are never returned as they are invalid"
            );
            assert_eq!(
                absolutize(p("a/../.."), Some(&cwd)),
                cwd.parent().unwrap(),
                "it automatically extends the poppable items by using the current working dir"
            );
        }

        #[test]
        fn trailing_relative_components_are_resolved() {
            for (input, expected) in [
                ("./a/b/./c/../d/..", "./a/b"),
                ("/a/b/c/.././../.", "/a"),
                ("./a/..", "."),
                ("a/..", "."),
                ("./a/b/../../..", "."),
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
                    absolutize(path, None::<&Path>),
                    Cow::Borrowed(p(expected)),
                    "'{}' got an unexpected result",
                    input
                );
            }
        }
    }
}
