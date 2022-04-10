#[cfg(test)]
mod mutable_value {
    use std::convert::TryFrom;

    use git_config::file::GitConfig;

    fn init_config() -> GitConfig<'static> {
        GitConfig::try_from(
            r#"[core]
                a=b"100"
            [core]
                c=d
                e=f"#,
        )
        .unwrap()
    }

    #[test]
    fn value_is_correct() {
        let mut git_config = init_config();

        let value = git_config.get_raw_value_mut("core", None, "a").unwrap();
        assert_eq!(&*value.get().unwrap(), b"b100");
    }

    #[test]
    fn set_string_cleanly_updates() {
        let mut git_config = init_config();

        let mut value = git_config.get_raw_value_mut("core", None, "a").unwrap();
        value.set_string("hello world".to_string());
        assert_eq!(
            git_config.to_string(),
            r#"[core]
                a=hello world
            [core]
                c=d
                e=f"#,
        );

        let mut value = git_config.get_raw_value_mut("core", None, "e").unwrap();
        value.set_string(String::new());
        assert_eq!(
            git_config.to_string(),
            r#"[core]
                a=hello world
            [core]
                c=d
                e="#,
        );
    }

    #[test]
    fn delete_value() {
        let mut git_config = init_config();

        let mut value = git_config.get_raw_value_mut("core", None, "a").unwrap();
        value.delete();
        assert_eq!(
            git_config.to_string(),
            "[core]\n                \n            [core]
                c=d
                e=f",
        );

        let mut value = git_config.get_raw_value_mut("core", None, "c").unwrap();
        value.delete();
        assert_eq!(
            git_config.to_string(),
            "[core]\n                \n            [core]\n                \n                e=f",
        );
    }

    #[test]
    fn get_value_after_deleted() {
        let mut git_config = init_config();

        let mut value = git_config.get_raw_value_mut("core", None, "a").unwrap();
        value.delete();
        assert!(value.get().is_err());
    }

    #[test]
    fn set_string_after_deleted() {
        let mut git_config = init_config();

        let mut value = git_config.get_raw_value_mut("core", None, "a").unwrap();
        value.delete();
        value.set_string("hello world".to_string());
        assert_eq!(
            git_config.to_string(),
            r#"[core]
                a=hello world
            [core]
                c=d
                e=f"#,
        );
    }

    #[test]
    fn subsequent_delete_calls_are_noop() {
        let mut git_config = init_config();

        let mut value = git_config.get_raw_value_mut("core", None, "a").unwrap();
        for _ in 0..10 {
            value.delete();
        }
        assert_eq!(
            git_config.to_string(),
            "[core]\n                \n            [core]
                c=d
                e=f",
        );
    }

    #[test]
    fn partial_values_are_supported() {
        let mut git_config = GitConfig::try_from(
            r#"[core]
                a=b"100"\
b
            [core]
                c=d
                e=f"#,
        )
        .unwrap();
        let mut value = git_config.get_raw_value_mut("core", None, "a").unwrap();
        assert_eq!(&*value.get().unwrap(), b"b100b");
        value.delete();
        assert_eq!(
            git_config.to_string(),
            "[core]\n                \n            [core]
                c=d
                e=f",
        );
    }
}

#[cfg(test)]
mod mutable_multi_value {
    use std::{borrow::Cow, convert::TryFrom};

    use git_config::file::GitConfig;

