mod convert {
    use bstr::ByteSlice;
    use git_path::{realpath, to_unix_separators, to_windows_separators, RealPathError};
    use std::fs::create_dir_all;
    use std::ops::Deref;
    use std::path::{Path, PathBuf};
    use tempfile::{tempdir, tempdir_in};

    struct CanonicalizedTempDir {
        pub dir: tempfile::TempDir,
    }

    impl CanonicalizedTempDir {
        fn new() -> Self {
            let path = std::env::temp_dir().canonicalize().unwrap();
            let dir = tempdir_in(path).unwrap();
            Self { dir }
        }
    }

    impl Deref for CanonicalizedTempDir {
        type Target = Path;

        fn deref(&self) -> &Self::Target {
            self.dir.path()
        }
    }

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

    #[test]
    fn real_path_tests() {
        let cwd = tempdir().unwrap();
        let cwd = cwd.path();

        let tmp_dir = CanonicalizedTempDir::new();
        let empty_path = Path::new("");
        assert!(
            matches!(realpath(empty_path, cwd, 8).err().unwrap(), RealPathError::EmptyPath),
            "Empty path is error"
        );

        let relative_path = Path::new("b/.git");
        assert_eq!(
            realpath(relative_path, cwd, 8).unwrap(),
            cwd.join("b").join(".git"),
            "relative paths are prefixed with current dir"
        );

        let relative_path = Path::new("b//.git");
        assert_eq!(
            realpath(relative_path, cwd, 8).unwrap(),
            cwd.join("b").join(".git"),
            "empty path components are ignored"
        );

        let dot_path = Path::new("./tmp/.git");
        assert_eq!(
            realpath(dot_path, cwd, 8).unwrap(),
            cwd.join("tmp").join(".git"),
            "path starting with dot is relative and is prefixed with current dir"
        );

        let dot_path_with_dot_in_the_middle = Path::new("./tmp/a/./.git");
        assert_eq!(
            realpath(dot_path_with_dot_in_the_middle, cwd, 8).unwrap(),
            cwd.join("tmp").join("a").join(".git"),
            "dot in middle path components is ignored"
        );

        let dot_dot_path = Path::new("./b/../tmp/.git");
        assert_eq!(
            realpath(dot_dot_path, cwd, 8).unwrap(),
            cwd.join("tmp").join(".git"),
            "dot dot goes to parent path component"
        );

        #[cfg(not(target_os = "windows"))]
        let absolute_path = Path::new("/c/d/.git");
        #[cfg(target_os = "windows")]
        let absolute_path = Path::new("C:\\c\\d\\.git");
        assert_eq!(
            realpath(absolute_path, cwd, 8).unwrap(),
            absolute_path,
            "absolute path without symlinks is resolved to itself, unchanged"
        );

        let link_destination = tmp_dir.join("p").join("q");
        let link = "tmp_p_q_link";
        let root_dir = cwd.join("a").join("b");
        create_dir_all(&root_dir).unwrap();
        create_symlink(root_dir.join(link).as_path(), &link_destination);
        let absolute_path_with_symlink = root_dir.join(link).join(".git");
        assert_eq!(
            realpath(absolute_path_with_symlink.as_path(), cwd, 8).unwrap(),
            link_destination.join(".git"),
            "symlink to absolute path gets expanded"
        );

        let link_destination = PathBuf::from("p").join("q");
        let link = "pq_link";
        create_symlink(cwd.join(link).as_path(), &link_destination);
        let relative_path_with_symlink = PathBuf::from(link).join(".git");
        assert_eq!(
            realpath(relative_path_with_symlink.as_path(), cwd, 8).unwrap(),
            cwd.join("p").join("q").join(".git"),
            "symlink to relative path gets expanded"
        );

        let link_destination = PathBuf::from("x");
        let link = "x_link";
        create_symlink(cwd.join(link).as_path(), &link_destination);
        let relative_path_with_symlink = PathBuf::from(link).join(".git");
        assert!(
            matches!(
                realpath(relative_path_with_symlink.as_path(), cwd, 0).err().unwrap(),
                RealPathError::MaxSymlinksExceeded { max_symlinks: 0 }
            ),
            "max num of symlinks is exceeded"
        );
    }

    #[test]
    #[ignore]
    fn test_prefix_component() {
        // todo!()

        // enum Component.Prefix

        let mut pb = PathBuf::from("/tmp");
        pb.push(std::path::MAIN_SEPARATOR.to_string());

        for c in PathBuf::from("/a/b/c").components() {
            dbg!(c);
        }

        // pass iterator input_path.components() instead of input_path
        // have real_path mut and not return it
        // change error to this_error
    }

    fn create_symlink(link: &Path, link_dest: &Path) {
        #[cfg(not(target_os = "windows"))]
        std::os::unix::fs::symlink(link_dest, &link).unwrap();
        #[cfg(target_os = "windows")]
        std::os::windows::fs::symlink_file(link_dest, &link).unwrap();
    }

    mod absolutize {
        use git_path::absolutize;
        use std::borrow::Cow;
        use std::path::Path;

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
