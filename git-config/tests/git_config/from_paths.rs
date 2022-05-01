use std::{borrow::Cow, fs, io};

use crate::git_config::cow_str;
use git_config::{
    file::{from_paths::Error, GitConfig},
    parser::ParserOrIoError,
};
use tempfile::tempdir;

/// Escapes backslash when writing a path as string so that it is a valid windows path
fn escape_backslashes(path: &std::path::Path) -> String {
    path.to_str().unwrap().replace('\\', "\\\\")
}

#[test]
fn file_not_found() {
    let dir = tempdir().unwrap();
    let config_path = dir.path().join("config");

    let paths = vec![config_path];
    let error = GitConfig::from_paths(paths, &Default::default()).unwrap_err();
    assert!(
        matches!(error,  Error::ParserOrIoError(ParserOrIoError::Io(io_error)) if io_error.kind() == io::ErrorKind::NotFound)
    );
}

#[test]
fn single_path() {
    let dir = tempdir().unwrap();
    let config_path = dir.path().join("config");
    fs::write(config_path.as_path(), b"[core]\nboolean = true").unwrap();

    let paths = vec![config_path];
    let config = GitConfig::from_paths(paths, &Default::default()).unwrap();

    assert_eq!(
        config.raw_value("core", None, "boolean").unwrap(),
        Cow::<[u8]>::Borrowed(b"true")
    );

    assert_eq!(config.len(), 1);
}

#[test]
fn multiple_paths_single_value() -> crate::Result {
    let dir = tempdir()?;

    let a_path = dir.path().join("a");
    fs::write(a_path.as_path(), b"[core]\na = true")?;

    let b_path = dir.path().join("b");
    fs::write(b_path.as_path(), b"[core]\nb = true")?;

    let c_path = dir.path().join("c");
    fs::write(c_path.as_path(), b"[core]\nc = true")?;

    let d_path = dir.path().join("d");
    fs::write(d_path.as_path(), b"[core]\na = false")?;

    let paths = vec![a_path, b_path, c_path, d_path];
    let config = GitConfig::from_paths(paths, &Default::default())?;

    assert_eq!(config.boolean("core", None, "a"), Some(Ok(false)));
    assert_eq!(config.boolean("core", None, "b"), Some(Ok(true)));
    assert_eq!(config.boolean("core", None, "c"), Some(Ok(true)));
    assert_eq!(config.len(), 4);

    Ok(())
}

#[test]
fn multiple_paths_multi_value() -> crate::Result {
    let dir = tempdir()?;

    let a_path = dir.path().join("a");
    fs::write(a_path.as_path(), b"[core]\nkey = a")?;

    let b_path = dir.path().join("b");
    fs::write(b_path.as_path(), b"[core]\nkey = b")?;

    let c_path = dir.path().join("c");
    fs::write(c_path.as_path(), b"[core]\nkey = c")?;

    let d_path = dir.path().join("d");
    fs::write(d_path.as_path(), b"[include]\npath = d_path")?;

    let e_path = dir.path().join("e");
    fs::write(e_path.as_path(), b"[include]\npath = e_path")?;

    let paths = vec![a_path, b_path, c_path, d_path, e_path];
    let config = GitConfig::from_paths(paths, &Default::default())?;

    assert_eq!(
        config.strings("core", None, "key"),
        Some(vec![cow_str("a"), cow_str("b"), cow_str("c"),])
    );

    assert_eq!(
        config.strings("include", None, "path"),
        Some(vec![cow_str("d_path"), cow_str("e_path")])
    );

    assert_eq!(config.len(), 5);
    Ok(())
}

mod includes {
    mod unconditional {
        use std::{borrow::Cow, fs};

        use crate::git_config::from_paths::escape_backslashes;
        use git_config::file::{from_paths, GitConfig};
        use tempfile::tempdir;