    fn init_config() -> GitConfig<'static> {
        GitConfig::try_from(
            r#"[core]
                a=b"100"
            [core]
                a=d
                a=f"#,
        )
        .unwrap()
    }

    #[test]
    fn value_is_correct() {
        let mut git_config = init_config();

        let value = git_config.get_raw_multi_value_mut("core", None, "a").unwrap();
        assert_eq!(
            &*value.get().unwrap(),
            vec![
                Cow::<[u8]>::Owned(b"b100".to_vec()),
                Cow::<[u8]>::Borrowed(b"d"),
                Cow::<[u8]>::Borrowed(b"f"),
            ]
        );
    }

    #[test]
    fn non_empty_sizes_are_correct() {
        let mut git_config = init_config();
        assert_eq!(git_config.get_raw_multi_value_mut("core", None, "a").unwrap().len(), 3);
        assert!(!git_config
            .get_raw_multi_value_mut("core", None, "a")
            .unwrap()
            .is_empty());
    }

    #[test]
    fn set_value_at_start() {
        let mut git_config = init_config();
        let mut values = git_config.get_raw_multi_value_mut("core", None, "a").unwrap();
        values.set_string(0, "Hello".to_string());
        assert_eq!(
            git_config.to_string(),
            r#"[core]
                a=Hello
            [core]
                a=d
                a=f"#,
        );
    }

    #[test]
    fn set_value_at_end() {
        let mut git_config = init_config();
        let mut values = git_config.get_raw_multi_value_mut("core", None, "a").unwrap();
        values.set_string(2, "Hello".to_string());
        assert_eq!(
            git_config.to_string(),
            r#"[core]
                a=b"100"
            [core]
                a=d
                a=Hello"#,
        );
    }

    #[test]
    fn set_values_all() {
        let mut git_config = init_config();
        let mut values = git_config.get_raw_multi_value_mut("core", None, "a").unwrap();
        values.set_owned_values_all(b"Hello");
        assert_eq!(
            git_config.to_string(),
            r#"[core]
                a=Hello
            [core]
                a=Hello
                a=Hello"#,
        );
    }

    #[test]
    fn delete() {
        let mut git_config = init_config();
        let mut values = git_config.get_raw_multi_value_mut("core", None, "a").unwrap();
        values.delete(0);
        assert_eq!(
            git_config.to_string(),
            "[core]\n                \n            [core]
                a=d
                a=f",
        );
    }

    #[test]
    fn delete_all() {
        let mut git_config = init_config();
        let mut values = git_config.get_raw_multi_value_mut("core", None, "a").unwrap();
        values.delete_all();
        assert!(values.get().is_err());
        assert_eq!(
            git_config.to_string(),
            "[core]\n                \n            [core]\n                \n                ",
        );
    }

    #[test]
    fn partial_values_are_supported() {
        let mut git_config = GitConfig::try_from(
            r#"[core]
                a=b\
"100"
            [core]
                a=d\
b
                a=f\
a"#,
        )
        .unwrap();
        let mut values = git_config.get_raw_multi_value_mut("core", None, "a").unwrap();

        assert_eq!(
            &*values.get().unwrap(),
            vec![
                Cow::<[u8]>::Owned(b"b100".to_vec()),
                Cow::<[u8]>::Borrowed(b"db"),
                Cow::<[u8]>::Borrowed(b"fa"),
            ]
        );

        values.delete_all();
        assert!(values.get().is_err());
    }
}

#[cfg(test)]
mod from_paths_tests {
    use std::{borrow::Cow, fs, io, path::Path};

    use git_config::{
        file::{from_paths, from_paths::Error, GitConfig},
        parser::ParserOrIoError,
    };
    use tempfile::tempdir;

    /// Escapes backslash when writing a path as string so that it is a valid windows path
    fn escape_backslashes(path: &std::path::Path) -> String {
        path.to_str().unwrap().replace('\\', "\\\\")
    }

    #[test]
    fn parse_config_with_windows_line_endings_successfully() -> crate::Result {
        GitConfig::open(Path::new("tests").join("fixtures").join("repo-config.crlf"))?;
        Ok(())
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
            config.get_raw_value("core", None, "boolean"),
            Ok(Cow::<[u8]>::Borrowed(b"true"))
        );

