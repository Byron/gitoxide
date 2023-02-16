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
            assert!(!gix_path::is_absolute("c:\\abs/path"));
            assert!(!gix_path::is_absolute("c:\\abs\\path"));
        }
    }

    #[cfg(windows)]
    mod on_windows {
        #[test]
        fn drive_prefixes_are_true() {
            assert!(gix_path::is_absolute("c:\\abs/path"));
            assert!(gix_path::is_absolute("c:\\abs\\path"));
        }

        #[test]
        fn relative_paths_with_backslashes_are_false() {
            assert!(!gix_path::is_absolute(".\\rel/path"));
            assert!(!gix_path::is_absolute("rel\\path"));
        }

        #[test]
        fn path_starting_with_backslash_is_false() {
            assert!(!gix_path::is_absolute("\\rel\\path"));
        }
    }
}