        #[test]
        fn multiple() {
            let dir = tempdir().unwrap();

            let a_path = dir.path().join("a");
            fs::write(
                a_path.as_path(),
                "
        [core]
          a = false
          sslVerify = true
          d = 41",
            )
            .unwrap();

            let b_path = dir.path().join("b");
            let relative_b_path: std::path::PathBuf = "b".into();
            fs::write(
                b_path.as_path(),
                "
        [diff]
          renames = true",
            )
            .unwrap();
            let ignore_path = dir.path().join("ignore");
            fs::write(
                ignore_path.as_path(),
                "
        [diff]
          renames = invalid",
            )
            .unwrap();

            let a_path_string = escape_backslashes(a_path.parent().unwrap());
            let non_canonical_path_a = format!("{}/./a", a_path_string);
            let non_existing_path = "/dfgwfsghfdsfs";
            let c_path = dir.path().join("c");
            fs::write(
                c_path.as_path(),
                format!(
                    "
        [core]
          c = 12
          d = 42
        [include]
          path = {}
          path = {}
          path = {}
        [include.ignore]
          path = {}
        [http]
          sslVerify = false",
                    non_existing_path,
                    non_canonical_path_a,
                    relative_b_path.as_path().to_str().unwrap(),
                    escape_backslashes(&ignore_path)
                ),
            )
            .unwrap();

            let config = GitConfig::from_paths(vec![c_path], &Default::default()).unwrap();

            assert_eq!(
                config.raw_value("core", None, "c").unwrap(),
                Cow::<[u8]>::Borrowed(b"12")
            );
            assert_eq!(
                config.raw_value("core", None, "d").unwrap(),
                Cow::<[u8]>::Borrowed(b"41")
            );
            assert_eq!(
                config.raw_value("http", None, "sslVerify").unwrap(),
                Cow::<[u8]>::Borrowed(b"false")
            );

            assert_eq!(
                config.raw_value("diff", None, "renames").unwrap(),
                Cow::<[u8]>::Borrowed(b"true")
            );

            assert_eq!(
                config.raw_value("core", None, "a").unwrap(),
                Cow::<[u8]>::Borrowed(b"false")
            );
        }

        #[test]
        fn respect_max_depth() {
            let dir = tempdir().unwrap();

            // 0 includes 1 - base level
            // 1 includes 2
            // 2 includes 3
            // 3 includes 4
            // 4 has no includes
            let max_depth = 4u8;
            for (i, next_i) in (0..max_depth).zip(1..=max_depth) {
                let path = dir.path().join(i.to_string());
                let next_path = dir.path().join(next_i.to_string());
                fs::write(
                    path.as_path(),
                    format!(
                        "
                [core]
                  i = {i} 
                [include]
                  path = {}",
                        escape_backslashes(&next_path),
                    ),
                )
                .unwrap();
            }

            fs::write(
                dir.path().join(max_depth.to_string()),
                "
                [core]
                  i = {}"
                    .replace("{}", &max_depth.to_string()),
            )
            .unwrap();

            let options = from_paths::Options::default();
            let config = GitConfig::from_paths(vec![dir.path().join("0")], &options).unwrap();
            assert_eq!(
                config.raw_multi_value("core", None, "i").unwrap(),
                vec![
                    Cow::Borrowed(b"0"),
                    Cow::Borrowed(b"1"),
                    Cow::Borrowed(b"2"),
                    Cow::Borrowed(b"3"),
                    Cow::Borrowed(b"4")
                ]
            );

            // with max_allowed_depth of 1 and 4 levels of includes and error_on_max_depth_exceeded: false, max_allowed_depth is exceeded and the value of level 1 is returned
            // this is equivalent to running git with --no-includes option
            let options = from_paths::Options {
                max_depth: 1,
                error_on_max_depth_exceeded: false,
                ..Default::default()
            };
            let config = GitConfig::from_paths(vec![dir.path().join("0")], &options).unwrap();
            assert_eq!(
                config.raw_value("core", None, "i").unwrap(),
                Cow::<[u8]>::Borrowed(b"1")
            );

            // with default max_allowed_depth of 10 and 4 levels of includes, last level is read
            let options = from_paths::Options::default();
            let config = GitConfig::from_paths(vec![dir.path().join("0")], &options).unwrap();
            assert_eq!(
                config.raw_value("core", None, "i").unwrap(),
                Cow::<[u8]>::Borrowed(b"4")
            );

            // with max_allowed_depth of 5, the base and 4 levels of includes, last level is read
            let options = from_paths::Options {
                max_depth: 5,
                ..Default::default()
            };
            let config = GitConfig::from_paths(vec![dir.path().join("0")], &options).unwrap();
            assert_eq!(
                config.raw_value("core", None, "i").unwrap(),
                Cow::<[u8]>::Borrowed(b"4")
            );

            // with max_allowed_depth of 2 and 4 levels of includes, max_allowed_depth is exceeded and error is returned
            let options = from_paths::Options {
                max_depth: 2,
                ..Default::default()
            };
            let config = GitConfig::from_paths(vec![dir.path().join("0")], &options);
            assert!(matches!(
                config.unwrap_err(),
                from_paths::Error::IncludeDepthExceeded { max_depth: 2 }
            ));

            // with max_allowed_depth of 2 and 4 levels of includes and error_on_max_depth_exceeded: false , max_allowed_depth is exceeded and the value of level 2 is returned
            let options = from_paths::Options {
                max_depth: 2,
                error_on_max_depth_exceeded: false,
                ..Default::default()
            };
            let config = GitConfig::from_paths(vec![dir.path().join("0")], &options).unwrap();
            assert_eq!(
                config.raw_value("core", None, "i").unwrap(),
                Cow::<[u8]>::Borrowed(b"2")
            );

            // with max_allowed_depth of 0 and 4 levels of includes, max_allowed_depth is exceeded and error is returned
            let options = from_paths::Options {
                max_depth: 0,
                ..Default::default()
            };
            let config = GitConfig::from_paths(vec![dir.path().join("0")], &options);
            assert!(matches!(
                config.unwrap_err(),
                from_paths::Error::IncludeDepthExceeded { max_depth: 0 }
            ));
        }