        assert_eq!(config.len(), 1);
    }

    #[test]
    fn multiple_includes() {
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
            config.get_raw_value("core", None, "c"),
            Ok(Cow::<[u8]>::Borrowed(b"12"))
        );
        assert_eq!(
            config.get_raw_value("core", None, "d"),
            Ok(Cow::<[u8]>::Borrowed(b"41"))
        );
        assert_eq!(
            config.get_raw_value("http", None, "sslVerify"),
            Ok(Cow::<[u8]>::Borrowed(b"false"))
        );

        assert_eq!(
            config.get_raw_value("diff", None, "renames"),
            Ok(Cow::<[u8]>::Borrowed(b"true"))
        );

        assert_eq!(
            config.get_raw_value("core", None, "a"),
            Ok(Cow::<[u8]>::Borrowed(b"false"))
        );
    }

    #[test]
    fn nested_include_respects_max_depth() {
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
            config.get_raw_multi_value("core", None, "i").unwrap(),
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
        assert_eq!(config.get_raw_value("core", None, "i"), Ok(Cow::<[u8]>::Borrowed(b"1")));

        // with default max_allowed_depth of 10 and 4 levels of includes, last level is read
        let options = from_paths::Options::default();
        let config = GitConfig::from_paths(vec![dir.path().join("0")], &options).unwrap();
        assert_eq!(config.get_raw_value("core", None, "i"), Ok(Cow::<[u8]>::Borrowed(b"4")));

        // with max_allowed_depth of 5, the base and 4 levels of includes, last level is read
        let options = from_paths::Options {
            max_depth: 5,
            ..Default::default()
        };
        let config = GitConfig::from_paths(vec![dir.path().join("0")], &options).unwrap();
        assert_eq!(config.get_raw_value("core", None, "i"), Ok(Cow::<[u8]>::Borrowed(b"4")));

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
        assert_eq!(config.get_raw_value("core", None, "i"), Ok(Cow::<[u8]>::Borrowed(b"2")));

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
    fn config_files_are_included_unconditionally() {
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
            config.get_raw_value("core", None, "b"),
            Ok(Cow::<[u8]>::Borrowed(b"false"))
        );
    }

    #[test]
    fn include_path_cycles_are_detected() {
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
            config.get_raw_multi_value("core", None, "b").unwrap(),
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
    fn nested_include() {
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

        assert_eq!(config.get_raw_value("core", None, "c"), Ok(Cow::<[u8]>::Borrowed(b"1")));

        assert_eq!(
            config.get_raw_value("core", None, "b"),
            Ok(Cow::<[u8]>::Borrowed(b"true"))
        );

        assert_eq!(
            config.get_raw_value("core", None, "a"),
            Ok(Cow::<[u8]>::Borrowed(b"false"))
        );
    }

    #[test]
    fn include_if_with_gitdir() {
        let dir = tempdir().unwrap();

        let a_path = dir.path().join("a");
        let b_path = dir.path().join("b");
        let c_path = dir.path().join("c");
        let c_slash_path = dir.path().join("c_slash");
        let d_path = dir.path().join("d");
        let inside_a_x_path = dir.path().join("a").join("x").join(".git");

        fs::write(
            a_path.as_path(),
            format!(
                r#"
            [core]
              b = 1
            [includeIf "gitdir:~/.git"]
              path = {}
            [includeIf "gitdir:~/.git/"]
              path = {}
            [includeIf "gitdir:a/.git"]
              path = {}
            [includeIf "gitdir:{}"]
              path = {}"#,
                escape_backslashes(&c_path),
                escape_backslashes(&c_slash_path),
                escape_backslashes(&d_path),
                escape_backslashes(&inside_a_x_path),
                escape_backslashes(&b_path)
            ),
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
            c_slash_path.as_path(),
            "
            [core]
              b = 7",
        )
        .unwrap();

        fs::write(
            d_path.as_path(),
            "
            [core]
              b = 4",
        )
        .unwrap();

        let options = from_paths::Options {
            git_dir: Some(inside_a_x_path.as_path()),
            ..Default::default()
        };
        let config = GitConfig::from_paths(vec![a_path.clone()], &options).unwrap();
        assert_eq!(
            config.get_raw_value("core", None, "b"),
            Ok(Cow::<[u8]>::Borrowed(b"2")),
            "absolute paths from options and config are the same"
        );

        let home_git_path = dirs::home_dir().unwrap().join(".git");
        let options = from_paths::Options {
            git_dir: Some(home_git_path.as_path()),
            ..Default::default()
        };
        let config = GitConfig::from_paths(vec![a_path.clone()], &options).unwrap();
        // sometimes returns
        // tilde ~ path is resolved to home directory
        // Left:  Ok([[49], [55], [51]])
        // Right: Ok([[49], [51], [55]])

        // run multiple times
        assert_eq!(
            config.get_raw_multi_value("core", None, "b"),
            Ok(vec![
                Cow::<[u8]>::Borrowed(b"1"),
                Cow::<[u8]>::Borrowed(b"3"),
                Cow::<[u8]>::Borrowed(b"7")
            ]),
            "tilde ~ path is resolved to home directory"
        );

        let a_path_clone = a_path.clone().join(".git");
        let options = from_paths::Options {
            git_dir: Some(a_path_clone.as_path()),
            ..Default::default()
        };
        let config = GitConfig::from_paths(vec![a_path], &options).unwrap();
        assert_eq!(
            config.get_raw_value("core", None, "b"),
            Ok(Cow::<[u8]>::Borrowed(b"4")),
            "relative config path is resolved correctly"
        );
    }

    #[test]
    fn multiple_paths_single_value() {
        let dir = tempdir().unwrap();

        let a_path = dir.path().join("a");
        fs::write(a_path.as_path(), b"[core]\na = true").unwrap();

        let b_path = dir.path().join("b");
        fs::write(b_path.as_path(), b"[core]\nb = true").unwrap();

        let c_path = dir.path().join("c");
        fs::write(c_path.as_path(), b"[core]\nc = true").unwrap();

        let d_path = dir.path().join("d");
        fs::write(d_path.as_path(), b"[core]\na = false").unwrap();

        let paths = vec![a_path, b_path, c_path, d_path];
        let config = GitConfig::from_paths(paths, &Default::default()).unwrap();

        assert_eq!(
            config.get_raw_value("core", None, "a"),
            Ok(Cow::<[u8]>::Borrowed(b"false"))
        );

        assert_eq!(
            config.get_raw_value("core", None, "b"),
            Ok(Cow::<[u8]>::Borrowed(b"true"))
        );

        assert_eq!(
            config.get_raw_value("core", None, "c"),
            Ok(Cow::<[u8]>::Borrowed(b"true"))
        );

        assert_eq!(config.len(), 4);
    }

    #[test]
    fn multiple_paths_multi_value() {
        let dir = tempdir().unwrap();

        let a_path = dir.path().join("a");
        fs::write(a_path.as_path(), b"[core]\nkey = a").unwrap();

        let b_path = dir.path().join("b");
        fs::write(b_path.as_path(), b"[core]\nkey = b").unwrap();

        let c_path = dir.path().join("c");
        fs::write(c_path.as_path(), b"[core]\nkey = c").unwrap();

        let d_path = dir.path().join("d");
        fs::write(d_path.as_path(), b"[include]\npath = d_path").unwrap();

        let e_path = dir.path().join("e");
        fs::write(e_path.as_path(), b"[include]\npath = e_path").unwrap();

        let paths = vec![a_path, b_path, c_path, d_path, e_path];
        let config = GitConfig::from_paths(paths, &Default::default()).unwrap();

        assert_eq!(
            config.get_raw_multi_value("core", None, "key").unwrap(),
            vec![Cow::Borrowed(b"a"), Cow::Borrowed(b"b"), Cow::Borrowed(b"c")]
        );

        assert_eq!(
            config.get_raw_multi_value("include", None, "path").unwrap(),
            vec![Cow::Borrowed(b"d_path"), Cow::Borrowed(b"e_path")]
        );

        assert_eq!(config.len(), 5);
    }
}

