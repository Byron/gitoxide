mod convert {
    use bstr::ByteSlice;
    use git_path::{real_path, to_unix_separators, to_windows_separators};
    use std::fs::create_dir_all;
    use std::os::unix::fs;
    use std::path::{Path, PathBuf};
    use tempfile::{tempdir, tempdir_in};

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

    // git_repository tests, baseline

    #[test]
    fn real_path_tests() {
        let cwd = tempdir().unwrap();
        let cwd = cwd.path();
        let tmp_dir = tempdir_in("/private/tmp").unwrap();
        let tmp_dir = tmp_dir.path();

        let empty_path = Path::new("");
        assert_eq!(
            real_path(empty_path, cwd),
            Err("Empty is not a valid path".into()),
            "Empty path is error"
        );

        let relative_path = Path::new("b/.git");
        assert_eq!(
            real_path(relative_path, cwd).unwrap(),
            cwd.join("b").join(".git"),
            "relative paths are prefixed with current dir"
        );

        let relative_path = Path::new("b//.git");
        assert_eq!(
            real_path(relative_path, cwd).unwrap(),
            cwd.join("b").join(".git"),
            "empty path components are ignored"
        );

        let dot_path = Path::new("./tmp/.git");
        assert_eq!(
            real_path(dot_path, cwd).unwrap(),
            cwd.join("tmp").join(".git"),
            "path starting with dot is relative and is prefixed with current dir"
        );

        let dot_path_with_dot_in_the_middle = Path::new("./tmp/a/./.git");
        assert_eq!(
            real_path(dot_path_with_dot_in_the_middle, cwd).unwrap(),
            cwd.join("tmp").join("a").join(".git"),
            "dot in middle path components is ignored"
        );

        let dot_dot_path = Path::new("./b/../tmp/.git");
        assert_eq!(
            real_path(dot_dot_path, cwd).unwrap(),
            cwd.join("tmp").join(".git"),
            "dot dot goes to parent path component"
        );

        let absolute_path = Path::new("/c/d/.git");
        assert_eq!(
            real_path(absolute_path, cwd).unwrap(),
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
            real_path(absolute_path_with_symlink.as_path(), cwd).unwrap(),
            link_destination.join(".git"),
            "symlink to absolute path gets expanded"
        );

        let link_destination = PathBuf::from("p").join("q");
        let link = "pq_link";
        create_symlink(cwd.join(link).as_path(), &link_destination);
        let relative_path_with_symlink = PathBuf::from(link).join(".git");
        assert_eq!(
            real_path(relative_path_with_symlink.as_path(), cwd).unwrap(),
            cwd.join("p").join("q").join(".git"),
            "symlink to relative path gets expanded"
        );
    }

    #[test]
    #[ignore]
    fn test_prefix_component() {
        // todo!()

        // enum Component.Prefix

        let a = PathBuf::from("a");
        let mut a = a.components();
        let _ = a.next();

        let mut pb = PathBuf::new();
        pb.push(a.collect::<PathBuf>());

        assert_eq!(pb, PathBuf::new());
        assert_eq!(pb.components().next(), None);

        // pass iterator input_path.components() instead of input_path
        // have real_path mut and not return it
        // change error to this_error
    }

    fn create_symlink(link: &Path, link_dest: &Path) {
        fs::symlink(link_dest, &link).unwrap();
    }
}
