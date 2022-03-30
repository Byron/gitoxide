#[cfg(test)]
mod from_paths {
    use std::borrow::Cow;
    use std::path::Path;
    use std::{fs, io};

    use git_config::file::GitConfig;
    use git_config::parser::ParserOrIoError;
    use tempfile::tempdir;

    #[test]
    fn parse_config_with_windows_line_endings_successfully() -> crate::Result {
        GitConfig::open(Path::new("tests").join("fixtures").join("repo-config.crlf"))?;
        Ok(())
    }

    #[test]
    fn file_not_found() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("config");

        let paths = vec![config_path.as_path()];
        let error = GitConfig::from_paths(&paths).unwrap_err();
        assert!(matches!(error, ParserOrIoError::Io(io_error) if io_error.kind() == io::ErrorKind::NotFound));
    }

    #[test]
    fn single_path() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("config");
        fs::write(config_path.as_path(), b"[core]\nboolean = true").expect("Unable to write config file");

        let paths = vec![config_path.as_path()];
        let config = GitConfig::from_paths(&paths).unwrap();

        assert_eq!(
            config.get_raw_value("core", None, "boolean"),
            Ok(Cow::<[u8]>::Borrowed(b"true"))
        );

        assert_eq!(config.len(), 1);
    }

    #[test]
    fn multiple_paths_single_value() {
        let dir = tempdir().unwrap();

        let a_path = dir.path().join("a");
        fs::write(a_path.as_path(), b"[core]\na = true").expect("Unable to write config file");

        let b_path = dir.path().join("b");
        fs::write(b_path.as_path(), b"[core]\nb = true").expect("Unable to write config file");

        let c_path = dir.path().join("c");
        fs::write(c_path.as_path(), b"[core]\nc = true").expect("Unable to write config file");

        let paths = vec![a_path.as_path(), b_path.as_path(), c_path.as_path()];
        let config = GitConfig::from_paths(&paths).unwrap();

        assert_eq!(
            config.get_raw_value("core", None, "a"),
            Ok(Cow::<[u8]>::Borrowed(b"true"))
        );

        assert_eq!(
            config.get_raw_value("core", None, "b"),
            Ok(Cow::<[u8]>::Borrowed(b"true"))
        );

        assert_eq!(
            config.get_raw_value("core", None, "c"),
            Ok(Cow::<[u8]>::Borrowed(b"true"))
        );

        assert_eq!(config.len(), 3);
    }

    #[test]
    fn multiple_paths_multi_value() {
        let dir = tempdir().unwrap();

        let a_path = dir.path().join("a");
        fs::write(a_path.as_path(), b"[core]\nkey = a").expect("Unable to write config file");

        let b_path = dir.path().join("b");
        fs::write(b_path.as_path(), b"[core]\nkey = b").expect("Unable to write config file");

        let c_path = dir.path().join("c");
        fs::write(c_path.as_path(), b"[core]\nkey = c").expect("Unable to write config file");

        let paths = vec![a_path.as_path(), b_path.as_path(), c_path.as_path()];
        let config = GitConfig::from_paths(&paths).unwrap();

        assert_eq!(
            config.get_raw_multi_value("core", None, "key").unwrap(),
            vec![Cow::Borrowed(b"a"), Cow::Borrowed(b"b"), Cow::Borrowed(b"c")]
        );

        assert_eq!(config.len(), 3);
    }
}

#[cfg(test)]
pub mod from_env_paths {}

#[cfg(test)]
mod from_env {
    use std::borrow::Cow;
    use std::env;

    use git_config::file::error::GitConfigFromEnvError;
    use git_config::file::GitConfig;
    use serial_test::serial;

    struct Env {
        altered_vars: Vec<&'static str>,
    }

    impl Env {
        fn new() -> Self {
            Env {
                altered_vars: Vec::new(),
            }
        }

        fn set(mut self, var: &'static str, value: &'static str) -> Self {
            env::set_var(var, value);
            self.altered_vars.push(var);
            self
        }
    }

    impl Drop for Env {
        fn drop(&mut self) {
            for var in &self.altered_vars {
                env::remove_var(var);
            }
        }
    }

    #[test]
    #[serial]
    fn empty_without_relevant_environment() {
        let config = GitConfig::from_env().unwrap();
        assert!(config.is_none());
    }

    #[test]
    #[serial]
    fn empty_with_zero_count() {
        let _env = Env::new().set("GIT_CONFIG_COUNT", "0");
        let config = GitConfig::from_env().unwrap();
        assert!(config.is_none());
    }

    #[test]
    #[serial]
    fn parse_error_with_invalid_count() {
        let _env = Env::new().set("GIT_CONFIG_COUNT", "invalid");
        let err = GitConfig::from_env().unwrap_err();
        assert!(matches!(err, GitConfigFromEnvError::ParseError(_)));
    }

    #[test]
    #[serial]
    fn single_key_value_pair() {
        let _env = Env::new()
            .set("GIT_CONFIG_COUNT", "1")
            .set("GIT_CONFIG_KEY_0", "core.key")
            .set("GIT_CONFIG_VALUE_0", "value");

        let config = GitConfig::from_env().unwrap().unwrap();
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

        let config = GitConfig::from_env().unwrap().unwrap();

        assert_eq!(config.get_raw_value("core", None, "a"), Ok(Cow::<[u8]>::Borrowed(b"a")));
        assert_eq!(config.get_raw_value("core", None, "b"), Ok(Cow::<[u8]>::Borrowed(b"b")));
        assert_eq!(config.get_raw_value("core", None, "c"), Ok(Cow::<[u8]>::Borrowed(b"c")));
        assert_eq!(config.len(), 3);
    }
}

#[cfg(test)]
mod mutable_value {
    use git_config::file::GitConfig;
    use std::convert::TryFrom;

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
    use git_config::file::GitConfig;
    use std::{borrow::Cow, convert::TryFrom};

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
mod get_raw_value {
    use git_config::file::{GitConfig, GitConfigError};
    use git_config::parser::SectionHeaderName;
    use std::borrow::Cow;
    use std::convert::TryFrom;

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
    use git_config::file::GitConfig;
    use git_config::values::{Boolean, TrueVariant, Value};
    use std::borrow::Cow;
    use std::convert::TryFrom;
    use std::error::Error;

    #[test]
    fn single_section() -> Result<(), Box<dyn Error>> {
        let config = GitConfig::try_from("[core]\na=b\nc").unwrap();
        let first_value: Value = config.value("core", None, "a")?;
        let second_value: Boolean = config.value("core", None, "c")?;

        assert_eq!(first_value, Value::Other(Cow::Borrowed(b"b")));
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
        let value = config.value::<Value>("remote", Some("origin"), "url").unwrap();
        assert_eq!(value, Value::Other(Cow::Borrowed(b"git@github.com:Byron/gitoxide.git")));
    }
}

#[cfg(test)]
mod get_raw_multi_value {
    use git_config::file::{GitConfig, GitConfigError};
    use git_config::parser::SectionHeaderName;
    use std::borrow::Cow;
    use std::convert::TryFrom;

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
    use git_config::file::GitConfig;
    use std::convert::TryFrom;

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