#[cfg(test)]
mod from_env_tests {
    use std::{borrow::Cow, env, fs};

    use git_config::file::{from_env, from_paths, from_paths::Options, GitConfig};
    use serial_test::serial;
    use tempfile::tempdir;

    struct Env<'a> {
        altered_vars: Vec<&'a str>,
    }

    impl<'a> Env<'a> {
        fn new() -> Self {
            Env {
                altered_vars: Vec::new(),
            }
        }

        fn set(mut self, var: &'a str, value: &'a str) -> Self {
            env::set_var(var, value);
            self.altered_vars.push(var);
            self
        }
    }

    impl<'a> Drop for Env<'a> {
        fn drop(&mut self) {
            for var in &self.altered_vars {
                env::remove_var(var);
            }
        }
    }

    #[test]
    #[serial]
    fn empty_without_relevant_environment() {
        let config = GitConfig::from_env(&Options::default()).unwrap();
        assert!(config.is_none());
    }

    #[test]
    #[serial]
    fn empty_with_zero_count() {
        let _env = Env::new().set("GIT_CONFIG_COUNT", "0");
        let config = GitConfig::from_env(&Options::default()).unwrap();
        assert!(config.is_none());
    }

    #[test]
    #[serial]
    fn parse_error_with_invalid_count() {
        let _env = Env::new().set("GIT_CONFIG_COUNT", "invalid");
        let err = GitConfig::from_env(&Options::default()).unwrap_err();
        assert!(matches!(err, from_env::Error::ParseError(_)));
    }

    #[test]
    #[serial]
    fn single_key_value_pair() {
        let _env = Env::new()
            .set("GIT_CONFIG_COUNT", "1")
            .set("GIT_CONFIG_KEY_0", "core.key")
            .set("GIT_CONFIG_VALUE_0", "value");

        let config = GitConfig::from_env(&Options::default()).unwrap().unwrap();
        assert_eq!(
            config.get_raw_value("core", None, "key"),
            Ok(Cow::<[u8]>::Borrowed(b"value"))
        );

        assert_eq!(config.len(), 1);
    }

    #[test]
    #[serial]
    fn multiple_key_value_pairs() {
        let _env = Env::new()
            .set("GIT_CONFIG_COUNT", "3")
            .set("GIT_CONFIG_KEY_0", "core.a")
            .set("GIT_CONFIG_VALUE_0", "a")
            .set("GIT_CONFIG_KEY_1", "core.b")
            .set("GIT_CONFIG_VALUE_1", "b")
            .set("GIT_CONFIG_KEY_2", "core.c")
            .set("GIT_CONFIG_VALUE_2", "c");

        let config = GitConfig::from_env(&Options::default()).unwrap().unwrap();

        assert_eq!(config.get_raw_value("core", None, "a"), Ok(Cow::<[u8]>::Borrowed(b"a")));
        assert_eq!(config.get_raw_value("core", None, "b"), Ok(Cow::<[u8]>::Borrowed(b"b")));
        assert_eq!(config.get_raw_value("core", None, "c"), Ok(Cow::<[u8]>::Borrowed(b"c")));
        assert_eq!(config.len(), 3);
    }

    #[test]
    #[serial]
    fn error_on_relative_paths_in_include_paths() {
        let _env = Env::new()
            .set("GIT_CONFIG_COUNT", "1")
            .set("GIT_CONFIG_KEY_0", "include.path")
            .set("GIT_CONFIG_VALUE_0", "some_git_config");

        let config = GitConfig::from_env(&Options::default());

        assert!(matches!(
            config,
            Err(from_env::Error::FromPathsError(from_paths::Error::MissingConfigPath))
        ));
    }

    #[test]
    #[serial]
    fn follow_include_paths() {
        let dir = tempdir().unwrap();
        let a_path = dir.path().join("a");
        fs::write(&a_path, "[core]\nkey = changed").unwrap();
        let b_path = dir.path().join("b");
        fs::write(&b_path, "[core]\nkey = invalid").unwrap();

        let _env = Env::new()
            .set("GIT_CONFIG_COUNT", "4")
            .set("GIT_CONFIG_KEY_0", "core.key")
            .set("GIT_CONFIG_VALUE_0", "value")
            .set("GIT_CONFIG_KEY_1", "include.path")
            .set("GIT_CONFIG_VALUE_1", a_path.to_str().unwrap())
            .set("GIT_CONFIG_KEY_2", "other.path")
            .set("GIT_CONFIG_VALUE_2", b_path.to_str().unwrap())
            .set("GIT_CONFIG_KEY_3", "include.origin.path")
            .set("GIT_CONFIG_VALUE_3", b_path.to_str().unwrap());

        let config = GitConfig::from_env(&Options::default()).unwrap().unwrap();

        assert_eq!(
            config.get_raw_value("core", None, "key"),
            Ok(Cow::<[u8]>::Borrowed(b"changed"))
        );
        assert_eq!(config.len(), 5);
    }
}