        #[test]
        fn simple() {
            let dir = tempdir().unwrap();

            let a_path = dir.path().join("a");
            let b_path = dir.path().join("b");

            fs::write(
                a_path.as_path(),
                format!(
                    "
        [core]
          b = true
        [include]
          path = {}
        [core]
          b = true
        [include]
          path = {}",
                    escape_backslashes(&b_path),
                    escape_backslashes(&b_path)
                ),
            )
            .unwrap();

            fs::write(
                b_path.as_path(),
                "
        [core]
          b = false",
            )
            .unwrap();

            let config = GitConfig::from_paths(vec![a_path], &Default::default()).unwrap();
            assert_eq!(
                config.raw_value("core", None, "b").unwrap(),
                Cow::<[u8]>::Borrowed(b"false")
            );
        }

        #[test]
        fn cycle_detection() {
            let dir = tempdir().unwrap();

            let a_path = dir.path().join("a");
            let b_path = dir.path().join("b");

            fs::write(
                a_path.as_path(),
                format!(
                    "
        [core]
          b = 0
        [include]
          path = {}",
                    escape_backslashes(&b_path),
                ),
            )
            .unwrap();

            fs::write(
                b_path.as_path(),
                format!(
                    "
        [core]
          b = 1
        [include]
          path = {}",
                    escape_backslashes(&a_path),
                ),
            )
            .unwrap();

            let options = from_paths::Options {
                max_depth: 4,
                ..Default::default()
            };
            let config = GitConfig::from_paths(vec![a_path.clone()], &options);
            assert!(matches!(
                config.unwrap_err(),
                from_paths::Error::IncludeDepthExceeded { max_depth: 4 }
            ));

            let options = from_paths::Options {
                max_depth: 4,
                error_on_max_depth_exceeded: false,
                ..Default::default()
            };
            let config = GitConfig::from_paths(vec![a_path], &options).unwrap();
            assert_eq!(
                config.raw_multi_value("core", None, "b").unwrap(),
                vec![
                    Cow::Borrowed(b"0"),
                    Cow::Borrowed(b"1"),
                    Cow::Borrowed(b"0"),
                    Cow::Borrowed(b"1"),
                    Cow::Borrowed(b"0"),
                ]
            );
        }

        #[test]
        fn nested() {
            let dir = tempdir().unwrap();

            let a_path = dir.path().join("a");
            fs::write(
                a_path.as_path(),
                "
        [core]
          a = false
          c = 1",
            )
            .unwrap();

            let b_path = dir.path().join("b");
            fs::write(
                b_path.as_path(),
                format!(
                    "
        [core]
          b = true
        [include]
          path = {}",
                    escape_backslashes(&a_path)
                ),
            )
            .unwrap();

            let c_path = dir.path().join("c");
            fs::write(
                c_path.as_path(),
                format!(
                    "
        [core]
          c = 12
        [include]
          path = {}",
                    escape_backslashes(&b_path)
                ),
            )
            .unwrap();

            let config = GitConfig::from_paths(vec![c_path], &Default::default()).unwrap();

            assert_eq!(
                config.raw_value("core", None, "c").unwrap(),
                Cow::<[u8]>::Borrowed(b"1")
            );

            assert_eq!(
                config.raw_value("core", None, "b").unwrap(),
                Cow::<[u8]>::Borrowed(b"true")
            );

            assert_eq!(
                config.raw_value("core", None, "a").unwrap(),
                Cow::<[u8]>::Borrowed(b"false")
            );
        }
    }

