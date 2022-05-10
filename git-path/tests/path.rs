mod convert {
    use bstr::ByteSlice;
    use git_path::{real_path, to_unix_separators, to_windows_separators, RealPathError};
    use std::fs::{create_dir_all, remove_dir_all};
    use std::ops::Deref;
    #[cfg(not(target_os = "windows"))]
    use std::os::unix::fs;
    #[cfg(target_os = "windows")]
    use std::os::windows::fs;
    use std::path::{Path, PathBuf};
    use tempfile::{tempdir, tempdir_in};

    struct TempDirIn<'a> {
        pub path: &'a Path,
    }

    impl<'a> TempDirIn<'a> {
        fn new(path: &'a str) -> Self {
            let path = Path::new(path);
            create_dir_all(path).unwrap();
            tempdir_in(path).unwrap();
            Self { path }
        }
    }

    impl Deref for TempDirIn<'_> {
        type Target = Path;

        fn deref(&self) -> &Self::Target {
            self.path
        }
    }

    impl Drop for TempDirIn<'_> {
        fn drop(&mut self) {
            remove_dir_all(&self.path).unwrap();
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

        #[cfg(not(target_os = "windows"))]
        let tmp_dir = TempDirIn::new("/private/tmp/t");
        #[cfg(target_os = "windows")]
        let tmp_dir = TempDirIn::new("%localappdata%\\Temp\\t");

        let empty_path = Path::new("");
        assert!(
            matches!(real_path(empty_path, cwd, 8).err().unwrap(), RealPathError::EmptyPath),
            "Empty path is error"
        );

        let relative_path = Path::new("b/.git");
        assert_eq!(
            real_path(relative_path, cwd, 8).unwrap(),
            cwd.join("b").join(".git"),
            "relative paths are prefixed with current dir"
        );

        let relative_path = Path::new("b//.git");
        assert_eq!(
            real_path(relative_path, cwd, 8).unwrap(),
            cwd.join("b").join(".git"),
            "empty path components are ignored"
        );

        let dot_path = Path::new("./tmp/.git");
        assert_eq!(
            real_path(dot_path, cwd, 8).unwrap(),
            cwd.join("tmp").join(".git"),
            "path starting with dot is relative and is prefixed with current dir"
        );

        let dot_path_with_dot_in_the_middle = Path::new("./tmp/a/./.git");
        assert_eq!(
            real_path(dot_path_with_dot_in_the_middle, cwd, 8).unwrap(),
            cwd.join("tmp").join("a").join(".git"),
            "dot in middle path components is ignored"
        );

        let dot_dot_path = Path::new("./b/../tmp/.git");
        assert_eq!(
            real_path(dot_dot_path, cwd, 8).unwrap(),
            cwd.join("tmp").join(".git"),
            "dot dot goes to parent path component"
        );

        let absolute_path = Path::new("/c/d/.git");
        assert_eq!(
            real_path(absolute_path, cwd, 8).unwrap(),
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
            real_path(absolute_path_with_symlink.as_path(), cwd, 8).unwrap(),
            link_destination.join(".git"),
            "symlink to absolute path gets expanded"
        );

        let link_destination = PathBuf::from("p").join("q");
        let link = "pq_link";
        create_symlink(cwd.join(link).as_path(), &link_destination);
        let relative_path_with_symlink = PathBuf::from(link).join(".git");
        assert_eq!(
            real_path(relative_path_with_symlink.as_path(), cwd, 8).unwrap(),
            cwd.join("p").join("q").join(".git"),
            "symlink to relative path gets expanded"
        );

        let link_destination = PathBuf::from("x");
        let link = "x_link";
        create_symlink(cwd.join(link).as_path(), &link_destination);
        let relative_path_with_symlink = PathBuf::from(link).join(".git");
        assert!(
            matches!(
                real_path(relative_path_with_symlink.as_path(), cwd, 0).err().unwrap(),
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

        let tmp_dir_in = TempDirIn::new("/tmp/a/b/s");
        let p = tmp_dir_in.path;
        assert!(p.exists());
        assert_eq!(p, Path::new("/tmp/a/b/s"));
        drop(tmp_dir_in);
        assert!(!p.exists());

        // pass iterator input_path.components() instead of input_path
        // have real_path mut and not return it
        // change error to this_error
    }

    fn create_symlink(link: &Path, link_dest: &Path) {
        #[cfg(not(target_os = "windows"))]
        fs::symlink(link_dest, &link).unwrap();
        #[cfg(target_os = "windows")]
        fs::symlink_file(link_dest, &link).unwrap();
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