#[cfg(test)]
mod get_raw_value {
    use std::{borrow::Cow, convert::TryFrom};

    use git_config::{
        file::{GitConfig, GitConfigError},
        parser::SectionHeaderName,
    };

    #[test]
    fn single_section() {
        let config = GitConfig::try_from("[core]\na=b\nc=d").unwrap();
        assert_eq!(config.get_raw_value("core", None, "a"), Ok(Cow::<[u8]>::Borrowed(b"b")));
        assert_eq!(config.get_raw_value("core", None, "c"), Ok(Cow::<[u8]>::Borrowed(b"d")));
    }

    #[test]
    fn last_one_wins_respected_in_section() {
        let config = GitConfig::try_from("[core]\na=b\na=d").unwrap();
        assert_eq!(config.get_raw_value("core", None, "a"), Ok(Cow::<[u8]>::Borrowed(b"d")));
    }

    #[test]
    fn last_one_wins_respected_across_section() {
        let config = GitConfig::try_from("[core]\na=b\n[core]\na=d").unwrap();
        assert_eq!(config.get_raw_value("core", None, "a"), Ok(Cow::<[u8]>::Borrowed(b"d")));
    }

    #[test]
    fn section_not_found() {
        let config = GitConfig::try_from("[core]\na=b\nc=d").unwrap();
        assert_eq!(
            config.get_raw_value("foo", None, "a"),
            Err(GitConfigError::SectionDoesNotExist(SectionHeaderName("foo".into())))
        );
    }