    mod conditional {
        use std::convert::TryFrom;
        use std::path::PathBuf;
        use std::{borrow::Cow, fs};

        use crate::git_config::from_paths::escape_backslashes;
        use git_config::file::{from_paths, GitConfig};
        use git_ref::FullName;
        use tempfile::tempdir;

        #[test]
        fn girdir_and_onbranch() {
            let dir = tempdir().unwrap();

            let a_path = dir.path().join("a");
            let b_path = dir.path().join("b");
            let c_path = dir.path().join("c");
            let c_slash_path = dir.path().join("c_slash");
            let d_path = dir.path().join("d");
            let e_path = dir.path().join("e");
            let i_path = dir.path().join("i");
            let g_path = dir.path().join("g");
            let w_path = dir.path().join("w");
            let x_path = dir.path().join("x");
            let branch_path = dir.path().join("branch");

            fs::write(
                a_path.as_path(),
                format!(
                    r#"
        [core]
          x = 1
          a = 1
          b = 1
          c = 1
          i = 1
        [includeIf "onbranch:/br/"]
          path = {}
        [includeIf "gitdir/i:a/B/c/D/"]
          path = {}
        [includeIf "gitdir:c\\d"]
          path = {}
        [includeIf "gitdir:./p/"]
          path = {}
        [includeIf "gitdir:z/y/"]
          path = {}
        [includeIf "gitdir:w/.git"]
          path = {}
        [includeIf "gitdir:~/.git"]
          path = {}
        [includeIf "gitdir:~/c/"]
          path = {}
        [includeIf "gitdir:a/.git"]
          path = {}
        [includeIf "gitdir:/e/x/"]
          path = {}"#,
                    escape_backslashes(&branch_path),
                    escape_backslashes(&i_path),
                    escape_backslashes(&x_path),
                    escape_backslashes(&g_path),
                    escape_backslashes(&e_path),
                    escape_backslashes(&w_path),
                    escape_backslashes(&c_path),
                    escape_backslashes(&c_slash_path),
                    escape_backslashes(&d_path),
                    escape_backslashes(&b_path)
                ),
            )
            .unwrap();

            fs::write(
                branch_path.as_path(),
                "
        [core]
          x = 7",
            )
            .unwrap();

            fs::write(
                i_path.as_path(),
                "
        [core]
          i = 3",
            )
            .unwrap();

            fs::write(
                x_path.as_path(),
                "
        [core]
          c = 5",
            )
            .unwrap();

            fs::write(
                b_path.as_path(),
                "
        [core]
          b = 2",
            )
            .unwrap();

            fs::write(
                c_path.as_path(),
                "
        [core]
          b = 3",
            )
            .unwrap();

            fs::write(
                d_path.as_path(),
                "
        [core]
          b = 4",
            )
            .unwrap();

            fs::write(
                e_path.as_path(),
                "
        [core]
          a = 5",
            )
            .unwrap();

            fs::write(
                w_path.as_path(),
                "
        [core]
          a = 6",
            )
            .unwrap();

            fs::write(
                c_slash_path.as_path(),
                "
        [core]
          b = 7",
            )
            .unwrap();

            fs::write(
                g_path.as_path(),
                "
        [core]
          b = 8",
            )
            .unwrap();

            let branch_name = FullName::try_from("refs/heads/repo/br/one").unwrap();
            let branch_name = branch_name.to_ref();
            let options = from_paths::Options {
                branch_name: Some(branch_name),
                ..Default::default()
            };

            let config = GitConfig::from_paths(vec![a_path.clone()], &options).unwrap();
            assert_eq!(
                config.raw_value("core", None, "x").unwrap(),
                Cow::<[u8]>::Borrowed(b"7"),
                "branch name match"
            );

            let a_c_d_path = PathBuf::from("/a/b/c/d/.git");
            let options = from_paths::Options {
                git_dir: Some(a_c_d_path.as_path()),
                ..Default::default()
            };

            let config = GitConfig::from_paths(vec![a_path.clone()], &options).unwrap();
            assert_eq!(
                config.raw_value("core", None, "i").unwrap(),
                Cow::<[u8]>::Borrowed(b"3"),
                "case insensitive patterns match"
            );

            let a_c_d_path = PathBuf::from("/a/c/d/.git");
            let options = from_paths::Options {
                git_dir: Some(a_c_d_path.as_path()),
                ..Default::default()
            };

            let config = GitConfig::from_paths(vec![a_path.clone()], &options).unwrap();
            assert_eq!(
                config.raw_value("core", None, "c").unwrap(),
                Cow::<[u8]>::Borrowed(b"1"),
                "patterns with backslashes do not match"
            );

            let a_p_path = a_path.parent().unwrap().join("p").join("q").join(".git");
            let options = from_paths::Options {
                git_dir: Some(a_p_path.as_path()),
                ..Default::default()
            };

            let config = GitConfig::from_paths(vec![a_path.clone()], &options).unwrap();
            assert_eq!(
                config.raw_value("core", None, "b").unwrap(),
                Cow::<[u8]>::Borrowed(b"8"),
                "relative path pattern is matched correctly"
            );

            let a_z_y_b_path = a_path.join("z").join("y").join("b").join(".git");
            let options = from_paths::Options {
                git_dir: Some(a_z_y_b_path.as_path()),
                ..Default::default()
            };

            let config = GitConfig::from_paths(vec![a_path.clone()], &options).unwrap();
            assert_eq!(
                config.raw_value("core", None, "a").unwrap(),
                Cow::<[u8]>::Borrowed(b"5"),
                "the pattern is prefixed and suffixed with ** to match GIT_DIR containing it in the middle"
            );

            let cw_path = PathBuf::from("C:\\w\\.git".to_string());
            let options = from_paths::Options {
                git_dir: Some(cw_path.as_path()),
                ..Default::default()
            };
            let config = GitConfig::from_paths(vec![a_path.clone()], &options).unwrap();
            assert_eq!(
                config.raw_value("core", None, "a").unwrap(),
                Cow::<[u8]>::Borrowed(b"6"),
                "backslashes in GIT_DIR are converted to forward slashes"
            );

            let home_git_path = dirs::home_dir().unwrap().join(".git");
            let options = from_paths::Options {
                git_dir: Some(home_git_path.as_path()),
                ..Default::default()
            };
            let config = GitConfig::from_paths(vec![a_path.clone()], &options).unwrap();
            assert_eq!(
                config.raw_multi_value("core", None, "b").unwrap(),
                vec![Cow::<[u8]>::Borrowed(b"1"), Cow::<[u8]>::Borrowed(b"3")],
                "tilde ~ path is resolved to home directory"
            );

            let home_git_path = dirs::home_dir().unwrap().join("c").join("d").join(".git");
            let options = from_paths::Options {
                git_dir: Some(home_git_path.as_path()),
                ..Default::default()
            };
            let config = GitConfig::from_paths(vec![a_path.clone()], &options).unwrap();
            assert_eq!(
                config.raw_value("core", None, "b").unwrap(),
                Cow::<[u8]>::Borrowed(b"7"),
                "path with trailing slash is matched"
            );

            let x_a_path = dir.path().join("x").join("a").join(".git");
            let options = from_paths::Options {
                git_dir: Some(x_a_path.as_path()),
                ..Default::default()
            };
            let config = GitConfig::from_paths(vec![a_path.clone()], &options).unwrap();
            assert_eq!(
                config.raw_value("core", None, "b").unwrap(),
                Cow::<[u8]>::Borrowed(b"4"),
                "** is prepended so paths ending with the pattern are matched"
            );

            let e_x_y_path = PathBuf::from("/e/x/y/.git");
            let options = from_paths::Options {
                git_dir: Some(e_x_y_path.as_path()),
                ..Default::default()
            };
            let config = GitConfig::from_paths(vec![a_path], &options).unwrap();
            assert_eq!(
                config.raw_value("core", None, "b").unwrap(),
                Cow::<[u8]>::Borrowed(b"2"),
                "absolute path pattern is matched with sub path from GIT_DIR"
            );
        }
    }
}
