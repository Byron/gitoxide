#[test]
fn windows_drive_prefix() {
    use bstr::ByteSlice;

    for drive in ["C", "1", "ä", "֍", "😀"] {
        let prefix = format!("{drive}:");
        for suffix in ["", "/repos/sub", r"\repos\sub", "repos/sub"] {
            let input = format!("{prefix}{suffix}");
            assert_eq!(
                gix_path::windows_drive_prefix(input.as_bytes().as_bstr()),
                Some(prefix.as_bytes().as_bstr()),
                "drive prefixes include a single character and the colon"
            );
        }
    }
    for input in ["", "C", "ä", "😀", "host:repo", "äö:repo", "./a:repo", "../repo"] {
        assert_eq!(
            gix_path::windows_drive_prefix(input.as_bytes().as_bstr()),
            None,
            "{input:?} does not start with a drive prefix"
        );
    }
}

mod is_absolute {
    #[test]
    fn absolute_linux_path_is_true() {
        assert!(gix_path::is_absolute("/"));
        assert!(gix_path::is_absolute("/abs/path"));
    }

    #[test]
    fn relative_linux_path_is_false() {
        assert!(!gix_path::is_absolute("./relative/path"));
        assert!(!gix_path::is_absolute("relative/path"));
    }

    #[cfg(not(windows))]
    mod not_on_windows {
        #[test]
        fn drive_prefixes_are_false() {
            assert!(!gix_path::is_absolute(r"c:\abs/path"));
            assert!(!gix_path::is_absolute(r"c:\abs\path"));
        }
    }

    #[cfg(windows)]
    mod on_windows {
        #[test]
        fn drive_prefixes_are_true() {
            assert!(gix_path::is_absolute(r"c:\abs/path"));
            assert!(gix_path::is_absolute(r"c:\abs\path"));
        }

        #[test]
        fn relative_paths_with_backslashes_are_false() {
            assert!(!gix_path::is_absolute(r".\rel/path"));
            assert!(!gix_path::is_absolute(r"rel\path"));
        }

        #[test]
        fn path_starting_with_backslash_is_false() {
            assert!(!gix_path::is_absolute(r"\rel\path"));
        }
    }
}