    #[test]
    fn subsection_not_found() {
        let config = GitConfig::try_from("[core]\na=b\nc=d").unwrap();
        assert_eq!(
            config.get_raw_value("core", Some("a"), "a"),
            Err(GitConfigError::SubSectionDoesNotExist(Some("a")))
        );
    }

    #[test]
    fn key_not_found() {
        let config = GitConfig::try_from("[core]\na=b\nc=d").unwrap();
        assert_eq!(
            config.get_raw_value("core", None, "aaaaaa"),
            Err(GitConfigError::KeyDoesNotExist)
        );
    }

    #[test]
    fn subsection_must_be_respected() {
        let config = GitConfig::try_from("[core]a=b\n[core.a]a=c").unwrap();
        assert_eq!(config.get_raw_value("core", None, "a"), Ok(Cow::<[u8]>::Borrowed(b"b")));
        assert_eq!(
            config.get_raw_value("core", Some("a"), "a"),
            Ok(Cow::<[u8]>::Borrowed(b"c"))
        );
    }
}

#[cfg(test)]
mod get_value {
    use std::{borrow::Cow, convert::TryFrom, error::Error};

    use git_config::{
        file::GitConfig,
        values::{Boolean, Bytes, TrueVariant},
    };

    #[test]
    fn single_section() -> Result<(), Box<dyn Error>> {
        let config = GitConfig::try_from("[core]\na=b\nc").unwrap();
        let first_value: Bytes = config.value("core", None, "a")?;
        let second_value: Boolean = config.value("core", None, "c")?;

        assert_eq!(
            first_value,
            Bytes {
                value: Cow::Borrowed(b"b")
            }
        );
        assert_eq!(second_value, Boolean::True(TrueVariant::Implicit));

        Ok(())
    }

