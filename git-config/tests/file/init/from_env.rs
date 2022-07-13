use std::{borrow::Cow, env, fs};

use git_config::file::from_paths::Options;
use git_config::file::{from_env, from_paths};
use git_config::File;
use serial_test::serial;
use tempfile::tempdir;

use crate::file::init::from_paths::escape_backslashes;

pub struct Env<'a> {
    altered_vars: Vec<&'a str>,
}

impl<'a> Env<'a> {
    pub(crate) fn new() -> Self {
        Env {
            altered_vars: Vec::new(),
        }
    }

    pub(crate) fn set(mut self, var: &'a str, value: impl Into<String>) -> Self {
        env::set_var(var, value.into());
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
    let config = File::from_env(Options::default()).unwrap();
    assert!(config.is_none());
}

#[test]
#[serial]
fn empty_with_zero_count() {
    let _env = Env::new().set("GIT_CONFIG_COUNT", "0");
    let config = File::from_env(Options::default()).unwrap();
    assert!(config.is_none());
}

#[test]
#[serial]
fn parse_error_with_invalid_count() {
    let _env = Env::new().set("GIT_CONFIG_COUNT", "invalid");
    let err = File::from_env(Options::default()).unwrap_err();
    assert!(matches!(err, from_env::Error::InvalidConfigCount { .. }));
}

#[test]
#[serial]
fn single_key_value_pair() {
    let _env = Env::new()
        .set("GIT_CONFIG_COUNT", "1")
        .set("GIT_CONFIG_KEY_0", "core.key")
        .set("GIT_CONFIG_VALUE_0", "value");

    let config = File::from_env(Options::default()).unwrap().unwrap();
    assert_eq!(
        config.raw_value("core", None, "key").unwrap(),
        Cow::<[u8]>::Borrowed(b"value")
    );

    assert_eq!(config.num_values(), 1);
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

    let config = File::from_env(Options::default()).unwrap().unwrap();

    assert_eq!(
        config.raw_value("core", None, "a").unwrap(),
        Cow::<[u8]>::Borrowed(b"a")
    );
    assert_eq!(
        config.raw_value("core", None, "b").unwrap(),
        Cow::<[u8]>::Borrowed(b"b")
    );
    assert_eq!(
        config.raw_value("core", None, "c").unwrap(),
        Cow::<[u8]>::Borrowed(b"c")
    );
    assert_eq!(config.num_values(), 3);
}

#[test]
#[serial]
fn error_on_relative_paths_in_include_paths() {
    let _env = Env::new()
        .set("GIT_CONFIG_COUNT", "1")
        .set("GIT_CONFIG_KEY_0", "include.path")
        .set("GIT_CONFIG_VALUE_0", "some_git_config");

    let res = File::from_env(Options::default());
    assert!(matches!(
        res,
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
        .set("GIT_CONFIG_VALUE_1", escape_backslashes(a_path))
        .set("GIT_CONFIG_KEY_2", "other.path")
        .set("GIT_CONFIG_VALUE_2", escape_backslashes(&b_path))
        .set("GIT_CONFIG_KEY_3", "include.origin.path")
        .set("GIT_CONFIG_VALUE_3", escape_backslashes(b_path));

    let config = File::from_env(Options::default()).unwrap().unwrap();

    assert_eq!(
        config.raw_value("core", None, "key").unwrap(),
        Cow::<[u8]>::Borrowed(b"changed")
    );
    assert_eq!(config.num_values(), 5);
}