    #[test]
    fn sections_by_name() {
        let config = r#"
        [core]
            repositoryformatversion = 0
            filemode = true
            bare = false
            logallrefupdates = true
        [remote "origin"]
            url = git@github.com:Byron/gitoxide.git
            fetch = +refs/heads/*:refs/remotes/origin/*
        "#;

        let config = GitConfig::try_from(config).unwrap();
        let value = config.value::<Bytes>("remote", Some("origin"), "url").unwrap();
        assert_eq!(
            value,
            Bytes {
                value: Cow::Borrowed(b"git@github.com:Byron/gitoxide.git")
            }
        );
    }
}

#[cfg(test)]
mod get_raw_multi_value {
    use std::{borrow::Cow, convert::TryFrom};

    use git_config::{
        file::{GitConfig, GitConfigError},
        parser::SectionHeaderName,
    };

    #[test]
    fn single_value_is_identical_to_single_value_query() {
        let config = GitConfig::try_from("[core]\na=b\nc=d").unwrap();
        assert_eq!(
            vec![config.get_raw_value("core", None, "a").unwrap()],
            config.get_raw_multi_value("core", None, "a").unwrap()
        );
    }

    #[test]
    fn multi_value_in_section() {
        let config = GitConfig::try_from("[core]\na=b\na=c").unwrap();
        assert_eq!(
            config.get_raw_multi_value("core", None, "a").unwrap(),
            vec![Cow::Borrowed(b"b"), Cow::Borrowed(b"c")]
        );
    }

    #[test]
    fn multi_value_across_sections() {
        let config = GitConfig::try_from("[core]\na=b\na=c\n[core]a=d").unwrap();
        assert_eq!(
            config.get_raw_multi_value("core", None, "a").unwrap(),
            vec![Cow::Borrowed(b"b"), Cow::Borrowed(b"c"), Cow::Borrowed(b"d")]
        );
    }

    #[test]
    fn section_not_found() {
        let config = GitConfig::try_from("[core]\na=b\nc=d").unwrap();
        assert_eq!(
            config.get_raw_multi_value("foo", None, "a"),
            Err(GitConfigError::SectionDoesNotExist(SectionHeaderName("foo".into())))
        );
    }

    #[test]
    fn subsection_not_found() {
        let config = GitConfig::try_from("[core]\na=b\nc=d").unwrap();
        assert_eq!(
            config.get_raw_multi_value("core", Some("a"), "a"),
            Err(GitConfigError::SubSectionDoesNotExist(Some("a")))
        );
    }

    #[test]
    fn key_not_found() {
        let config = GitConfig::try_from("[core]\na=b\nc=d").unwrap();
        assert_eq!(
            config.get_raw_multi_value("core", None, "aaaaaa"),
            Err(GitConfigError::KeyDoesNotExist)
        );
    }

    #[test]
    fn subsection_must_be_respected() {
        let config = GitConfig::try_from("[core]a=b\n[core.a]a=c").unwrap();
        assert_eq!(
            config.get_raw_multi_value("core", None, "a").unwrap(),
            vec![Cow::Borrowed(b"b")]
        );
        assert_eq!(
            config.get_raw_multi_value("core", Some("a"), "a").unwrap(),
            vec![Cow::Borrowed(b"c")]
        );
    }

    #[test]
    fn non_relevant_subsection_is_ignored() {
        let config = GitConfig::try_from("[core]\na=b\na=c\n[core]a=d\n[core]g=g").unwrap();
        assert_eq!(
            config.get_raw_multi_value("core", None, "a").unwrap(),
            vec![Cow::Borrowed(b"b"), Cow::Borrowed(b"c"), Cow::Borrowed(b"d")]
        );
    }
}

#[cfg(test)]
mod display {
    use std::convert::TryFrom;

    use git_config::file::GitConfig;

    #[test]
    fn can_reconstruct_empty_config() {
        let config = r#"

        "#;
        assert_eq!(GitConfig::try_from(config).unwrap().to_string(), config);
    }

    #[test]
    fn can_reconstruct_non_empty_config() {
        let config = r#"
            [user]
                email = code@eddie.sh
            [core]
                autocrlf = input
            [push]
                default = simple
            [commit]
                gpgsign = true
            [gpg]
                program = gpg
            [url "ssh://git@github.com/"]
                insteadOf = "github://"
            [url "ssh://git@git.eddie.sh/edward/"]
                insteadOf = "gitea://"
            [pull]
                ff = only
            [init]
                defaultBranch = master
        "#;

        assert_eq!(GitConfig::try_from(config).unwrap().to_string(), config);
    }

    #[test]
    fn can_reconstruct_configs_with_implicits() {
        let config = r#"
            [user]
                email
                name
            [core]
                autocrlf
            [push]
                default
            [commit]
                gpgsign
        "#;

        assert_eq!(GitConfig::try_from(config).unwrap().to_string(), config);
    }

    #[test]
    fn can_reconstruct_configs_without_whitespace_in_middle() {
        let config = r#"
            [core]
                autocrlf=input
            [push]
                default=simple
            [commit]
                gpgsign=true
            [pull]
                ff = only
            [init]
                defaultBranch = master
        "#;

        assert_eq!(GitConfig::try_from(config).unwrap().to_string(), config);
    }
}
